extern crate proc_macro;

use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, parse_quote, ItemEnum};

use proc_macro::TokenStream;
use proc_macro2;

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
        var.attrs.retain(|attr| !tagged_filter(attr));
        let field_ident = var.ident.to_string();
        let ser_func = format_ident!("mixed_tags_ser_{}", field_ident);
        let ser_func_str = format!("mixed_tags_ser_{}", field_ident);
        let de_func = format_ident!("mixed_tags_de_{}", field_ident);
        let de_func_str = format!("mixed_tags_de_{}", field_ident);
        let unnamed = match var.fields {
            syn::Fields::Unnamed(ref u) => u,
            _ => {
                result.extend(
                    [proc_macro2::TokenStream::from(quote! {
                        compile_error!("Need NewType(Type) variant");
                    })]
                    .iter()
                    .cloned(),
                );
                continue;
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
                fn #ser_func<S>(val: &#inner_type, serializer: S)
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
                fn #de_func<'de, D>(deserializer: D) -> Result<#inner_type, D::Error>
                where
                    D: serde::de::Deserializer<'de>,
                {
                    use tagged_visitor::TaggedVisitor;
                    use serde::de::Visitor;

                    deserializer.deserialize_struct(
                        #enum_ident,
                        std::slice::from_ref(&#field_ident),
                        TaggedVisitor::new(#field_ident)
                    )
                }
            }]
            .iter()
            .cloned(),
        );
        var.attrs.push(parse_quote! {
            #[serde(serialize_with = #ser_func_str)]
        });
        var.attrs.push(parse_quote! {
            #[serde(deserialize_with = #de_func_str)]
        });
    }

    let mut stream = item.into_token_stream();
    stream.extend([result].iter().cloned());
    // eprintln!("{}", stream);
    stream.into()
}
