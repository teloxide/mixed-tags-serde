#![allow(unused_imports)]
#![allow(dead_code)]
#![feature(log_syntax)]

extern crate proc_macro;

use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, DeriveInput, Expr, ItemEnum, Token,
};

use proc_macro::TokenStream;
use proc_macro2;

enum Target {
    Serialize,
    Deserialize,
    Both,
}

#[proc_macro_attribute]
pub fn mixed_tags(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemEnum);
    let mut result = proc_macro2::TokenStream::new();
    let enum_ident = item.ident.to_string();

    let tagged_filter = |attr: &syn::Attribute|
        /*attr.style == syn::AttrStyle::Outer &&*/
        attr.path.segments.len() == 1
        && attr.path.segments
                .first().unwrap()
                .ident.to_string()
            == "tagged";
    for var in item.variants.iter_mut() {
        if !var.attrs.iter().any(tagged_filter) {
            continue;
        }
        var.attrs.retain(|attr| {
            attr.path.segments.len() != 1
                || attr.path.segments.first().unwrap().ident.to_string() != "serde"
        });
        let field_ident = var.ident.to_string();
        let func_name = format_ident!("mixed_tags_se_{}", field_ident);
        let func_name_str = format!("mixed_tags_se_{}", field_ident);
        let unnamed = match var.fields {
            syn::Fields::Unnamed(ref u) => u,
            _ => {
                return quote! {
                    compile_error!("Need NewType(Type) variant")
                }
                .into()
            }
        };
        let first_field = match unnamed.unnamed.first() {
            None => {
                return quote! {
                    compile_error!("Need NewType(Type) variant")
                }
                .into()
            }
            Some(field) => field,
        };
        let inner_type = &first_field.ty;
        result.extend(
            [quote! {
                fn #func_name<S>(val: &#inner_type, serializer: S)
                    -> Result<S::Ok, S::Error>
                    where
                        S: serde::ser::Serializer
                {
                    use serde::ser::SerializeStruct;
                    let mut sv
                        = serializer.serialize_struct(#enum_ident, 1)?;
                    sv.serialize_field(#field_ident, val)?;
                    sv.end()
                }
            }]
            .iter()
            .cloned(),
        );
        var.attrs.retain(|attr| !tagged_filter(attr));
        var.attrs
            .push(parse_quote! { #[serde(serialize_with = #func_name_str)] });
    }

    let mut stream = item.into_token_stream();
    stream.extend([result].iter().cloned());
    // eprintln!("{}", stream);
    stream.into()
}

mod visitor;
use serde::{
    de::Deserializer,
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::slice;
use visitor::TaggedVisitor;

fn tagged_de<'de, T, D>(des: D, ty: &'static str, tag: &'static &'static str) -> Result<T, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    des.deserialize_struct(ty, slice::from_ref(&tag), TaggedVisitor::new(tag))
}

fn tagged_ser<T, S>(val: &T, ser: S, ty: &'static str, tag: &'static str) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let mut sv = ser.serialize_struct(ty, 1)?;
    sv.serialize_field(tag, val)?;
    sv.end()
}
