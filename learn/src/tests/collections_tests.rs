use std::collections::{HashSet, HashMap};

/// ref: https://www.tutorialspoint.com/rust/rust_collections.htm
///
#[test]
fn set_works() {
    let mut names = HashSet::new();
    names.insert("Mohtashim");
    names.insert("Kannan");
    names.insert("TutorialsPoint");
    names.insert("Mohtashim");

    match names.get(&"Mohtashim"){
        Some(value)=>{
            println!("found {}",value);
        }
        None =>{
            println!("not found");
        }
    }
    println!("{:?}",names);
}

#[test]
fn map_iter_works() {
    let mut state_codes = HashMap::new();
    state_codes.insert("KL", "Kerala");
    state_codes.insert("MH", "Maharashtra");

    for (key, val) in state_codes.iter() {
        println!("key: {} val: {}", key, val);
    }
}

#[test]
fn vec_iter_works() {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(500);

    for i in &v {
        println!("{}",i);
    }
    println!("{:?}",v);
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct EntryMove {
    from: u32,
    to: u32,
    value: u8,
}

