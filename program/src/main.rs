//! A program that hashes the input 10 times using SHA-256.

#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};

fn sha2(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

pub fn main() {
    let mut input: [u8; 32] = [5u8; 32];

    // Hash the input 10 times
    for _ in 0..10 {
        input = sha2(&input);
    }

    let output_hex = hex::encode(input);

    sp1_zkvm::io::commit(&output_hex);
}
