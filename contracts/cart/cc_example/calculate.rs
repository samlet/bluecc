// use ink_storage::collections::{
//         HashMap as StorageHashMap,
//         Stash as StorageStash,
//         Vec as StorageVec,
//     };
// use std::collections::BTreeMap;
use ink_prelude::collections::BTreeMap;

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

