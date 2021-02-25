use proc_macro2::{Ident, Span, TokenStream};
use syn::visit::{self, Visit};
use syn::{File, ItemFn};
use syn::{parse_quote, Expr, Lit, LitInt, Result};
use std::str::FromStr;

pub struct FnVisitor<'ast> {
    pub functions: Vec<&'ast ItemFn>,
}

impl<'ast> Visit<'ast> for FnVisitor<'ast> {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.functions.push(node);
        visit::visit_item_fn(self, node);
    }
}

/// ```
/// parse_ident("abstract").unwrap_err();
/// ```
pub fn parse_ident(s: &str) -> Result<Ident> {
    syn::parse2(TokenStream::from_str(s).unwrap())
}

