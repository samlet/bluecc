use ink_storage::collections::Stash as StorageStash;
use ink_storage::collections::HashMap as StorageHashMap;

#[test]
fn new_works() {
    let test_values = [b'A', b'B', b'C', b'D', b'E', b'F'];
    assert_eq!(test_values.is_empty(), false);
}


#[test]
fn from_iterator_works() {
    let test_values = [b'A', b'B', b'C', b'D', b'E', b'F'];
    let stash = test_values.iter().copied().collect::<StorageStash<_>>();
    assert_eq!(stash, {
        let mut stash = StorageStash::new();
        for (index, value) in test_values.iter().enumerate() {
            assert_eq!(index as u32, stash.put(*value));
        }
        stash
    });
    assert_eq!(stash.len(), test_values.len() as u32);
    assert_eq!(stash.is_empty(), false);
}

#[test]
fn insert_works() {
    let mut hmap = <StorageHashMap<u8, i32>>::new();
    // Start with an empty hash map.
    assert_eq!(hmap.len(), 0);
    assert_eq!(hmap.get(&b'A'), None);
    // Insert first value.
    hmap.insert(b'A', 1);
    assert_eq!(hmap.len(), 1);
    assert_eq!(hmap.get(&b'A'), Some(&1));
    assert_eq!(hmap.get_mut(&b'A'), Some(&mut 1));
    // Update the inserted value.
    hmap.insert(b'A', 2);
    assert_eq!(hmap.len(), 1);
    assert_eq!(hmap.get(&b'A'), Some(&2));
    assert_eq!(hmap.get_mut(&b'A'), Some(&mut 2));
    // Insert another value.
    hmap.insert(b'B', 3);
    assert_eq!(hmap.len(), 2);
    assert_eq!(hmap.get(&b'B'), Some(&3));
    assert_eq!(hmap.get_mut(&b'B'), Some(&mut 3));
}