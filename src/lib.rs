#![allow(unused_imports)]

extern crate proc_macro;

use syn::{DeriveInput, Item, parse_macro_input};
use quote::{quote, ToTokens};

use proc_macro::TokenStream;
use proc_macro2;

#[proc_macro_attribute]
pub fn mixed_tags(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = Item::from(parse_macro_input!(item as DeriveInput));
    let target = if let Item::Enum(target) = input {
        target
    } else {
        return TokenStream::from(quote! {
            compile_error!("SerdeHack works only with enums");
        });
    };
    TokenStream::from(target.into_token_stream())
}
