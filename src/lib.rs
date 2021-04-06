#![warn(rust_2018_idioms, missing_debug_implementations, missing_docs)]
//! This is an adaptation of [SLOTH](https://eprint.iacr.org/2015/366) (slow-timed hash function) into a time-asymmetric permutation using a standard CBC block cipher. This code is largely based on the C implementation used in [PySloth](https://github.com/randomchain/pysloth/blob/master/sloth.c) which is the same as used in the paper.

mod sloth;

use crate::sloth::Sloth;

/// Spartan struct used to encode and validate
#[derive(Debug, Clone)]
pub struct Spartan<const PRIME_SIZE_BYTES: usize, const PIECE_SIZE_BYTES: usize> {
    genesis_piece: [u8; PIECE_SIZE_BYTES],
    sloth: Sloth<PRIME_SIZE_BYTES, PIECE_SIZE_BYTES>,
}

/// Spartan configured for 64-bit prime and 4096-byte genesis piece size
pub type Spartan64bit4096 = Spartan<8, 4096>;
/// Spartan configured for 128-bit prime and 4096-byte genesis piece size
pub type Spartan128bit4096 = Spartan<16, 4096>;
/// Spartan configured for 256-bit prime and 4096-byte genesis piece size
pub type Spartan256bit4096 = Spartan<32, 4096>;
/// Spartan configured for 512-bit prime and 4096-byte genesis piece size
pub type Spartan512bit4096 = Spartan<64, 4096>;
/// Spartan configured for 1024-bit prime and 4096-byte genesis piece size
pub type Spartan1024bit4096 = Spartan<128, 4096>;
/// Spartan configured for 2048-bit prime and 4096-byte genesis piece size
pub type Spartan2048bit4096 = Spartan<256, 4096>;
/// Spartan configured for 4096-bit prime and 4096-byte genesis piece size
pub type Spartan4096bit4096 = Spartan<512, 4096>;

impl<const PRIME_SIZE_BYTES: usize, const PIECE_SIZE_BYTES: usize>
    Spartan<PRIME_SIZE_BYTES, PIECE_SIZE_BYTES>
{
    fn new_internal(genesis_piece: [u8; PIECE_SIZE_BYTES]) -> Self {
        let sloth = Sloth::new();
        Self {
            genesis_piece,
            sloth,
        }
    }
}

impl Spartan<8, 4096> {
    /// New instance with 64-bit prime and 4096-byte genesis piece size
    pub fn new(genesis_piece: [u8; 4096]) -> Self {
        Self::new_internal(genesis_piece)
    }
}

impl Spartan<16, 4096> {
    /// New instance with 128-bit prime and 4096-byte genesis piece size
    pub fn new(genesis_piece: [u8; 4096]) -> Self {
        Self::new_internal(genesis_piece)
    }
}

impl Spartan<32, 4096> {
    /// New instance with 256-bit prime and 4096-byte genesis piece size
    pub fn new(genesis_piece: [u8; 4096]) -> Self {
        Self::new_internal(genesis_piece)
    }
}

impl Spartan<64, 4096> {
    /// New instance with 512-bit prime and 4096-byte genesis piece size
    pub fn new(genesis_piece: [u8; 4096]) -> Self {
        Self::new_internal(genesis_piece)
    }
}

impl Spartan<128, 4096> {
    /// New instance with 1024-bit prime and 4096-byte genesis piece size
    pub fn new(genesis_piece: [u8; 4096]) -> Self {
        Self::new_internal(genesis_piece)
    }
}

impl Spartan<256, 4096> {
    /// New instance with 2048-bit prime and 4096-byte genesis piece size
    pub fn new(genesis_piece: [u8; 4096]) -> Self {
        Self::new_internal(genesis_piece)
    }
}

impl Spartan<512, 4096> {
    /// New instance with 4096-bit prime and 4096-byte genesis piece size
    pub fn new(genesis_piece: [u8; 4096]) -> Self {
        Self::new_internal(genesis_piece)
    }
}

impl<const PRIME_SIZE_BYTES: usize, const PIECE_SIZE_BYTES: usize>
    Spartan<PRIME_SIZE_BYTES, PIECE_SIZE_BYTES>
{
    /// Create an encoding based on genesis piece using provided encoding key hash, nonce and
    /// desired number of rounds
    pub fn encode(
        &self,
        encoding_key_hash: [u8; PRIME_SIZE_BYTES],
        nonce: u64,
        rounds: usize,
    ) -> [u8; PIECE_SIZE_BYTES] {
        let mut expanded_iv = encoding_key_hash;
        for (i, &byte) in nonce.to_le_bytes().iter().rev().enumerate() {
            expanded_iv[PRIME_SIZE_BYTES - i - 1] ^= byte;
        }

        let mut encoding = self.genesis_piece;
        // TODO: Better error handling
        self.sloth
            .encode(&mut encoding, expanded_iv, rounds)
            .unwrap();

        encoding
    }

    /// Check if previously created encoding is valid
    pub fn is_valid(
        &self,
        mut encoding: [u8; PIECE_SIZE_BYTES],
        encoding_key_hash: [u8; PRIME_SIZE_BYTES],
        nonce: u64,
        rounds: usize,
    ) -> bool {
        let mut expanded_iv = encoding_key_hash;
        for (i, &byte) in nonce.to_le_bytes().iter().rev().enumerate() {
            expanded_iv[PRIME_SIZE_BYTES - i - 1] ^= byte;
        }

        self.sloth.decode(&mut encoding, expanded_iv, rounds);

        encoding == self.genesis_piece
    }

    /// Check if previously created encoding is valid by leveraging parallelism
    #[cfg(feature = "parallel")]
    pub fn is_valid_parallel(
        &self,
        piece: [u8; PIECE_SIZE_BYTES],
        encoding_key_hash: [u8; PRIME_SIZE_BYTES],
        nonce: u64,
        rounds: usize,
    ) -> bool {
        let mut piece = piece;
        let mut expanded_iv = encoding_key_hash;
        for (i, &byte) in nonce.to_le_bytes().iter().rev().enumerate() {
            expanded_iv[PRIME_SIZE_BYTES - i - 1] ^= byte;
        }

        self.sloth.decode_parallel(&mut piece, expanded_iv, rounds);

        piece == self.genesis_piece
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    fn random_bytes<const BYTES: usize>() -> [u8; BYTES] {
        let mut bytes = [0u8; BYTES];
        rand::thread_rng().fill(&mut bytes[..]);
        bytes
    }

    #[test]
    fn test_random_piece() {
        let genesis_piece = random_bytes();
        let encoding_key = random_bytes();
        let nonce = rand::random();

        let spartan = Spartan256bit4096::new(genesis_piece);
        let encoding = spartan.encode(encoding_key, nonce, 1);

        assert!(spartan.is_valid(encoding, encoding_key, nonce, 1));

        assert!(spartan.is_valid_parallel(encoding, encoding_key, nonce, 1));
    }
}
