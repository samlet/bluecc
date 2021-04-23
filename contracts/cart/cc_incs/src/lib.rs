#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]
#![cfg_attr(not(feature = "std"), no_std)]

/// AVERAGE:统计选择的数据区域平均值。与MEDIAN不一样，一个是指位置，一个是数值
pub fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
