use proc_macro_error::abort;

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

pub fn invalid_attr_context(attr: &syn::Attribute) {
    let ident = attr.path().get_ident().map(|i| i.to_string()).unwrap_or_default();
    abort!(
        attr,
        "Attribute {} used in the wrong context", ident;
        note = "Only source and err attributes are allowed here";
    )
}
