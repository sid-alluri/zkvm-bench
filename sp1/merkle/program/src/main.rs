#![no_main]
sp1_zkvm::entrypoint!(main);
use alloy_sol_types::SolType;
use fibonacci_lib::{fibonacci, PublicValuesStruct};
use sha3::{Digest, Keccak256};

pub fn main() {
    let leaf = sp1_zkvm::io::read::<Vec<u8>>();
    let path = sp1_zkvm::io::read::<Vec<Vec<u8>>>();
    let root = sp1_zkvm::io::read::<Vec<u8>>();

    let mut hasher = Keccak256::new();
    let concat_first = concat(&leaf, &path[0]);
    hasher.update(&concat_first);

    let mut buf = hasher.finalize_reset().to_vec();
    // Hash the rest of the path
    for path_element in path.iter().skip(1) {
        let concat_next = concat(&buf, path_element);
        hasher.update(&concat_next);
        buf = hasher.finalize_reset().to_vec();
    }

    // Assert equality with root
    let mut result = 1;
    if buf != root {
        result = 0;
    }

    // write public output to the journal
    sp1_zkvm::io::commit(&result);
}

// Helper function to concatenate two byte vectors
pub fn concat(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut c = Vec::with_capacity(a.len() + b.len());
    c.extend_from_slice(a);
    c.extend_from_slice(b);
    c
}

// Hash a single element
pub fn hash_elem(input: u8) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(&[input]);
    hasher.finalize().to_vec()
}
