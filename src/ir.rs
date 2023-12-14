use crate::attrs::{FieldAttrs, StructAttrs};
use proc_macro2::Ident;
use proc_macro_error::abort;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Data, DataStruct, DeriveInput, Field, FieldValue, Token};

pub struct IR {
    input: DeriveInput,
}

impl IR {
    pub fn new(input: DeriveInput) -> Self {
        Self { input }
    }

    fn data(&self) -> DataStruct {
        match &self.input.data {
            Data::Struct(data) => data.clone(),
            _ => abort! {
                self.input,
                "Unions and Enums are not supported!"
            },
        }
    }
    pub fn ident(&self) -> Ident {
        self.input.ident.clone()
    }

    pub fn validate(&self) {
        if self.is_generic() {
            abort! {
                self.input,
                "Generic structs are not supported!"
            }
        }
    }

    pub fn is_generic(&self) -> bool {
        !self.input.generics.params.is_empty()
    }

    pub fn struct_attrs(&self) -> StructAttrs {
        StructAttrs::parse(&self.input.ident, &self.input.attrs)
    }

    pub fn source(&self) -> Ident {
        self.struct_attrs().source().clone()
    }

    pub fn error(&self) -> syn::Type {
        self.struct_attrs().error().clone()
    }

    pub fn source_ident(&self) -> Ident {
        source_ident()
    }

    pub fn fields(&self) -> Punctuated<FieldValue, Token![,]> {
        self.data().fields.iter().map(to_field_definition).collect()
    }
}

fn to_field_definition(field: &Field) -> FieldValue {
    let attrs = FieldAttrs::parse(field, &field.attrs);
    let ident = &field.ident;
    if attrs.default() {
        parse_quote!(#ident: Default::default())
    } else if let Some(expr) = attrs.expr() {
        parse_quote!(#ident: #expr)
    } else {
        let input = source_ident();
        parse_quote!(#ident: #input.try_into()?)
    }
}

#[inline]
fn source_ident() -> Ident {
    format_ident!("input")
}
