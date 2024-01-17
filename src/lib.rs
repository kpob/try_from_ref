use crate::{ir::IR, item::Item};
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::ToTokens;

mod attrs;
mod ir;
mod item;
mod utils;

/// This function is a procedural macro that derives the `TryFromRef` trait for a given input.
/// The `TryFromRef` trait allows for converting a reference to a type into another type, with the possibility of failure.
/// It is used with the `#[derive(TryFromRef)]` attribute.
/// 
/// By the default, the generated implementation of `TryFromRef` trait will convert all fields of the source type into the target type.
/// However, this behavior can be changed by using the `#[default]` and `#[expr]` attributes.
/// #[default] sets the value of the field to the default value of the field type.
/// #[expr] sets the value of the field to the result of the given expression.
/// 
/// # Arguments
/// 
/// - `input`: A `TokenStream` representing the input for the procedural macro.
/// 
/// # Returns
/// 
/// The generated code as a `TokenStream`.
/// 
/// A `TokenStream` representing the generated code for the `TryFromRef` trait implementation.
/// 
/// # Example
/// 
/// ```
/// use derive_try_from::TryFromRef;
/// 
/// struct Source {
///    a: u32,
///    b: u32,
/// }
/// 
/// 
/// #[derive(TryFromRef)]
/// #[source(Source)]
/// #[err(&'static str)]
/// struct MyStruct {
///     // fields
/// }
/// ```
#[proc_macro_derive(TryFromRef, attributes(source, default, expr, err))]
#[proc_macro_error]
pub fn derive_try_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let ir = IR::new(input);
    ir.validate();
    Item::from(ir).into_token_stream().into()
}
