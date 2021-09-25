mod r#struct;
mod r#type;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Error, Path, Result};

use crate::ast;

pub fn derive(input: &DeriveInput) -> Result<TokenStream> {
    let input = match &input.data {
        Data::Struct(data) => ast::Struct::from_syn(input, data)?,
        Data::Enum(_) | Data::Union(_) => {
            return Err(Error::new_spanned(
                input,
                "enums and unions are not supported",
            ))
        }
    };
    input.validate()?;
    let path = get_path(&input);

    r#struct::impl_class(&input, &path)
}

fn get_path(input: &ast::Struct<'_>) -> Path {
    const KRATE: &str = "::op";

    input.attrs.op_crate.as_ref().map_or_else(
        || syn::parse(KRATE.parse::<TokenStream1>().unwrap()).unwrap(),
        |attr| attr.krate.clone(),
    )
}
