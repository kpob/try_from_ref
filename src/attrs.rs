use crate::utils::{invalid_attr_context, parse_attr_or_abort};
use proc_macro_error::abort;
use quote::ToTokens;

const ARG_SOURCE: &str = "source";
const ARG_ERR: &str = "err";
const ARG_DEFAULT: &str = "default";
const ARG_EXPR: &str = "expr";

/// Represents the attributes of a struct.
///
/// This struct holds information about the attributes of a struct, including the source type and the error type.
pub struct StructAttrs {
    source: syn::Type,
    err: syn::Type,
}

impl StructAttrs {
    /// Parses the attributes and returns an instance of `StructAttrs`.
    ///
    /// If an unknown attribute is found or the source or error is missing,
    /// this function aborts the compilation.
    ///
    /// # Arguments
    ///
    /// * `context` - The context used for error reporting.
    /// * `attrs` - The attributes to parse.
    ///
    /// # Returns
    ///
    /// An instance of `StructAttrs` with the parsed attributes.
    pub fn parse<T: ToTokens>(context: &T, attrs: &[syn::Attribute]) -> Self {
        let mut source = None;
        let mut err = None;
        for attr in attrs {
            let path = attr.path();
            if path.is_ident(ARG_SOURCE) {
                source = Some(parse_attr_or_abort::<syn::Type>(attr));
            }
            if path.is_ident(ARG_ERR) {
                err = Some(parse_attr_or_abort::<syn::Type>(attr));
            }
            if path.is_ident(ARG_DEFAULT) || path.is_ident(ARG_EXPR) {
                invalid_attr_context(attr)
            }
        }

        let source = source.unwrap_or_else(|| {
            abort! {
                context,
                "Missing source attribute";
                help = "Add #[source(Type)] attribute";
            }
        });
        let err = err.unwrap_or_else(|| {
            abort! {
                context,
                "Missing err attribute";
                help = "Add #[err(Type)] attribute";
            }
        });

        Self { source, err }
    }

    /// Returns a reference to the `source` identifier.
    pub fn source(&self) -> &syn::Type {
        &self.source
    }

    /// Returns a reference to the error type.
    pub fn error(&self) -> &syn::Type {
        &self.err
    }
}

/// Represents the attributes of a field.
///
/// This struct holds information about the attributes of a field.
/// The `default` attribute is used to indicate that the field should initialized with the default value.
/// The `expr` attribute is used to indicate that the field should initialized with the given expression.
pub struct FieldAttrs {
    default: bool,
    expr: Option<syn::Expr>,
}

impl FieldAttrs {
    /// Parses the attributes and returns an instance of `FieldAttrs`.
    ///
    /// If an unknown attribute is found, this function aborts the compilation.
    ///
    /// # Arguments
    ///
    /// * `context` - The context used for error reporting.
    /// * `attrs` - The attributes to parse.
    ///
    /// # Returns
    ///
    /// An instance of `Self` with the parsed attributes.
    pub fn parse<T: ToTokens>(context: &T, attrs: &[syn::Attribute]) -> Self {
        let mut default = false;
        let mut expr = None;
        for attr in attrs {
            let path = attr.path();
            if path.is_ident(ARG_DEFAULT) {
                default = true;
            }
            if path.is_ident(ARG_EXPR) {
                expr = Some(parse_attr_or_abort::<syn::Expr>(attr));
            }
            if path.is_ident(ARG_SOURCE) || path.is_ident(ARG_ERR) {
                invalid_attr_context(attr)
            }
        }

        if default && expr.is_some() {
            abort! {
                context,
                "Cannot use default and expr attributes together";
                help = "Remove one of them"
            }
        }

        Self { default, expr }
    }

    /// Returns `true` if the `default` attribute is present, `false` otherwise.
    pub fn default(&self) -> bool {
        self.default
    }

    /// Returns a reference to the `expr` attribute.
    pub fn expr(&self) -> Option<&syn::Expr> {
        self.expr.as_ref()
    }
}
