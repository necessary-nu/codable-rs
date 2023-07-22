//! This crate implements the macro for `codable` and should not be used directly.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Encode, attributes(codable))]
/// Document your macro here.
pub fn derive_encode(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as proc_macro2::TokenStream);

    match codable_macro_impl::derive_encode(item) {
        Ok(tokens) => tokens.into(),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

#[proc_macro_derive(Decode, attributes(codable))]
/// Document your macro here.
pub fn derive_decode(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as proc_macro2::TokenStream);

    match codable_macro_impl::derive_decode(item) {
        Ok(tokens) => tokens.into(),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
