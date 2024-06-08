pub mod blockchain;
pub mod world_state;
pub mod transaction;
pub mod account;
pub mod block;

/// Will take an array of bytes and transform it into a string by interpreting every byte
/// as an character
pub fn byte_vector_to_string(arr: &Vec<u8>) -> String {
    arr.iter().map(|&c| c as char).collect()
}
