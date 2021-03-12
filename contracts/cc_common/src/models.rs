use ink_lang as ink;
use ink_prelude::vec::Vec;
use ink_storage::{
    collections::{HashMap as StorageHashMap, Stash as StorageStash, Vec as StorageVec},
    traits::{PackedLayout, SpreadLayout},
    Lazy,
};
use scale::Output;
use ink_env::AccountId;

// ...
type Balance=u64;

/// Indicates whether a transaction is already confirmed or needs further confirmations.
#[derive(scale::Encode, scale::Decode, Clone, Copy, SpreadLayout, PackedLayout)]
#[cfg_attr(
feature = "std",
derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
)]
pub enum ConfirmationStatus {
    /// The transaction is already confirmed.
    Confirmed,
    /// Indicates how many confirmations are remaining.
    ConfirmationsNeeded(u32),
}

/// A Transaction is what every `owner` can submit for confirmation by other owners.
/// If enough owners agree it will be executed by the contract.
#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
pub struct Transaction {
    /// The AccountId of the contract that is called in this transaction.
    pub callee: AccountId,
    /// The selector bytes that identifies the function of the callee that should be called.
    pub selector: [u8; 4],
    /// The SCALE encoded parameters that are passed to the called function.
    pub input: Vec<u8>,
    /// The amount of chain balance that is transferred to the callee.
    pub transferred_value: Balance,
    /// Gas limit for the execution of the call.
    pub gas_limit: u64,
}

/// Errors that can occur upon calling this contract.
#[derive(Copy, Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub enum Error {
    /// Returned if the call failed.
    TransactionFailed,
}

