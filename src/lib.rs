#![allow(unused_imports)]

extern crate proc_macro;

use syn::{
    DeriveInput, Expr,
    ItemEnum, parse_macro_input,
    punctuated::Punctuated, Token
};
use quote::{quote, ToTokens};

use proc_macro::TokenStream;
use proc_macro2;

#[proc_macro_attribute]
pub fn mixed_tags(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemEnum);

    let filter = |attr: &syn::Attribute|
        /*attr.style == syn::AttrStyle::Outer
        &&*/
        attr.path.segments.len() == 1
        && attr.path.segments
                .first().unwrap()
                .ident.to_string()
            == "tagged";
    let _variants = item.variants.iter_mut()
        .filter_map(|var|
            if !var.attrs.iter()
                .any(filter)
            {
                None
            } else {
                var.attrs.retain(|attr| !filter(attr));
                Some(var)
            }
        )
        .collect::<Vec<_>>();

    item.into_token_stream().into()
}
