use std::collections::HashMap;

#[derive(Debug)]
struct ComplexThing {
    number: Option<i32>,
    string: String,
    tuple: ((), HashMap<i32, &'static str>)
}

fn main() {
    let mut x = HashMap::new();

    for i in 0..10 {
        x.insert(format!("Key {}", i), ComplexThing {
            number: if i % 3 == 0 { None } else { Some(i) },
            string: format!("I am string #{}", i),
            tuple: ((), HashMap::new()),
        });
    }

    println!("? is comparatively hard to read:\n{:?}\n", x);
    // ref: https://www.reddit.com/r/rust/comments/3ceaui/psa_produces_prettyprinted_debug_output/
    println!("#? is much easier to comprehend:\n{:#?}", x);
}

