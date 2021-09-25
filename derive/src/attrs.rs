use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Attribute, Error, Ident, Result, Token,
};

mod kw {
    syn::custom_keyword!(flags);
}

pub struct Attrs<'a> {
    pub op_crate: Option<CrateAttr<'a>>,
    pub property: Option<PropertyAttr<'a>>,
}

pub struct CrateAttr<'a> {
    pub original: &'a Attribute,
    pub krate: syn::Path,
}

pub struct PropertyAttr<'a> {
    pub original: &'a Attribute,
    pub flags: Option<Vec<Ident>>,
}

pub fn get(input: &[Attribute]) -> Result<Attrs<'_>> {
    let mut attrs = Attrs {
        op_crate: None,
        property: None,
    };

    for attr in input {
        if attr.path.is_ident("op_crate") {
            parse_crate_attr(&mut attrs, attr)?;
        } else if attr.path.is_ident("property") {
            parse_property_attr(&mut attrs, attr)?;
        }
    }

    Ok(attrs)
}

fn parse_crate_attr<'a>(attrs: &mut Attrs<'a>, attr: &'a Attribute) -> Result<()> {
    if attrs.op_crate.is_some() {
        return Err(Error::new_spanned(
            attr,
            "duplicate #[op_crate] attribute found",
        ));
    }

    attrs.op_crate = Some(CrateAttr {
        original: attr,
        krate: attr.parse_args_with(|input: ParseStream<'_>| {
            input.parse::<Ident>()?;
            if input.peek(token::Paren) {
                // #[op_crate(path)]
                let value;
                parenthesized!(value in input);
                value.call(syn::Path::parse_mod_style)
            } else {
                // #[op_crate = path]
                input.parse::<Token![=]>()?;
                input.call(syn::Path::parse_mod_style)
            }
        })?,
    });
    Ok(())
}

fn parse_property_attr<'a>(attrs: &mut Attrs<'a>, attr: &'a Attribute) -> Result<()> {
    if attrs.property.is_some() {
        return Err(Error::new_spanned(
            attr,
            "duplicate #[property] attribute found",
        ));
    }

    attrs.property = Some(PropertyAttr {
        original: attr,
        flags: None,
    });
    let property = attrs.property.as_mut().unwrap();

    attr.parse_args_with(|input: ParseStream<'_>| {
        while !input.is_empty() {
            let look = input.lookahead1();
            if look.peek(kw::flags) {
                parse_flags_arg(property, attr, input)?;
            }
        }
        Ok(())
    })
}

fn parse_flags_arg<'a>(
    attrs: &mut PropertyAttr<'a>,
    attr: &'a Attribute,
    input: ParseStream<'_>,
) -> Result<()> {
    if attrs.flags.is_some() {
        return Err(Error::new_spanned(
            attr,
            "duplicate #[property(flags)] attribute found",
        ));
    }

    input.parse::<kw::flags>()?;
    let content;
    parenthesized!(content in input);
    let bits: Punctuated<Ident, Token![|]> = content.parse_terminated(Ident::parse)?;

    attrs.flags = Some(bits.into_iter().collect());
    Ok(())
}
