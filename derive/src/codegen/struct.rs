use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Data, DeriveInput, Path, Result, Visibility};

use super::r#type::impl_type;
use crate::ast;

pub fn impl_class(input: &ast::Struct<'_>, path: &Path) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let idents: Vec<_> = input.fields.iter().map(|f| &f.member).collect();
    let names: Vec<_> = input.fields.iter().map(|f| f.name()).collect();

    let type_impl = impl_type(input, path)?;
    let pclass_trait = spanned_propertyclass_trait(input.original, path);

    Ok(quote! {
        #type_impl

        impl #impl_generics #pclass_trait for #ty #ty_generics #where_clause {
            fn property(
                &self,
                name: &::std::primitive::str
            ) -> ::std::option::Option<&dyn #path::Type> {
                match name {
                    #(#names => ::std::option::Option::Some(&self.#idents),)*
                    _ => ::std::option::Option::None,
                }
            }

            fn property_mut(
                &mut self,
                name: &::std::primitive::str
            ) -> ::std::option::Option<&mut dyn #path::Type> {
                match name {
                    #(#names => ::std::option::Option::Some(&mut self.#idents),)*
                    _ => ::std::option::Option::None,
                }
            }
        }
    })
}

fn spanned_propertyclass_trait(input: &DeriveInput, path: &Path) -> TokenStream {
    let vis_span = match &input.vis {
        Visibility::Public(vis) => Some(vis.pub_token.span()),
        Visibility::Crate(vis) => Some(vis.crate_token.span()),
        Visibility::Restricted(vis) => Some(vis.pub_token.span()),
        Visibility::Inherited => None,
    };
    let data_span = match &input.data {
        Data::Struct(data) => data.struct_token.span(),
        Data::Enum(data) => data.enum_token.span(),
        Data::Union(data) => data.union_token.span(),
    };
    let first_span = vis_span.unwrap_or(data_span);
    let last_span = input.ident.span();

    let path = quote_spanned!(first_span=> #path::);
    let ty = quote_spanned!(last_span=> PropertyClass);

    quote!(#path #ty)
}
