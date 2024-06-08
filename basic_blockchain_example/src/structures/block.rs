use blake2::{Blake2b512, Digest};
use super::byte_vector_to_string;
use crate::structures::transaction::Transaction;

/// One single part of the blockchain.
/// Basically contains a list of transactions
#[derive(Clone, Debug)]
pub struct Block {
    /// Actions that this block includes
    /// There has to be at least one
    pub transactions: Vec<Transaction>,

    /// This actually connects the blocks together
    pub prev_hash: Option<String>,

    /// We store the hash of the block here also in order to
    /// save the last block from being tampered with later on
    pub hash: Option<String>,

    /// Some arbitrary number which will be later used for Proof of Work
    pub nonce: u128,
}

impl Block {
    pub fn new(prev_hash: Option<String>) -> Self {
        Block {
            nonce: 0,
            hash: None,
            prev_hash,
            transactions: Vec::new(),
        }
    }

    /// Changes the nonce number and updates the hash
    pub fn set_nonce(&mut self, nonce: u128) {
        self.nonce = nonce;
        self.update_hash();
    }

    /// Will calculate the hash of the whole block including transactions Blake2 hasher
    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b512::new();

        for transaction in self.transactions.iter() {
            hasher.update(transaction.calculate_hash())
        }

        let block_as_string = format!("{:?}", (&self.prev_hash, &self.nonce));
        hasher.update(&block_as_string);

        let mut result = Vec::new();

        let finalized_hash = hasher.finalize();

        for b in &finalized_hash[..] {
            result.push(*b);
        }
        result
    }

    /// Appends a transaction to the queue
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
        self.update_hash();
    }

    /// Will return the amount of transactions
    pub fn get_transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Will update the hash field by including all transactions currently inside
    /// the public modifier is only for the demonstration of attacks
    pub fn update_hash(&mut self) {
        self.hash = Some(byte_vector_to_string(&self.calculate_hash()));
    }

    /// Checks if the hash is set and matches the blocks interna
    pub fn verify_own_hash(&self) -> bool {
        self.hash.is_some() && // Hash set
            self.hash.as_ref().unwrap().eq(&byte_vector_to_string(&self.calculate_hash()))
    }
}
