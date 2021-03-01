use std::collections::btree_map::BTreeMap;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[test]
fn vec_test(){
    let vec = vec![1, 2, 3, 4];
    for x in vec.iter() {
        println!("vec contained {}", x);
    }

    for x in vec.iter().rev() {
        println!("vec contained {}", x);
    }
}

#[test]
fn vec_iter_mut_works() {
    let mut vec = vec![1, 2, 3, 4];
    for x in vec.iter_mut() {
        *x += 1;
    }
    println!("{:?}", vec);
}

#[test]
fn tree_works() {
    let mut count = BTreeMap::new();
    let message = "she sells sea shells by the sea shore";

    // Entry API的目的是提供一个高效的机制，以有条件地操作 map 的内容，
    // 是否存在密钥为条件。其主要的动机用例是提供有效的累积器map。
    // 例如，如果一个人希望统计每个key被看到的次数，他们必须执行一些条件逻辑来判断
    // 这个键是否是第一次被看到。通常情况下，这将需要在查找之后再进行插入，
    // 有效地重复了每次插入时的搜索工作。
    for c in message.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    assert_eq!(count.get(&'s'), Some(&8));

    println!("Number of occurrences of each character");
    for (char, count) in &count {
        println!("{}: {}", char, count);
    }
}

/// 追踪顾客在酒吧的醉酒情况
#[test]
fn blood_alcohol_works() {
    // A client of the bar. They have a blood alcohol level.
    struct Person { blood_alcohol: f32 }

    // All the orders made to the bar, by client ID.
    let orders = vec![1, 2, 1, 2, 3, 4, 1, 2, 2, 3, 4, 1, 1, 1];

    // Our clients.
    let mut blood_alcohol = BTreeMap::new();

    for id in orders {
        // If this is the first time we've seen this customer, initialize them
        // with no blood alcohol. Otherwise, just retrieve them.
        let person = blood_alcohol.entry(id).or_insert(Person { blood_alcohol: 0.0 });

        // Reduce their blood alcohol level. It takes time to order and drink a beer!
        person.blood_alcohol *= 0.9;

        // Check if they're sober enough to have another beer.
        if person.blood_alcohol > 0.3 {
            // Too drunk... for now.
            println!("Sorry {}, I have to cut you off", id);
        } else {
            // Have another!
            person.blood_alcohol += 0.1;
        }
    }
}

#[derive(Debug)]
struct Foo {
    a: u32,
    b: &'static str,
}

// we will compare `Foo`s by their `a` value only.
impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool { self.a == other.a }
}

impl Eq for Foo {}

// we will hash `Foo`s by their `a` value only.
impl Hash for Foo {
    fn hash<H: Hasher>(&self, h: &mut H) { self.a.hash(h); }
}

impl PartialOrd for Foo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.a.partial_cmp(&other.a) }
}

impl Ord for Foo {
    fn cmp(&self, other: &Self) -> Ordering { self.a.cmp(&other.a) }
}

/// 插入和复合键#
/// 如果我们有一个比较复杂的键，调用插入将不会更新该键的值。
/// ref: https://skyao.io/learning-rust/std/collections/
#[test]
fn compound_key_works() {
    let mut map = BTreeMap::new();
    map.insert(Foo { a: 1, b: "baz" }, 99);

    // We already have a Foo with an a of 1, so this will be updating the value.
    map.insert(Foo { a: 1, b: "xyz" }, 100);

    // The value has been updated...
    assert_eq!(map.values().next().unwrap(), &100);

    // ...but the key hasn't changed. b is still "baz", not "xyz".
    assert_eq!(map.keys().next().unwrap().b, "baz");

    let result:Vec<&i32>=map.values().filter(|&x| *x>99).collect();
    println!("{:?}", result);
}

