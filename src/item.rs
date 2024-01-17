use crate::ir::IR;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::{FieldValue, Token, Type};

/// Represents an item to be tokenized.
///
/// In the result of tokenization, an implementation of `TryFrom` trait is generated.
pub struct Item {
    source: Type,
    target: Ident,
    error_ty: Type,
    input: Ident,
    fields: Punctuated<FieldValue, Token![,]>,
}

impl From<IR> for Item {
    fn from(value: IR) -> Self {
        Self {
            source: value.source(),
            target: value.target_ident(),
            error_ty: value.error(),
            input: value.source_ident(),
            fields: value.fields(),
        }
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let source = &self.source;
        let target = &self.target;
        let error = &self.error_ty;
        let input = &self.input;
        let fields = &self.fields;

        tokens.append_all(quote::quote!(
            #[automatically_derived]
            impl ::core::convert::TryFrom<&'_ #source> for #target {
                type Error = #error;

                fn try_from(#input: &'_ #source) -> ::core::result::Result<Self, Self::Error> {
                    Ok(Self { #fields })
                }
            }
        ));
    }
}
