use std::time::SystemTime;

use blake2::{Blake2b512, Digest};

use crate::structures::account::AccountType;
use crate::structures::world_state::WorldState;

/// Stores a request to the blockchain
#[derive(Clone, Debug)]
pub struct Transaction {
    /// Unique number (will be used for randomization later; prevents replay attacks)
    nonce: u128,

    /// Account ID
    from: String,

    /// Stores the time the transaction was created
    created_at: SystemTime,

    /// the type of the transaction and its additional information
    pub record: TransactionData,

    /// Signature of the hash of the whole message
    signature: Option<String>,
}

impl Transaction {
    pub fn new(from: String, transaction_data: TransactionData, nonce: u128) -> Self {
        Transaction {
            from,
            nonce,
            record: transaction_data,
            created_at: SystemTime::now(),
            signature: None,
        }
    }

    /// Will change the world state according to the transactions commands
    pub fn execute<T: WorldState>(&self, world_state: &mut T, is_initial: &bool) -> Result<(), &'static str> {
        // Check if sending user does exist (no one not on the chain can execute transactions)
        if let Some(_account) = world_state.get_account_by_id(&self.from) {
            // Do some more checkups later on...
        } else {
            if !is_initial {
                return Err("Account does not exist (Code: 93482390)");
            }
        }

        // match is like a switch (pattern matching) in C++ or Java
        // We will check for the type of transaction here and execute its logic
        return match &self.record {
            TransactionData::CreateUserAccount(account) => {
                world_state.create_account(account.into(), AccountType::User)
            }

            TransactionData::CreateTokens { receiver, amount } => {
                if !is_initial {
                    return Err("Token creation is only available on initial creation (Code: 2394233)");
                }
                // Get the receiving user (must exist)
                return if let Some(account) = world_state.get_account_by_id(receiver) {
                    account.write().unwrap().tokens += *amount;
                    Ok(())
                } else {
                    Err("Receiver Account does not exist (Code: 23482309)")
                };
            }

            TransactionData::TransferTokens { to, amount } => {
                let recv_tokens: u128;
                let sender_tokens: u128;

                if let Some(recv) = world_state.get_account_by_id(to) {
                    // Be extra careful here, even in the genesis block the sender account has to exist
                    recv_tokens = recv.read().unwrap().tokens;
                } else {
                    return Err("Receiver Account does not exist! (Code: 3242342380)");
                }

                if let Some(sender) = world_state.get_account_by_id(&self.from) {
                    sender_tokens = sender.read().unwrap().tokens;
                } else {
                    return Err("That account does not exist! (Code: 23423923)");
                }

                let balance_recv_new = recv_tokens.checked_add(*amount);
                let balance_sender_new = sender_tokens.checked_sub(*amount);

                return if balance_recv_new.is_some() && balance_sender_new.is_some() {
                    world_state.get_account_by_id(&self.from).unwrap().write().unwrap().tokens = balance_sender_new.unwrap();
                    world_state.get_account_by_id(to).unwrap().write().unwrap().tokens = balance_recv_new.unwrap();
                    Ok(())
                } else {
                    Err("Overspent or Arithmetic error (Code: 48239084203)")
                };
            }

            _ => { // Not implemented transaction type
                Err("Unknown Transaction type (not implemented) (Code: 487289724389)")
            }
        };
    }

    /// Will calculate the hash using Blake2 hasher
    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b512::new();
        let transaction_as_string = format!("{:?}", (&self.created_at, &self.record,
                                                     &self.from, &self.nonce));

        hasher.update(&transaction_as_string);

        let mut result = Vec::new();

        let finalized_hash = hasher.finalize();

        for b in &finalized_hash[..] {
            result.push(*b);
        }
        result
    }

    /// Will hash the transaction and check if the signature is valid
    /// (i.e., it is created by the owners private key)
    /// if the message is not signed it will always return false
    pub fn check_signature(&self) -> bool {
        if !(self.is_signed()) {
            return false;
        }
        false
    }

    pub fn is_signed(&self) -> bool {
        self.signature.is_some()
    }
}

/// A single operation to be stored on the chain
/// Noticeable, enums in rust actually can carry data in a
/// tuple-like structure (CreateUserAccount) or a dictionary-like (the ChangeStoreValue)
#[derive(Clone, Debug, PartialEq)]
pub enum TransactionData {
    /// Will be used to store a new user account
    CreateUserAccount(String),

    /// Will be used to change or create a arbitrary value into an account
    ChangeStoreValue { key: String, value: String },

    /// Will be used to move tokens from one owner to another
    TransferTokens { to: String, amount: u128 },

    /// Just create tokens out of nowhere
    CreateTokens { receiver: String, amount: u128 },
}
