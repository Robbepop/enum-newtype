/// Extension methods for [`struct@syn::Attribute`].
pub trait AttributeExt {
    /// Returns `true` if the [`struct@syn::Attribute`] is a Rust documentation attribute.
    fn is_docs_attribute(&self) -> bool;

    /// Returns `Some` if the [`struct@syn::Attribute`] is a Rust doc attribute.
    fn filter_docs(&self) -> Option<&syn::Attribute>;

    /// Returns `true` if the [`struct@syn::Attribute`] is a Rust `derive` attribute.
    fn is_derive_attribute(&self) -> bool;

    /// Returns `Some` if the [`struct@syn::Attribute`] is a Rust `derive` attribute.
    fn filter_derive(&self) -> Option<&syn::Attribute>;
}

impl AttributeExt for syn::Attribute {
    fn is_docs_attribute(&self) -> bool {
        self.path().is_ident("doc")
    }

    fn filter_docs(&self) -> Option<&syn::Attribute> {
        if self.is_docs_attribute() {
            return Some(self);
        }
        None
    }

    fn is_derive_attribute(&self) -> bool {
        self.path().is_ident("derive")
    }

    fn filter_derive(&self) -> Option<&syn::Attribute> {
        if self.is_derive_attribute() {
            return Some(self);
        }
        None
    }
}
