use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::{HashSet as Set, HashSet};
use syn::fold::{self, Fold};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, parse_quote, File, Expr, Ident, ItemFn, Local, Pat, Stmt, Token};
use syn::visit::{self, Visit};

use bluecc_stubs::utils::{FnVisitor, parse_ident};

/// Parses a list of variable names separated by commas.
///
///     a, b, c
///
/// This is how the compiler passes in arguments to our attribute -- it is
/// everything inside the delimiters after the attribute name.
///
///     #[trace_var(a, b, c)]
///                 ^^^^^^^
pub struct Args {
    vars: Set<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

impl Args {
    /// Determines whether the given `Expr` is a path referring to one of the
    /// variables we intend to print. Expressions are used as the left-hand side
    /// of the assignment operator.
    fn should_print_expr(&self, e: &Expr) -> bool {
        match *e {
            Expr::Path(ref e) => {
                if e.path.leading_colon.is_some() {
                    false
                } else if e.path.segments.len() != 1 {
                    false
                } else {
                    let first = e.path.segments.first().unwrap();
                    self.vars.contains(&first.ident) && first.arguments.is_empty()
                }
            }
            _ => false,
        }
    }

    /// Determines whether the given `Pat` is an identifier equal to one of the
    /// variables we intend to print. Patterns are used as the left-hand side of
    /// a `let` binding.
    fn should_print_pat(&self, p: &Pat) -> bool {
        match p {
            Pat::Ident(ref p) => self.vars.contains(&p.ident),
            _ => false,
        }
    }

    /// Produces an expression that assigns the right-hand side to the left-hand
    /// side and then prints the value.
    ///
    ///     // Before
    ///     VAR = INIT
    ///
    ///     // After
    ///     { VAR = INIT; println!("VAR = {:?}", VAR); }
    fn assign_and_print(&mut self, left: Expr, op: &dyn ToTokens, right: Expr) -> Expr {
        let right = fold::fold_expr(self, right);
        parse_quote!({
            #left #op #right;
            println!(concat!(stringify!(#left), " = {:?}"), #left);
        })
    }

    /// Produces a let-binding that assigns the right-hand side to the left-hand
    /// side and then prints the value.
    ///
    ///     // Before
    ///     let VAR = INIT;
    ///
    ///     // After
    ///     let VAR = { let VAR = INIT; println!("VAR = {:?}", VAR); VAR };
    fn let_and_print(&mut self, local: Local) -> Stmt {
        let Local { pat, init, .. } = local;
        let init = self.fold_expr(*init.unwrap().1);
        let ident = match pat {
            Pat::Ident(ref p) => &p.ident,
            _ => unreachable!(),
        };
        parse_quote! {
            let #pat = {
                #[allow(unused_mut)]
                let #pat = #init;
                println!(concat!(stringify!(#ident), " = {:?}"), #ident);
                #ident
            };
        }
    }
}

/// The `Fold` trait is a way to traverse an owned syntax tree and replace some
/// of its nodes.
///
/// Syn provides two other syntax tree traversal traits: `Visit` which walks a
/// shared borrow of a syntax tree, and `VisitMut` which walks an exclusive
/// borrow of a syntax tree and can mutate it in place.
///
/// All three traits have a method corresponding to each type of node in Syn's
/// syntax tree. All of these methods have default no-op implementations that
/// simply recurse on any child nodes. We can override only those methods for
/// which we want non-default behavior. In this case the traversal needs to
/// transform `Expr` and `Stmt` nodes.
impl Fold for Args {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Assign(e) => {
                if self.should_print_expr(&e.left) {
                    self.assign_and_print(*e.left, &e.eq_token, *e.right)
                } else {
                    Expr::Assign(fold::fold_expr_assign(self, e))
                }
            }
            Expr::AssignOp(e) => {
                if self.should_print_expr(&e.left) {
                    self.assign_and_print(*e.left, &e.op, *e.right)
                } else {
                    Expr::AssignOp(fold::fold_expr_assign_op(self, e))
                }
            }
            _ => fold::fold_expr(self, e),
        }
    }

    fn fold_stmt(&mut self, s: Stmt) -> Stmt {
        match s {
            Stmt::Local(s) => {
                if s.init.is_some() && self.should_print_pat(&s.pat) {
                    self.let_and_print(s)
                } else {
                    Stmt::Local(fold::fold_local(self, s))
                }
            }
            _ => fold::fold_stmt(self, s),
        }
    }
}

pub fn trace_var(args: &mut Args, input: ItemFn) -> TokenStream {
    // let input = parse_macro_input!(input as ItemFn);
    // // Parse the list of variables the user wanted to print.
    // let mut args = parse_macro_input!(args as Args);

    // Use a syntax tree traversal to transform the function body.
    let output = args.fold_item_fn(input);

    // Hand the resulting function body back to the compiler.
    TokenStream::from(quote!(#output))
}

fn main() {
    let code = quote! {
        fn factorial(mut n: u64) -> u64 {
            let mut p = 1;
            while n > 1 {
                p *= n;
                n -= 1;
            }
            p
        }
    };
    // println!("{}", quote!(#code));
    let mut ids = HashSet::new();
    ids.insert(parse_ident("p").unwrap());
    let mut args=Args{vars:ids};

    let syntax_tree: File = syn::parse2(code).unwrap();
    let mut visitor = FnVisitor { functions: Vec::new() };
    visitor.visit_file(&syntax_tree);
    for f in visitor.functions {
        println!("Function with name={}", f.sig.ident);
        let output=trace_var(&mut args, f.clone());

        println!("{:#?}", output); // tokens tree structure
        println!("{}", output); // generated code
    }

    // println!("{}", code);
}

