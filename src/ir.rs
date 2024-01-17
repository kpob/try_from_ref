use crate::attrs::{FieldAttrs, StructAttrs};
use proc_macro2::Ident;
use proc_macro_error::abort;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Data, DataStruct, DeriveInput, Field, FieldValue, Token};

/// Represents the intermediate representation (IR) of the struct.
/// 
/// Target struct is the struct that the `TryFrom` trait is implemented for.
/// Source type is the type generic type of the `TryFrom` trait.
pub struct IR {
    input: DeriveInput,
}

impl IR {
    /// Creates a new instance of `Ir` with the given `input`.
    pub fn new(input: DeriveInput) -> Self {
        Self { input }
    }

    /// Returns the identifier of the target struct.
    pub fn target_ident(&self) -> Ident {
        self.input.ident.clone()
    }

    /// Validates the target struct.
    ///
    /// # Panics
    ///
    /// Panics if the target struct is generic.
    pub fn validate(&self) {
        if self.is_generic() {
            abort! {
                self.input,
                "Generic structs are not supported!"
            }
        }
    }

    /// Checks if the IR is generic.
    ///
    /// Returns `true` if the IR has generic parameters, `false` otherwise.
    pub fn is_generic(&self) -> bool {
        !self.input.generics.params.is_empty()
    }

    /// Returns the attributes of the struct.
    ///
    /// This function parses the target identifier as the context of a potential error
    /// and attributes of the struct and returns a [StructAttrs] object.
    pub fn struct_attrs(&self) -> StructAttrs {
        StructAttrs::parse(&self.input.ident, &self.input.attrs)
    }

    /// Returns the source type associated with the IR.
    /// Source type is the type that implements `TryFrom` trait.
    pub fn source(&self) -> syn::Type {
        self.struct_attrs().source().clone()
    }

    /// Returns the error type associated with the IR.
    pub fn error(&self) -> syn::Type {
        self.struct_attrs().error().clone()
    }

    /// Returns the identifier of the variable of the source type.
    pub fn source_ident(&self) -> Ident {
        source_ident()
    }

    /// Returns the fields of the `IR` struct.
    ///
    /// This function iterates over the fields of the `IR` struct and converts them into `FieldValue` objects.
    /// The resulting `FieldValue` objects are collected into a `Punctuated` list separated by commas.
    pub fn fields(&self) -> Punctuated<FieldValue, Token![,]> {
        self.data().fields.iter().map(to_field_definition).collect()
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
