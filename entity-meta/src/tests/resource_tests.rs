/// ref: https://doc.rust-lang.org/std/macro.include_bytes.html
#[test]
fn includes_works() {
    let bytes = include_bytes!("spanish.in");
    assert_eq!(bytes, b"adi\xc3\xb3s\n");
    print!("{}", String::from_utf8_lossy(bytes));
}
