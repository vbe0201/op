use proc_macro2::Ident;
use syn::{DataStruct, DeriveInput, Error, Generics, Member, Result, Type};

use crate::attrs::{self, Attrs};

pub struct Struct<'a> {
    pub original: &'a DeriveInput,
    pub attrs: Attrs<'a>,
    pub ident: Ident,
    pub generics: &'a Generics,
    pub fields: Vec<Field<'a>>,
}

impl<'a> Struct<'a> {
    pub fn from_syn(input: &'a DeriveInput, data: &'a DataStruct) -> Result<Self> {
        let attrs = attrs::get(&input.attrs)?;
        let fields = Field::multiple_from_syn(&data.fields)?;

        Ok(Struct {
            original: input,
            attrs,
            ident: input.ident.clone(),
            generics: &input.generics,
            fields,
        })
    }

    pub fn validate(&self) -> Result<()> {
        check_outer_attrs(&self.attrs)?;
        for field in &self.fields {
            check_inner_attrs(&field.attrs)?;
        }
        Ok(())
    }
}

pub struct Field<'a> {
    pub original: &'a syn::Field,
    pub attrs: Attrs<'a>,
    pub member: Ident,
    pub ty: &'a Type,
}

impl<'a> Field<'a> {
    fn from_syn(node: &'a syn::Field) -> Result<Self> {
        Ok(Field {
            original: node,
            attrs: attrs::get(&node.attrs)?,
            member: if let Member::Named(member) = node
                .ident
                .clone()
                .map(Member::Named)
                .ok_or_else(|| Error::new_spanned(node, "tuple structs are unsupported"))?
            {
                member
            } else {
                unreachable!()
            },
            ty: &node.ty,
        })
    }

    fn multiple_from_syn(fields: &'a syn::Fields) -> Result<Vec<Self>> {
        fields.iter().map(|f| Field::from_syn(f)).collect()
    }

    pub fn name(&self) -> String {
        self.member.to_string().trim_start_matches("r#").to_owned()
    }

    #[allow(unused)]
    pub fn flags(&self) -> &[Ident] {
        self.attrs
            .property
            .as_ref()
            .map_or(&[], |attr| attr.flags.as_deref().unwrap_or(&[]))
    }
}

fn check_outer_attrs(attrs: &Attrs<'_>) -> Result<()> {
    if let Some(ref property) = attrs.property {
        return Err(Error::new_spanned(
            property.original,
            "unexpected #[property] attribute belongs on a specific struct field",
        ));
    }
    Ok(())
}

fn check_inner_attrs(attrs: &Attrs<'_>) -> Result<()> {
    if let Some(ref krate) = attrs.op_crate {
        return Err(Error::new_spanned(
            krate.original,
            "unexpected #[op_crate] attribute belongs on a struct",
        ));
    }
    Ok(())
}
