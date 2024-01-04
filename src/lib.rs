use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields};

#[proc_macro_derive(Inherit)]
pub fn derive_inherit(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let class = input.ident;
    if let Data::Struct(s) = input.data {
        if let Fields::Named(n) = s.fields {
            if let Some(f) = n.named.into_iter().next() {
                if let Some(i) = f.ident {
                    let t = f.ty;
                    return quote! {
                        impl Deref for #class {
                            type Target = #t;
                            fn deref(&self) -> &Self::Target {
                                &self.#i
                            }
                        }
                    }.into()
                }
            }
        }
    }
    quote!().into()
}