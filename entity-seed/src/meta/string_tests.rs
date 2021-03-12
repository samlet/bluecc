use itertools::Itertools;
use serde::de::Expected;

#[test]
fn str_from_utf8_works() {
    // 将UTF-8序列转为字符串
    let tao = std::str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
    assert_eq!("道", tao);

    // 将16进制Unicode码位转为字符串
    assert_eq!("道", String::from("\u{9053}"));

    let unicode_x = 0x9053;
    let utf_x_hex = 0xe98193;
    let utf_x_bin  = 0b111010011000000110010011;
    println!("unicode_x: {:b}", unicode_x);
    println!("utf_x_hex: {:b}", utf_x_hex);
    println!("utf_x_bin: 0x{:x}", utf_x_bin);
}

#[test]
fn split_works() {
    let sentence = "This is a sentence in Rust.";
    let words: Vec<&str> = sentence
        .split_whitespace()
        .collect();
    let words_containing_i: Vec<&str> = words
        .into_iter()
        .filter(|word| word.contains("i"))
        .collect();
    println!("{:?}", words_containing_i);
}

#[test]
fn parse_works() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);

    //
    let strings = vec!["93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);

    // partition
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

#[test]
fn format_works() {
    let data = [1.1, 2.71828, -3.];
    let data_formatter = data.iter()
        .format_with(", ",
                     |elt, f| f(&format_args!("{:.2}", elt)));
    assert_eq!(format!("{}", data_formatter),
               "1.10, 2.72, -3.00");

    // .format_with() is recursively composable
    let matrix = [[1., 2., 3.],
        [4., 5., 6.]];
    let matrix_formatter = matrix.iter().format_with("\n", |row, f| {
        f(&row.iter().format_with(", ", |elt, g| g(&elt)))
    });
    assert_eq!(format!("{}", matrix_formatter),
               "1, 2, 3\n4, 5, 6");

}
