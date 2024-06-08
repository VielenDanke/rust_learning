use std::collections::HashMap;

/// Represents an account on the blockchain
/// This is basically the primary part of the "world state" of the blockchain
/// It is the final status after performing all blocks in order
#[derive(Clone, Debug)]
pub struct Account {
    /// We want the account to be able to store any information we want (Dictionary)
    store: HashMap<String, String>,

    /// store if this is a user account or sth else
    acc_type: AccountType,

    /// Amount of tokens that account owns (like BTC or ETH)
    pub tokens: u128,
}


impl Account {
    /// Constructor
    pub fn new(account_type: AccountType) -> Self {
        return Self {
            tokens: 0,
            acc_type: account_type,
            store: HashMap::new()
        }
    }
}

/// We can support different types of accounts
/// which could be used to represent different roles within the system
/// This is just for later extension, for now we will only use User accounts
#[derive(Clone, Debug)]
pub enum AccountType {
    /// A common user account
    User,

    /// An account that technically does not represent an individual
    /// Think of this like a SmartContract in Ethereum. We will not use it
    /// in our implementation. It's just here if you want to go on implementing
    /// to provide a starting point for more :)
    Contract,

    /// Add whatever roles you need.
    /// Again, we will NOT make use of this for the example here
    Validator {
        // Again, enum members in rust may store additional data
        correctly_validated_blocks: u128,
        incorrectly_validated_blocks: u128,
        you_get_the_idea: bool,
    },
}
