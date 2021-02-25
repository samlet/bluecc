use quote::quote;
use syn::visit::{self, Visit};
use syn::{File, ItemFn};

// syn = { version = "1.0", features = ["full", "visit-mut"] }
use syn::visit_mut::{self, VisitMut};
use syn::{parse_quote, Expr, Lit, LitInt};
use crate::utils::FnVisitor;

/// ref: https://docs.rs/syn/1.0.60/syn/visit/index.html
#[test]
fn visit_works() {
    let code = quote! {
        pub fn f() {
            fn g() {}
        }
    };

    let syntax_tree: File = syn::parse2(code).unwrap();
    let mut visitor = FnVisitor { functions: Vec::new() };
    visitor.visit_file(&syntax_tree);
    for f in visitor.functions {
        println!("Function with name={}", f.sig.ident);
    }
}

struct BigintReplace;

impl VisitMut for BigintReplace {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Lit(expr) = &node {
            if let Lit::Int(int) = &expr.lit {
                if int.suffix() == "u256" {
                    let digits = int.base10_digits();
                    let unsuffixed: LitInt = syn::parse_str(digits).unwrap();
                    *node = parse_quote!(bigint::u256!(#unsuffixed));
                    return;
                }
            }
        }

        // Delegate to the default impl to visit nested expressions.
        visit_mut::visit_expr_mut(self, node);
    }
}

/// ref: https://docs.rs/syn/1.0.60/syn/visit_mut/index.html
#[test]
fn visit_mut_works() {
    let code = quote! {
        fn main() {
            let _ = 999u256;
        }
    };

    let mut syntax_tree: File = syn::parse2(code).unwrap();
    BigintReplace.visit_file_mut(&mut syntax_tree);
    println!("{}", quote!(#syntax_tree));
}