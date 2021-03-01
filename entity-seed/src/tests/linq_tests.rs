use functor::linq;
use functor::iter::Enumerable;

#[test]
fn try_linq() {
    let x = 1..100;
    let y: Vec<i32> = x.clone().map(|p| p * 2).collect();
    let e: Vec<i32> = linq!(from p in x.clone(), select p * 2).collect();
    assert_eq!(e, y);
}

/// https://gist.github.com/leonardo-m/6e9315a57fe9caa893472c2935e9d589
#[test]
fn where_indexed() {
    // linq5: Where - Indexed
    let digits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let short_digits = digits
        .iter()
        .enumerate()
        .filter(|&(index, &digit)| digit.len() < index)
        .map(|(_, &digit)| digit);

    println!("Short digits:");
    for d in short_digits {
        println!("The word {} is shorter than its value.", d);
    }
}

// linq9: Select - Anonymous Types 1
#[test]
fn anonymous_types() {
    let words = ["aPPLE", "BlUeBeRrY", "cHeRry"];

    struct ULPair { upper: String, lower: String }
    let upper_lower_words =
        words
            .iter()
            .map(|&w| ULPair {upper: w.to_uppercase(), lower: w.to_lowercase()});

    for ul in upper_lower_words {
        println!("Uppercase: {}, Lowercase: {}", ul.upper, ul.lower);
    }
}

