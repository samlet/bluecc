// [dependencies]
// quote = "1.0"
// syn = { version = "1.0", features = ["full", "visit"] }

use quote::quote;
use syn::visit::{self, Visit};
use syn::{File, ItemFn};

/*
ref: https://docs.rs/syn/1.0.60/syn/visit/index.html
 */

struct FnVisitor;

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        println!("Function with name={}", node.sig.ident);

        // Delegate to the default impl to visit any nested functions.
        visit::visit_item_fn(self, node);
    }
}

fn main() {
    let code = quote! {
        pub fn f() {
            fn g() {}
        }
    };

    let syntax_tree: File = syn::parse2(code).unwrap();
    FnVisitor.visit_file(&syntax_tree);
}

