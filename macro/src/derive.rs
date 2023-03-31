use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote_spanned};
use syn::{parse_quote, punctuated::Punctuated, spanned::Spanned};

use crate::utils::AttributeExt;

/// The parameter of the [`enum_newtype`] proc. macro.
pub struct Params {
    params: Punctuated<syn::Meta, syn::Token![,]>,
}

impl Params {
    /// Returns the identifier of the `name` parameter.
    fn name(&self) -> syn::Result<syn::Ident> {
        self.params
            .iter()
            .filter_map(|meta| {
                if let syn::Meta::NameValue(name_value) = meta {
                    return Some(&name_value.value);
                }
                None
            })
            .filter_map(|value| {
                if let syn::Expr::Path(expr_path) = value {
                    return expr_path.path.get_ident();
                }
                None
            })
            .next()
            .cloned()
            .ok_or_else(|| format_err!(self.params, "cannot find valid `name` parameter"))
    }
}

impl syn::parse::Parse for Params {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            params: input.parse_terminated(syn::Meta::parse, syn::Token![,])?,
        })
    }
}

pub fn enum_newtype(params: Params, input: syn::DeriveInput) -> TokenStream2 {
    match enum_newtype_impl(params, input) {
        Ok(result) => result,
        Err(error) => error.to_compile_error(),
    }
}

fn enum_newtype_impl(params: Params, input: syn::DeriveInput) -> syn::Result<TokenStream2> {
    let span = input.span();
    let attrs = &input.attrs;
    let derives = attrs
        .iter()
        .cloned()
        .filter(syn::Attribute::is_derive_attribute)
        .collect::<Vec<_>>();
    let vis = &input.vis;
    let generics = &input.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let ident = &input.ident;
    let data = enum_data(&input)?;
    let trait_ident = params.name()?;
    let newtype_variants = data
        .variants
        .iter()
        .map(|variant| variant_to_newtype(variant, &trait_ident));
    let newtype_aliases = data.variants.iter().map(variant_to_assoc_type);
    let newtype_alias_impls = data.variants.iter().map(variant_to_assoc_type_impl);
    let variant_structs = data
        .variants
        .iter()
        .map(|variant| variant_to_struct(variant, generics, &derives));
    let variant_idents = data.variants.iter().map(|variant| &variant.ident);
    let variant_struct_idents = variant_idents.clone().map(variant_struct_ident);
    Ok(quote_spanned!(span=>
        #( #attrs )*
        #vis enum #ident #generics {
            #( #newtype_variants ),*
        }

        pub trait #trait_ident #generics {
            #( #newtype_aliases )*
        }

        const _: () = {
            #( #variant_structs )*

            #(
                impl #impl_generics ::core::convert::From<#variant_struct_idents> for #ident #type_generics #where_clause {
                    fn from(variant: #variant_struct_idents) -> Self {
                        Self::#variant_idents(variant)
                    }
                }
            )*

            impl #impl_generics #trait_ident for #ident #type_generics #where_clause {
                #( #newtype_alias_impls )*
            }
        };
    ))
}

/// Returns the [`syn::DataEnum`] of the [`syn::Data`] or a [`syn::Error`].
fn enum_data(input: &syn::DeriveInput) -> syn::Result<syn::DataEnum> {
    match &input.data {
        syn::Data::Enum(data) => Ok(data.clone()),
        syn::Data::Struct(_) => Err(format_err!(
            input,
            "cannot use `#enum_newtype` on `struct` types"
        )),
        syn::Data::Union(_) => Err(format_err!(
            input,
            "cannot use `#enum_newtype` on `union` types"
        )),
    }
}

/// Creates a newtype [`syn::Variant`] referring to the `trait_ident` associated type.
fn variant_to_newtype(variant: &syn::Variant, trait_ident: &syn::Ident) -> syn::Variant {
    let ident = &variant.ident;
    syn::Variant {
        fields: syn::Fields::Unnamed(syn::parse_quote!((<Self as #trait_ident>::#ident))),
        ..variant.clone()
    }
}

/// Creates the associated type of the [`syn::Variant`] for the generated `trait_ident` trait.
fn variant_to_assoc_type(variant: &syn::Variant) -> syn::TraitItemType {
    let ident = &variant.ident;
    parse_quote!(type #ident;)
}

/// Creates the associated type of the [`syn::Variant`] for the generated `trait_ident` trait.
fn variant_to_assoc_type_impl(variant: &syn::Variant) -> syn::TraitItemType {
    let ident = &variant.ident;
    let variant_struct_ident = variant_struct_ident(ident);
    parse_quote!(type #ident = #variant_struct_ident;)
}

/// Creates a [`syn::ItemStruct`] from the given [`syn::Variant`] and [`syn::Generics`].
fn variant_to_struct(
    variant: &syn::Variant,
    generics: &syn::Generics,
    derives: &[syn::Attribute],
) -> syn::ItemStruct {
    let semi_token = match &variant.fields {
        syn::Fields::Named(_) => None,
        syn::Fields::Unnamed(_) | syn::Fields::Unit => Some(<syn::Token![;]>::default()),
    };
    let mut attrs = variant.attrs.clone();
    attrs.extend_from_slice(derives);
    syn::ItemStruct {
        attrs,
        vis: syn::Visibility::Public(<syn::Token![pub]>::default()),
        struct_token: <syn::Token![struct]>::default(),
        ident: variant_struct_ident(&variant.ident),
        generics: generics.clone(),
        fields: variant.fields.clone(),
        semi_token,
    }
}

/// Returns a prefixed identifier for the given variant struct identifier.
fn variant_struct_ident(ident: &syn::Ident) -> syn::Ident {
    format_ident!("__enum_newtype_{ident}")
}
