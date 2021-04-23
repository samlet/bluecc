#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]
#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(not(feature = "std"))]
// extern crate alloc;
//
// use cfg_if::cfg_if;
//
// cfg_if! {
//     if #[cfg(feature = "std")] {
//         pub use std::{
//             borrow,
//             boxed,
//             format,
//             string,
//             vec,
//         };
//
//         /// Collection types.
//         pub mod collections {
//             pub use self::{
//                 binary_heap::BinaryHeap,
//                 btree_map::BTreeMap,
//                 btree_set::BTreeSet,
//                 linked_list::LinkedList,
//                 vec_deque::VecDeque,
//                 Bound,
//             };
//             pub use std::collections::*;
//         }
//     } else {
//         pub use alloc::{
//             borrow,
//             boxed,
//             format,
//             string,
//             vec,
//         };
//
//         /// Collection types.
//         pub mod collections {
//             pub use self::{
//                 BTreeMap,
//                 BTreeSet,
//                 BinaryHeap,
//                 LinkedList,
//                 VecDeque,
//             };
//             pub use alloc::collections::*;
//             pub use core::ops::Bound;
//         }
//     }
// }

extern crate ink_prelude;
use ink_prelude::collections::BTreeMap;
use ink_prelude::string::String;

/// AVERAGE:统计选择的数据区域平均值。与MEDIAN不一样，一个是指位置，一个是数值
pub fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

/// MEDIAN:统计选择的数据区域的中间位置的那个数值
pub fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

/// MODE：统计选择的数据区域出现次数最多的数值
pub fn mode(numbers: &[i32]) -> i32 {
    // let mut occurrences = <StorageHashMap<i32, i32>>::new();
    let mut occurrences = <BTreeMap<i32, i32>>::new();
    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    let r=occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers");
    r
}

pub fn average_map(numbers: BTreeMap<String, i64>) -> f64 {
    numbers.iter().map(|(_k,v)|v).sum::<i64>() as f64
        / numbers.len() as f64
}

#[cfg(test)]
mod tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;

    #[test]
    fn stats_works(){
        let mut numbers = [42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];
        println!("AVERAGE: {}", average(&numbers));
        println!("MEDIAN: {}", median(&mut numbers));
        println!("MODE: {}", mode(&numbers));
    }
}

