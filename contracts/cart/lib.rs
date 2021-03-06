#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#![cfg_attr(not(feature = "std"), no_std)]

mod common;

use ink_lang as ink;

#[ink::contract]
mod cart {
    #[cfg(feature = "std")]
    use serde::{Serialize, Deserialize};

    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
            Stash as StorageStash,
            Vec as StorageVec,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        Lazy,
    };
    use scale::Output;
    use ink_storage::collections::stash::Entry as StashEntry;
    use crate::common::get_hash_id;
    use ink_env::DefaultEnvironment;
    // use ink_env::Environment;
    // use ink_prelude::collections::BTreeSet;
    use time::{prelude::*, Duration};

    type TransactionId = u32;
    const WRONG_TRANSACTION_ID: &str =
        "The user specified an invalid transaction id. Abort.";
    type ValueId=Vec<u8>;

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std",
    derive(Debug,PartialEq,Eq,
        scale_info::TypeInfo,
        ink_storage::traits::StorageLayout
        )
    )]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
    pub struct OrderHeader{
        pub order_id: Vec<u8>,  // id
        pub order_type_id: Vec<u8>,  // id
        pub order_name: Vec<u8>,  // name
        pub external_id: Vec<u8>,  // id
        pub sales_channel_enum_id: Vec<u8>,  // id
        pub order_date: Timestamp,  // date-time
        pub priority: u8,  // indicator
        pub entry_date: Timestamp,  // date-time
        pub pick_sheet_printed_date: Timestamp,  // date-time
        pub visit_id: Vec<u8>,  // id
        pub status_id: Vec<u8>,  // id
        pub created_by: Vec<u8>,  // id-vlong
        pub first_attempt_order_id: Vec<u8>,  // id
        pub currency_uom: Vec<u8>,  // id
        pub sync_status_id: Vec<u8>,  // id
        pub billing_account_id: Vec<u8>,  // id
        pub origin_facility_id: Vec<u8>,  // id
        pub web_site_id: Vec<u8>,  // id
        pub product_store_id: Vec<u8>,  // id
        pub agreement_id: Vec<u8>,  // id
        pub terminal_id: Vec<u8>,  // id-long
        pub transaction_id: Vec<u8>,  // id-long
        pub auto_order_shopping_list_id: Vec<u8>,  // id
        pub needs_inventory_issuance: u8,  // indicator
        pub is_rush_order: u8,  // indicator
        pub internal_code: Vec<u8>,  // id-long
        pub remaining_sub_total: Balance,  // currency-amount
        pub grand_total: Balance,  // currency-amount
        pub is_viewed: u8,  // indicator
        pub invoice_per_shipment: u8,  // indicator
    }

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std",
        derive(Debug,PartialEq,Eq,
        scale_info::TypeInfo,
        ink_storage::traits::StorageLayout
        )
    )]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize, Clone))]
    #[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
    pub struct ExampleItem{
        pub example_id: Vec<u8>,  // id
        pub example_item_seq_id: Vec<u8>,  // id
        #[cfg_attr(feature = "std", serde(default))]
        pub description: Vec<u8>,  // description
        #[cfg_attr(feature = "std", serde(default))]
        pub amount: u64,  // floating-point
        #[cfg_attr(feature = "std", serde(default))]
        pub amount_uom_id: Vec<u8>,  // id
    }

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std",
        derive(Debug,PartialEq,Eq,
        scale_info::TypeInfo,
        ink_storage::traits::StorageLayout
        )
    )]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize, Clone))]
    #[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
    pub struct CreateExampleItem {
        pub example_id: Vec<u8>,       // id
        pub example_item_seq_id: Vec<u8>,  // id
    }

    impl ExampleItem{
        pub fn new(example_id: Vec<u8>, example_item_seq_id: Vec<u8>) -> Self {
            ExampleItem {
                example_id,
                example_item_seq_id,
                description: Default::default(),
                amount: Default::default(),
                amount_uom_id: Default::default(),
            }
        }
    }

    /// Emitted when an owner submits a transaction.
    #[ink(event)]
    pub struct Submission {
        /// The transaction that was submitted.
        #[ink(topic)]
        transaction: TransactionId,
    }

    /// Emitted when a transaction was canceled.
    #[ink(event)]
    pub struct Cancelation {
        /// The transaction that was canceled.
        #[ink(topic)]
        transaction: TransactionId,
    }

    // type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Cart {
        /// Stores a single `bool` value on the storage.
        value: bool,
        ts: Timestamp,
        ms: i128,
        order_headers: StorageStash<OrderHeader>,
        example_items: StorageStash<ExampleItem>,
        // example_items_idx: StorageHashMap<(ValueId, ValueId), ExampleItem>,
        example_items_bag: StorageHashMap<Hash, Vec<TransactionId>>,
    }

    impl Cart {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {
                value: init_value,
                ts: Timestamp::default(),
                ms: 0,
                order_headers: StorageStash::default(),
                example_items: StorageStash::default(),
                // example_items_idx: StorageHashMap::new(),
                example_items_bag: StorageHashMap::new(),
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
            self.ts=self.env().block_timestamp();

            let mut duration = 1.seconds();
            duration *= -2;
            self.ms=duration.whole_milliseconds();
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn submit_example_item(
            &mut self,
            transaction: ExampleItem,
        ) -> TransactionId {
            self.ensure_caller_is_owner();
            let trans_id = self.example_items.put(transaction);
            self.env().emit_event(Submission {
                transaction: trans_id,
            });
            trans_id
        }

        #[ink(message)]
        pub fn create_example_item(
            &mut self,
            item: CreateExampleItem,
        ) -> TransactionId {
            self.ensure_caller_is_owner();
            let trans_id = self.example_items.put(ExampleItem{
                example_id: item.example_id,
                example_item_seq_id: item.example_item_seq_id,
                description: Default::default(),
                amount: Default::default(),
                amount_uom_id: Default::default(),
            });
            self.env().emit_event(Submission {
                transaction: trans_id,
            });
            trans_id
        }

        #[ink(message)]
        pub fn cancel_example_item(&mut self, trans_id: TransactionId) {
            self.ensure_from_wallet();
            if self.take_example_item(trans_id).is_some() {
                self.env().emit_event(Cancelation {
                    transaction: trans_id,
                });
            }
        }

        /// Remove the transaction identified by `trans_id` from `self.transactions`.
        /// Also removes all confirmation state associated with it.
        fn take_example_item(&mut self, trans_id: TransactionId) -> Option<ExampleItem> {
            let transaction = self.example_items.take(trans_id);
            if let Some(t)= &transaction {
                // self.clean_transaction_confirmations(trans_id);
                let hid=get_hash_id(t.example_id.as_slice());
                self.example_items_bag.take(&hid);
            }
            transaction
        }

        /// Panic if the transaction `trans_id` does not exit.
        fn ensure_example_item_exists(&self, trans_id: TransactionId) {
            self.example_items.get(trans_id).expect(WRONG_TRANSACTION_ID);
        }

        /// Panic if the sender is no owner of the wallet.
        fn ensure_caller_is_owner(&self) {
            // self.ensure_owner(&self.env().caller());
        }

        /// Panic if the sender is not this wallet.
        fn ensure_from_wallet(&self) {
            // assert_eq!(self.env().caller(), self.env().account_id());
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
        use serde_json::json;
        use bstr::{B, ByteSlice, ByteVec};

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let cart = Cart::default();
            assert_eq!(cart.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut cart = Cart::new(false);
            assert_eq!(cart.get(), false);
            cart.flip();
            assert_eq!(cart.get(), true);
        }

        #[ink::test]
        fn example_item_init_works() {
            let order_id="simple";
            let order_item_id="001";
            let item=ExampleItem::new(
                order_id.as_bytes().to_owned(),
                order_item_id.as_bytes().to_owned());
            let mut cart = Cart::new(false);
            cart.submit_example_item(item);
        }

        #[ink::test]
        // fn example_item_json_init_works() -> anyhow::Result<()>{
        fn example_item_json_init_works(){
            let item:ExampleItem=serde_json::from_value(json!({
                "description": b"EX1-001",
                "exampleItemSeqId": b"00001",
                "exampleId": b"EX01",
                "amount": 10
            })).unwrap();

            let mut cart = Cart::new(false);
            cart.submit_example_item(item.to_owned());

            // let item_json=serde_json::to_string_pretty(&item).unwrap();
            println!("{:?}", item);
            println!("{}", item.example_id.to_str().unwrap());
            // Ok(())
        }

        #[ink::test]
        fn create_example_item_works() {
            let mut cart = Cart::new(false);
            for i in 0..10 {
                let id = format!("ex_{}", i);
                let item_id = b"00001";
                let item: CreateExampleItem = serde_json::from_value(json!({
                    "exampleItemSeqId": item_id,
                    "exampleId": id.as_bytes()
                })).unwrap();

                let new_id = cart.create_example_item(item);
                println!("created {}", new_id);
            }

            assert_eq!(10, cart.example_items.len());
        }

        #[ink::test]
        fn example_item_iter_works() {
            let mut cart = Cart::new(false);
            for i in 0..10 {
                let id=format!("ex_{}", i);
                let item_id=b"00001";
                let item:ExampleItem=serde_json::from_value(json!({
                    "description": format!("no.{} example item ", i).as_bytes(),
                    "exampleItemSeqId": item_id,
                    "exampleId": id.as_bytes(),
                    "amount": 10
                })).unwrap();

                let new_id=cart.submit_example_item(item.to_owned());
                // cart.example_items_idx.insert((id.as_bytes().to_vec(), item_id.to_vec()), item);

                // let mut hash_output = [0x00_u8; 32];
                // ink_env::hash_bytes::<ink_env::hash::Keccak256>(id.as_bytes(), &mut hash_output);
                // let hash_id=Hash::from(hash_output);
                let hash_id=get_hash_id(id.as_bytes());
                if !cart.example_items_bag.contains_key(&hash_id){
                    cart.example_items_bag.insert(hash_id, vec![new_id]);
                }else{
                    cart.example_items_bag.get_mut(&hash_id)
                        .unwrap().push(new_id);
                }
            }

            for (trans_id, val) in
            cart.example_items
                .entries()
                .enumerate()
                .filter_map(|(n, entry)| {
                    match entry {
                        StashEntry::Vacant(_) => None,
                        StashEntry::Occupied(value) => Some((n as u32, value)),
                    }
                })
            {
                println!("{} - {:?}", trans_id, val.example_id.to_str().unwrap());
            }

            cart.cancel_example_item(5);
            println!(".. after take 5");

            for (trans_id, val) in
            cart.example_items
                .entries()
                .enumerate()
                .filter_map(|(n, entry)| {
                    match entry {
                        StashEntry::Vacant(_) => None,
                        StashEntry::Occupied(value) => Some((n as u32, value)),
                    }
                })
            {
                println!("{} - {:?}", trans_id, val.example_id.to_str().unwrap());
            }

            assert_eq!(None, cart.example_items.get(5));

            let hid=get_hash_id(b"ex_0");
            let vals=cart.example_items_bag.get(&hid).unwrap();
            println!("ex_0 contains: {:?}", vals);

            let hid=get_hash_id(b"ex_4");
            let vals=cart.example_items_bag.get(&hid).unwrap();
            println!("ex_4 contains: {:?}", vals);

            let hid=get_hash_id(b"ex_5");
            let emp=Vec::new();
            let vals=cart.example_items_bag.get(&hid).unwrap_or(&emp);
            println!("ex_5 contains: {:?}", vals);
        }

        // $ just test timestamp_works
        #[ink::test]
        fn timestamp_works() {
            let mut cart = Cart::new(false);
            cart.flip();
            println!("flip {} on {}", cart.get(), cart.ts);

            assert_eq!(Duration::week(), 604_800.seconds());
            let dura=Duration::weeks(1);
            let ms=dura.whole_milliseconds();
            println!("... {}, {}", ms, cart.ms);
        }
    }
}




