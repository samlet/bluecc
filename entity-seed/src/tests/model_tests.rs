use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
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

type TransactionId = u32;
type AccountId = u32;
type Balance = u32;

pub struct MultisigPlain {
    /// Every entry in this map represents the confirmation of an owner for a
    /// transaction. This is effecively a set rather than a map.
    confirmations: HashMap<(TransactionId, AccountId), ()>,
    /// The amount of confirmations for every transaction. This is a redundant
    /// information and is kept in order to prevent iterating through the
    /// confirmation set to check if a transaction is confirmed.
    confirmation_count: HashMap<TransactionId, u32>,
    /// Just the list of transactions. It is a stash as stable ids are necessary
    /// for referencing them in confirmation calls.
    // transactions: StorageStash<Transaction>,
    transactions: Vec<Transaction>,
    /// The list is a vector because iterating over it is necessary when cleaning
    /// up the confirmation set.
    owners: Vec<AccountId>,
    /// Redundant information to speed up the check whether a caller is an owner.
    is_owner: HashMap<AccountId, ()>,
    /// Minimum number of owners that have to confirm a transaction to be executed.
    requirement: u32,
}

impl MultisigPlain {
    pub fn new(requirement: u32, owners: Vec<AccountId>) -> Self {
        let is_owner: HashMap<_, _, _> =
            owners.iter().copied().map(|owner| (owner, ())).collect();
        let owners: Vec<_> = owners.iter().copied().collect();

        assert!(is_owner.len() == owners.len());
        Self {
            confirmations: HashMap::default(),
            confirmation_count: HashMap::default(),
            // transactions: StorageStash::default(),
            transactions: Vec::default(),
            owners,
            is_owner,
            requirement: requirement,
        }
    }
}

#[test]
fn model_works() {
    let sig=MultisigPlain::new(1, vec![10]);
    println!("{:#?}", sig.owners);
}

