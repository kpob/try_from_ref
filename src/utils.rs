use proc_macro_error::abort;


/// Parses the given attribute or aborts the program if parsing fails.
/// 
/// # Arguments
/// 
/// * `attr` - The attribute to parse.
/// 
/// # Generic Parameters
/// 
/// * `T` - The type to parse the attribute into. Must implement `syn::parse::Parse`.
/// 
/// # Returns
/// 
/// The parsed value of type `T`.
pub fn parse_attr_or_abort<T: syn::parse::Parse>(attr: &syn::Attribute) -> T {
    let ident = attr.path().get_ident().map(|i| i.to_string()).unwrap_or_default();
    match attr.parse_args::<T>() {
        Ok(res) => res,
        Err(err) => abort!(
            attr,
            "Could not parse {:?} attribute", ident;
            help = err;
        ),
    }
}
/// Aborts the program if the given attribute is used in the wrong context.
///
/// # Arguments
///
/// * `attr` - The attribute that is used in the wrong context.
///
/// # Panics
///
/// This function panics if the attribute does not exist or 
/// should not be used in this context (eg #[source] attribute in the context of a field).
pub fn invalid_attr_context(attr: &syn::Attribute) {
    let ident = attr.path().get_ident().map(|i| i.to_string()).unwrap_or_default();
    abort!(
        attr,
        "Attribute {} used in the wrong context", ident;
        note = "Only source and err attributes are allowed here";
    )
}
