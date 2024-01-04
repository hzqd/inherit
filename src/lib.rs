use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields, DataStruct, Field, FieldsNamed};

#[proc_macro_derive(Inherit)]
pub fn derive_inherit(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let class = input.ident;
    if let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), .. }) = input.data {
        if let Some(Field { ident: Some(id), ty, .. }) = named.into_iter().next() {
            return quote! {
                impl Deref for #class {
                    type Target = #ty;
                    fn deref(&self) -> &Self::Target {
                        &self.#id
                    }
                }
            }.into()
        }
    }
    panic!("Inherit only support struct with named field, which must be first.")
}
