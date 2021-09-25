use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Data, DeriveInput, Path, Result, Visibility};

use crate::ast;

pub fn impl_type(input: &ast::Struct<'_>, path: &Path) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let type_trait = spanned_type_trait(input.original, path);

    Ok(quote! {
        // SAFETY: `any` and `any_mut` both return `self`.
        unsafe impl #impl_generics #type_trait for #ty #ty_generics #where_clause {
            #[inline]
            fn any(&self) -> &dyn ::std::any::Any {
                self
            }

            #[inline]
            fn any_mut(&mut self) -> &mut dyn ::std::any::Any {
                self
            }

            fn type_ref(&self) -> #path::TypeRef<'_> {
                #path::TypeRef::Class(self)
            }

            fn type_mut(&mut self) -> #path::TypeMut<'_> {
                #path::TypeMut::Class(self)
            }

            fn clone_value(&self) -> ::std::boxed::Box<dyn #path::Type> {
                todo!()
            }

            #[inline]
            fn try_set(
                &mut self,
                value: ::std::boxed::Box<dyn #path::Type>
            ) -> ::std::result::Result<(), ::std::boxed::Box<dyn #path::Type>> {
                *self = *value.downcast()?;
                Ok(())
            }
        }
    })
}

fn spanned_type_trait(input: &DeriveInput, path: &Path) -> TokenStream {
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
    let ty = quote_spanned!(last_span=> Type);

    quote!(#path #ty)
}
