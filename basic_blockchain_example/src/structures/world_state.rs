use std::sync::{Arc, RwLock};

use crate::structures::account::{Account, AccountType};

/// Represents the current state of the blockchain after all Blocks are executed
/// A world state is technically not necessary since we always could build the information
/// by iterating through all the blocks. Generally, this doesn't seem like a good option
/// However, we do not force the actual Blockchain to implement a WorldState but rather
/// behave like having one. This trait therefore just defines an expected interface into our Blockchain
/// (Actually it doesn't even care if we the information is stored within a blockchain)
pub trait WorldState {
    /// Will bring us all registered user ids
    fn get_user_ids(&self) -> Vec<String>;

    /// Will return a account given it's id if is available
    fn get_account_by_id(&self, id: &String) -> Option<Arc<RwLock<Account>>>;

    /// Will add a new account
    fn create_account(&mut self, id: String, account_type: AccountType) -> Result<(), &'static str>;
}
