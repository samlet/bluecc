#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#![cfg_attr(not(feature = "std"), no_std)]

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

    type TransactionId = u32;

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

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Cart {
        /// Stores a single `bool` value on the storage.
        value: bool,
        order_headers: StorageStash<OrderHeader>,
        example_items: StorageStash<ExampleItem>,
    }

    impl Cart {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {
                value: init_value,
                order_headers: StorageStash::default(),
                example_items: StorageStash::default(),
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

        /// Panic if the sender is no owner of the wallet.
        fn ensure_caller_is_owner(&self) {
            // self.ensure_owner(&self.env().caller());
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
    }
}
