use itertools::Itertools;
use std::sync::mpsc::channel;

/// ref: https://dev.to/natserract/functional-programming-in-rust-3im8
///
fn fmt(prev_str: &str) -> String {
    let mut new_str = String::new();

    let closure_annotated = |next_str| -> String {
        new_str.push_str(prev_str);
        new_str.push_str(next_str);
        return new_str;
    };

    closure_annotated("dolor sit amet")
}

#[test]
fn closure_annotated_works() {
    let r_txt = "Lorem ipsum ";
    assert_eq!("Lorem ipsum dolor sit amet", fmt(r_txt));
}

#[derive(Debug)]
struct States<'a> {
    a: &'a i32,
    b: &'a i32,
}

trait Currying {
    type ReturnType: Fn(i32) -> i32;
    fn add(self) -> Self::ReturnType;
}

impl Currying for States<'static>{
    type ReturnType = Box<dyn Fn(i32) -> i32>;

    fn add(self) -> Self::ReturnType {
        Box::new(move|x| {
            x * self.a
        })
    }
}

#[test]
fn currying_works() {
    let r_value: States = States {
        a: &100,
        b: &100
    };

    let r1 = r_value.add();
    let r2 = r1(5);

    assert_eq!(500, r2);
}

#[allow(non_camel_case_types)]
type i64_t = i64;

trait Factor {
    fn factorial_tail_rec(val: i64_t) -> Self;
    fn factorial(num: i64_t) -> Self;
}

impl Factor for i64_t {
    fn factorial_tail_rec(val: i64_t) -> Self {
        val
    }

    fn factorial(num: i64_t) -> Self {
        match num {
            0 => 1,
            _ => num * Self::factorial_tail_rec(num - 1)
        }
    }
}
#[test]
fn recursion_works() {
    let result: i64_t = Factor::factorial(3);
    assert_eq!(6, result);
}

fn map<F>(arr: &[i32], func: F) -> Vec<i32> where F: Fn(&i32) -> i32{
    let mut new_array: Vec<i32> = vec![];
    for i in arr.iter() {
        new_array.push(func(i))
    }

    return new_array
}

#[test]
fn higher_order_functions_works() {
    let lists = vec![1, 4, 9, 16];
    let result = map(&lists, |i| *i + 2);

    assert_eq!(vec![3, 6, 11, 18], result)
}

struct State {
    x: i32,
}

trait Lazy {
    fn add(&self) -> i32;
    fn multiply(&self) -> i32;
    fn add_or_multiply(&self, add: bool) -> i32;
}

impl Lazy for State {
    fn add(&self) -> i32 {
        println!("executing add");
        &self.x + &self.x
    }

    fn multiply(&self) -> i32 {
        println!("executing multiply");
        &self.x * &self.x
    }

    fn add_or_multiply(&self, add: bool) -> i32 {
        match add {
            true => self.add(),
            false =>  self.multiply(),
        }
    }
}

#[test]
fn lazy_works() {
    let val: State = State {
        x: 20
    };

    assert_eq!(40, val.add_or_multiply(true));
    assert_eq!(400, val.add_or_multiply(false));
}

/// ref: https://mmstick.gitbooks.io/rust-programming-phoronix-reader-how-to/content/chapter02.html
#[test]
fn iter_works() {
    let numbers_iterator = [0,2,3,4,5].iter();
    let sum = numbers_iterator
        .fold(0, |total, next| total + next);
    let squared:Vec<i32> = (1..10).map(|x| x * x).collect();
    println!("{}, {:?}", sum, squared);
}

/// ref: https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.cartesian_product
// cartesian_product返回迭代器适配器，该适配器对两个迭代器self和J的元素集的笛卡尔乘积进行迭代。
#[test]
fn cartesian_product_works() {
    let it = (0..2).cartesian_product("αβ".chars());
    itertools::assert_equal(it, vec![(0, 'α'), (0, 'β'), (1, 'α'), (1, 'β')]);
}

#[test]
fn multi_prod_works() {
    let mut multi_prod = (0..3).map(|i| (i * 2)..(i * 2 + 2))
        .multi_cartesian_product();
    assert_eq!(multi_prod.next(), Some(vec![0, 2, 4]));
    assert_eq!(multi_prod.next(), Some(vec![0, 2, 5]));
    assert_eq!(multi_prod.next(), Some(vec![0, 3, 4]));
    assert_eq!(multi_prod.next(), Some(vec![0, 3, 5]));
    assert_eq!(multi_prod.next(), Some(vec![1, 2, 4]));
    assert_eq!(multi_prod.next(), Some(vec![1, 2, 5]));
    assert_eq!(multi_prod.next(), Some(vec![1, 3, 4]));
    assert_eq!(multi_prod.next(), Some(vec![1, 3, 5]));
    assert_eq!(multi_prod.next(), None);
}

/// 返回一个迭代器适配器，该适配器使用传入的闭包（可选）将连续的元素合并在一起。
#[test]
fn coalesce_works() {
    // sum same-sign runs together
    let data = vec![-1., -2., -3., 3., 1., 0., -1.];
    itertools::assert_equal(data.into_iter().coalesce(|x, y|
        if (x >= 0.) == (y >= 0.) {
            Ok(x + y)
        } else {
            Err((x, y))
        }),
        vec![-6., 4., -1.]);
}

#[test]
fn unique_by_works() {
    let data = vec!["a", "bb", "aa", "c", "ccc"];
    itertools::assert_equal(data.into_iter().unique_by(|s| s.len()),
                            vec!["a", "bb", "ccc"]);
}

#[test]
fn for_each_works() {
    let (tx, rx) = channel();
    (0..5).map(|x| x * 2 + 1)
        .for_each(move |x| tx.send(x).unwrap());

    let v: Vec<_> =  rx.iter().collect();
    assert_eq!(v, vec![1, 3, 5, 7, 9]);
}

#[test]
fn range_works() {
    let x=1;
    let r=x * 100 .. x * 110;
    println!("{}: {:?}", r.len(), r);

    //
    for x in 0..5 {
        println!("{}", x);
    }
}

#[test]
fn for_each_print_works() {
    (0..5).flat_map(|x| x * 100 .. x * 110)
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0)
        .for_each(|(i, x)| println!("{}:{}", i, x));
}

#[test]
fn collect_works() {
    let words = ["alpha", "beta", "gamma"];

    // chars() returns an iterator
    let merged: String = words.iter()
        .flat_map(|s| s.chars())
        .collect();
    assert_eq!(merged, "alphabetagamma");

    // as specific type
    let vector = (1..)            // Infinite range of integers
        .filter(|x| x % 2 != 0)   // Collect odd numbers
        .take(5)                  // Only take five numbers
        .map(|x| x * x)           // Square each number
        .collect::<Vec<usize>>(); // Return as a new Vec<usize>
    println!("{:?}", vector);     // Print result
}

#[test]
fn map_works() {
    //
    let a = [1, 2, 3];
    let mut iter = a.iter().map(|x| 2 * x);

    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), None);

    // copied
    let a = [1, 2, 3];
    let v_copied: Vec<_> = a.iter().copied().collect();

    // copied is the same as .map(|&x| x)
    let v_map: Vec<_> = a.iter().map(|&x| x).collect();

    assert_eq!(v_copied, vec![1, 2, 3]);
    assert_eq!(v_map, vec![1, 2, 3]);
}

#[test]
fn partition_works() {
    let a = [1, 2, 3];

    let (even, odd): (Vec<i32>, Vec<i32>) = a
        .iter()
        .partition(|&n| n % 2 == 0);

    assert_eq!(even, vec![2]);
    assert_eq!(odd, vec![1, 3]);
}

#[test]
fn find_works() {
    let text = "Hα";
    assert_eq!(text.chars().find_position(|ch| ch.is_lowercase()), Some((1, 'α')));

    // Basic usage:
    let a = [1, 2, 3];
    assert_eq!(a.iter().position(|&x| x == 2), Some(1));
    assert_eq!(a.iter().position(|&x| x == 5), None);

    // Stopping at the first true:
    let a = [1, 2, 3, 4];
    let mut iter = a.iter();

    assert_eq!(iter.position(|&x| x >= 2), Some(1));
    // we can still use `iter`, as there are more elements.
    assert_eq!(iter.next(), Some(&3));
    // The returned index depends on iterator state
    assert_eq!(iter.position(|&x| x == 4), Some(0));

    // rposition() is short-circuiting; in other words, it will stop processing as soon as it finds a true.
    let a = [1, 2, 3];
    assert_eq!(a.iter().rposition(|&x| x == 3), Some(2));
    assert_eq!(a.iter().rposition(|&x| x == 5), None);
}

#[test]
fn find_first_works() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let r=vec1.iter().find(|&&x| x == 2).unwrap_or(&0);
    assert_eq!(&2, r);  // &2 is value, not index

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    // `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    // let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`
    // println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}

#[test]
fn find_map_works() {
    let a = ["lol", "NaN", "2", "5"];

    let first_number = a.iter().find_map(|s| s.parse().ok());
    assert_eq!(first_number, Some(2));
}

#[test]
fn filter_works() {
    let a = [0i32, 1, 2];

    let mut iter = a.iter().filter(|x| x.is_positive());

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);

    // Because the closure passed to filter() takes a reference, and many iterators iterate over references, this leads to a possibly confusing situation, where the type of the closure is a double reference:
    let a = [0, 1, 2];

    let mut iter = a.iter().filter(|x| **x > 1); // need two *s!
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);

    let mut iter = a.iter().filter(|&x| *x > 1); // both & and *
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);

    let mut iter = a.iter().filter(|&&x| x > 1); // two &s
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);

    // Creates an iterator that both filters and maps.
    // The returned iterator yields only the values for which the supplied closure returns Some(value).
    let a = ["1", "two", "NaN", "four", "5"];
    let mut iter = a.iter().filter_map(|s| s.parse().ok());
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);

}

#[test]
fn concat_works() {
    let input = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
    assert_eq!(input.into_iter().concat(),
               vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn join_works() {
    assert_eq!(["a", "b", "c"].iter().join(", "), "a, b, c");
    assert_eq!([1, 2, 3].iter().join(", "), "1, 2, 3");
}

/// https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.into_group_map
#[test]
fn group_map_works() {
    let data = vec![(0, 10), (2, 12), (3, 13), (0, 20), (3, 33), (2, 42)];
    let lookup = data.into_iter().into_group_map();

    assert_eq!(lookup[&0], vec![10, 20]);
    assert_eq!(lookup.get(&1), None);
    assert_eq!(lookup[&2], vec![12, 42]);
    assert_eq!(lookup[&3], vec![13, 33]);
}

/// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan
/// scan（）接受两个参数：一个初始值，该初始值填充内部状态；
/// 一个闭包，包含两个参数，第一个是对内部状态的可变引用，第二个是迭代器元素。闭包可以分配给内部状态，以在迭代之间共享状态。
#[test]
fn scan_works() {
    let a = [1, 2, 3];

    let mut iter = a.iter().scan(1, |state, &x| {
        // each iteration, we'll multiply the state by the element
        *state = *state * x;

        // then, we'll yield the negation of the state
        Some(-*state)
    });

    assert_eq!(iter.next(), Some(-1));
    assert_eq!(iter.next(), Some(-2));
    assert_eq!(iter.next(), Some(-6));
    assert_eq!(iter.next(), None);
}

#[test]
fn inspect_works() {
    let a = [1, 4, 2, 3];

    // this iterator sequence is complex.
    let sum = a.iter()
        .cloned()
        .filter(|x| x % 2 == 0)
        .fold(0, |sum, i| sum + i);

    println!("{}", sum);

    // let's add some inspect() calls to investigate what's happening
    let sum = a.iter()
        .cloned()
        .inspect(|x| println!("about to filter: {}", x))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("made it through filter: {}", x))
        .fold(0, |sum, i| sum + i);

    println!("{}", sum);

    // Logging errors before discarding them:
    let lines = ["1", "2", "a"];

    let sum: i32 = lines
        .iter()
        .map(|line| line.parse::<i32>())
        .inspect(|num| {
            if let Err(ref e) = *num {
                println!("Parsing error: {}", e);
            }
        })
        .filter_map(Result::ok)
        .sum();

    println!("Sum: {}", sum);

}