use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Error, Result};

pub fn expand(attr: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let mut input: syn::DeriveInput = syn::parse2(input)?;

    if !attr.is_empty() {
        return Err(Error::new_spanned(
            attr,
            "unexpected arguments to the c_enum macro",
        ));
    }

    if input.generics.gt_token.is_some() {
        return Err(Error::new_spanned(
            input.generics,
            "generic enums are not yet supported",
        ));
    }

    let data = match &input.data {
        syn::Data::Enum(data) => data,
        syn::Data::Struct(data) => {
            return Err(Error::new_spanned(
                data.struct_token,
                "c_enum only supports enums",
            ))
        }
        syn::Data::Union(data) => {
            return Err(Error::new_spanned(
                data.union_token,
                "c_enum only supports enums",
            ))
        }
    };

    let repr: syn::Type = match input
        .attrs
        .iter()
        .position(|attr| attr.path().is_ident("repr"))
    {
        Some(index) => {
            let attr = input.attrs.remove(index);

            match attr.meta {
                syn::Meta::List(list) => syn::parse2(list.tokens)?,
                _ => {
                    return Err(Error::new_spanned(
                        attr,
                        "could not parse #[repr] annotation",
                    ))
                }
            }
        }
        None => syn::parse_quote!(isize),
    };

    let mut fields = Vec::new();
    let mut variants = Vec::new();
    for (i, variant) in data.variants.iter().enumerate() {
        let value = match &variant.discriminant {
            Some((_, expr)) => expr.clone(),
            None if i == 0 => syn::parse_quote_spanned!(variant.ident.span() => 0),
            None => {
                let prev = &data.variants[i - 1].ident;
                syn::parse_quote!(Self::#prev.0 + 1)
            }
        };

        let ident = &variant.ident;
        let attrs = &variant.attrs;

        variants.push(quote::quote_spanned!(variant.span() =>
            #( #attrs )*
            pub const #ident: Self = Self(#value);
        ));
        fields.push(ident.clone());
    }

    let ident = &input.ident;
    let attrs = &input.attrs;
    let vis = &input.vis;

    let new_doc = format!(
        " Create a new `{ident}` from a `{}`.",
        repr.to_token_stream()
    );
    let new_doc = syn::LitStr::new(&new_doc, Span::call_site());

    let field_names: Vec<_> = fields
        .iter()
        .map(|field| syn::LitStr::new(&format!("{ident}::{field}"), Span::call_site()))
        .collect();
    let ident_name = syn::LitStr::new(&ident.to_string(), Span::call_site());

    Ok(quote::quote! {
        #( #attrs )*
        #vis struct #ident(#repr);

        #[allow(non_upper_case_globals)]
        impl #ident {
            #( #variants )*
        }

        impl #ident {
            #[doc = #new_doc]
            pub const fn new(value: #repr) -> Self {
                Self(value)
            }
        }

        impl ::core::convert::From<#repr> for #ident {
            fn from(value: #repr) -> Self {
                Self::new(value)
            }
        }

        impl ::core::convert::From<#ident> for #repr {
            fn from(value: #ident) -> Self {
                value.0
            }
        }

        impl ::core::fmt::Debug for #ident 
        where
            #repr: ::core::fmt::Debug
        {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>
            ) -> ::core::fmt::Result {
                match self {
                    #( Self(value) if Self::#fields.0 == *value => f.write_str(#field_names), )*
                    Self(value) => f
                        .debug_tuple(#ident_name)
                        .field(value)
                        .finish()
                }
            }
        }
    })
}
