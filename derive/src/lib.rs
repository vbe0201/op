#[macro_use]
extern crate quote;

mod ast;
mod attrs;
mod codegen;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Type, attributes(op_crate, property))]
pub fn derive_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    codegen::derive(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
