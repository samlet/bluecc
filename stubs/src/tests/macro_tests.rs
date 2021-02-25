use quote::quote;
use syn::Lit;

#[test]
fn test_await() {
    // Must not parse as Expr::Field.
    let tokens = quote!(fut.await);
    println!("{}", tokens.to_string());

    // crate::snapshot!(tokens as Expr, @r###"
    // Expr::Await {
    //     base: Expr::Path {
    //         path: Path {
    //             segments: [
    //                 PathSegment {
    //                     ident: "fut",
    //                     arguments: None,
    //                 },
    //             ],
    //         },
    //     },
    // }
    // "###);
}


#[test]
fn test_literal_mangling() {
    let code = "0_4";
    let parsed: Lit = syn::parse_str(code).unwrap();
    assert_eq!(code, quote!(#parsed).to_string());
}
