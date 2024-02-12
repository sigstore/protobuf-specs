use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Serialize_proto)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    match input.data {
        syn::Data::Struct(_) => (),
        _ => return Default::default(),
    };

    let expanded = quote! {
        impl serde::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let message = prost_reflect::ReflectMessage::transcode_to_dynamic(self);
                serde::Serialize::serialize(&message, serializer)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Deserialize_proto)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    match input.data {
        syn::Data::Struct(_) => (),
        _ => return Default::default(),
    };

    let expanded = quote! {
        impl<'de> serde::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> Result<#ident, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let concrete_msg: #ident = Default::default();
                let descriptor = prost_reflect::ReflectMessage::descriptor(&concrete_msg);
                let dynamic_msg = prost_reflect::DynamicMessage::deserialize(descriptor, deserializer)?;

                Ok(dynamic_msg.transcode_to().expect("failed to convert DynamicMessage to concrete Message!"))
            }
        }
    };

    TokenStream::from(expanded)
}
