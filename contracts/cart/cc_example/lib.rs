#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]
#![cfg_attr(not(feature = "std"), no_std)]

mod calculate;

use ink_lang as ink;

#[ink::contract]
mod cc_example {
    use ink_storage::{
        collections::{HashMap as StorageHashMap, Stash as StorageStash, Vec as StorageVec},
        traits::{PackedLayout, SpreadLayout}
    };
    use decimal::prelude::ToPrimitive;
    use chrono::prelude::*;
    use ink_prelude::vec::Vec;
    use ink_prelude::string::String;
    use crate::calculate::{mode, average, average_map};
    // use crate::calculate::{mode, average};
    use ink_prelude::collections::BTreeMap;
    // use statis::average;

    // pub fn average(numbers: &[i32]) -> f32 {
    //     numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
    // }

    // #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default)]
    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std",
        derive(Debug,PartialEq,Eq,
        scale_info::TypeInfo,
        ink_storage::traits::StorageLayout
        )
    )]
    pub struct Decimal{
        num: i64, scale: u32,
    }

    impl Decimal{
        pub fn new(num: i64, scale: u32) -> Self {
            Decimal { num, scale }
        }
    }

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std",
        derive(Debug,PartialEq,Eq,
        scale_info::TypeInfo,
        ink_storage::traits::StorageLayout
        )
    )]
    pub struct ItemSpec {
        // id: Vec<u8>,  // id
        id: String,
        price: Decimal,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct CcExample {
        /// Stores a single `bool` value on the storage.
        value: bool,
        f_val: Decimal,
        str_val: String,
        item_spec: StorageStash<ItemSpec>,
    }

    impl CcExample {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value,
                f_val: Decimal::new(0,0),
                str_val: String::from("2014-11-28 12:00:09"),
                item_spec: StorageStash::new(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
            self.f_val= Decimal::new(1302,2);
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn get_fval(&self) -> i64 {
        // pub fn get_fval(&self) -> f64 {
            let val=decimal::Decimal::new(self.f_val.num, self.f_val.scale);
            val.to_i64().unwrap()
        }

        #[ink(message)]
        pub fn get_dt_rfc3339(&self) -> String{
            let dt=Utc.datetime_from_str(self.str_val.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
            // dt.to_rfc3339().into_bytes()
            dt.to_rfc3339()
        }

        #[ink(message)]
        pub fn get_dt_ms(&self) -> i64{
            let dt=Utc.datetime_from_str(self.str_val.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
            // dt.to_rfc3339().into_bytes()
            dt.timestamp_millis()
        }

        #[ink(message)]
        pub fn compu_mean(&self) -> (i32, i32){
            // use average::{MeanWithError, Estimate, Merge, assert_almost_eq};
            // use rayon::iter::{IntoParallelIterator, ParallelIterator};
            // let a: MeanWithError = (1..6).into_iter().map(f32::from).collect();
            // let r=a.mean();
            let mut numbers = [42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];
            let r=average(&numbers);
            (r as i32, mode(&numbers) as i32)
        }

        #[ink(message)]
        pub fn some_test(&self) {
            use integer_sqrt::IntegerSquareRoot;
            // let i: i32 = 5;
            // let _= (i as f64).sqrt(); // method not found in `f64`
            assert_eq!(4u8.integer_sqrt(), 2);

            let mut numbers = [42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];
            let _r=cc_incs::average(&numbers);
            let _r=cc_incs::mode(&numbers);
        }

        #[ink(message)]
        pub fn add_item(&mut self, name:String, price:Decimal){
            self.item_spec.put(ItemSpec{ id: name, price });
        }

        #[ink(message)]
        pub fn items_total_price(&self) -> i64 {
            let rs=self.item_spec.iter()
                .filter_map(|n|Some(decimal::Decimal::new(n.price.num, n.price.scale).to_i64()))
                .map(|n|n.unwrap())
                .collect::<Vec<i64>>();
            let total:i64=rs.iter().sum();
            let total_dec=decimal::Decimal::from(total);
            total_dec.to_i64().unwrap()
        }

        #[ink(message)]
        pub fn avg_price(&self) -> i64 {
            // let mut price_map=indexmap::IndexMap::new();
            let mut price_map=BTreeMap::new();
            for item in self.item_spec.iter(){
                price_map.insert(item.id.clone(), item.price.num);
            }
            let result=average_map(&price_map);
            // let result=price_map.iter().map(|(_k,v)|v).sum::<i64>() as f64 / price_map.len() as f64;
            result as i64
            // 0
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let cc_example = CcExample::default();
            assert_eq!(cc_example.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut cc_example = CcExample::new(false);
            assert_eq!(cc_example.get(), false);
            cc_example.flip();
            assert_eq!(cc_example.get(), true);
        }

        #[ink::test]
        fn decimal_works() {
            let mut cc_example = CcExample::new(false);
            cc_example.flip();
            let fval=cc_example.get_fval();
            println!("{}", fval);
        }

        #[ink::test]
        fn dt_val_works() {
            let mut cc_example = CcExample::new(false);
            let rfc=cc_example.get_dt_rfc3339();
            println!("{:?}, {}", rfc, cc_example.get_dt_ms());
        }

        #[ink::test]
        fn mean_works() {
            let mut cc_example = CcExample::new(false);
            println!("{:?}", cc_example.compu_mean());
        }

        #[ink::test]
        fn total_works() {
            let mut cc_example = CcExample::new(false);
            cc_example.add_item("basic".to_string(), Decimal::new(1800,2));
            cc_example.add_item("basic".to_string(), Decimal::new(7200,2));
            println!("{:?}", cc_example.items_total_price());
        }

        #[ink::test]
        fn avg_works() {
            let mut cc_example = CcExample::new(false);
            cc_example.add_item("basic".to_string(), Decimal::new(1800,2));
            cc_example.add_item("list".to_string(), Decimal::new(7200,2));
            cc_example.add_item("discount".to_string(), Decimal::new(7200,2));
            println!("{:?}", cc_example.avg_price());
        }
    }
}
