use crate::{ir::IR, item::Item};
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::ToTokens;

mod attrs;
mod ir;
mod item;
mod utils;

#[proc_macro_derive(TryFromRef, attributes(source, default, expr, err))]
#[proc_macro_error]
pub fn derive_try_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let ir = IR::new(input);
    ir.validate();
    Item::from(ir).into_token_stream().into()
}
