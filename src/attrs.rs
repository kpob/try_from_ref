use crate::utils::{invalid_attr_context, parse_attr_or_abort};
use proc_macro_error::abort;
use quote::ToTokens;

const ARG_SOURCE: &str = "source";
const ARG_ERR: &str = "err";
const ARG_DEFAULT: &str = "default";
const ARG_EXPR: &str = "expr";

pub struct StructAttrs {
    source: syn::Ident,
    err: syn::Type,
}
impl StructAttrs {
    pub fn parse<T: ToTokens>(context: &T, attrs: &[syn::Attribute]) -> Self {
        let mut source = None;
        let mut err = None;
        for attr in attrs {
            let path = attr.path();
            if path.is_ident(ARG_SOURCE) {
                source = Some(parse_attr_or_abort::<syn::Ident>(attr));
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
    pub fn source(&self) -> &syn::Ident {
        &self.source
    }
    pub fn error(&self) -> &syn::Type {
        &self.err
    }
}

pub struct FieldAttrs {
    default: bool,
    expr: Option<syn::Expr>,
}

impl FieldAttrs {
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
    pub fn default(&self) -> bool {
        self.default
    }
    pub fn expr(&self) -> Option<&syn::Expr> {
        self.expr.as_ref()
    }
}
