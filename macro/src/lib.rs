#[macro_use]
mod error;
mod derive;
mod utils;

use derive::Params;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn enum_newtype(attr: TokenStream, item: TokenStream) -> TokenStream {
    derive::enum_newtype(
        parse_macro_input!(attr as Params),
        parse_macro_input!(item as DeriveInput),
    )
    .into()
}
