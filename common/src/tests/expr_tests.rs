/// ref: https://github.com/rekka/meval-rs
#[test]
fn expr_bind_works() {
    let expr: meval::Expr = "sin(pi * x)".parse().unwrap();
    let func = expr.bind("x").unwrap();

    let vs: Vec<_> = (0..100+1).map(|i| func(i as f64 / 100.)).collect();

    println!("sin(pi * x), 0 <= x <= 1: {:?}", vs);
}

