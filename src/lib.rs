mod sloth;

use crate::sloth::Sloth;

pub struct Spartan<const PRIME_SIZE_BYTES: usize, const PIECE_SIZE_BYTES: usize> {
    genesis_piece: [u8; PIECE_SIZE_BYTES],
    sloth: Sloth<PRIME_SIZE_BYTES, PIECE_SIZE_BYTES>,
}

impl Spartan<8, 4096> {
    pub fn new(genesis_piece: [u8; 4096]) -> Self {
        let sloth = Sloth::new();
        Self {
            genesis_piece,
            sloth,
        }
    }
}

impl Spartan<32, 4096> {
    pub fn new(genesis_piece: [u8; 4096]) -> Self {
        let sloth = Sloth::new();
        Self {
            genesis_piece,
            sloth,
        }
    }
}

impl<const PRIME_SIZE_BYTES: usize, const PIECE_SIZE_BYTES: usize>
    Spartan<PRIME_SIZE_BYTES, PIECE_SIZE_BYTES>
{
    pub fn encode(
        &self,
        encoding_key_hash: [u8; PRIME_SIZE_BYTES],
        nonce: u32,
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

    pub fn is_valid(
        &self,
        piece: [u8; PIECE_SIZE_BYTES],
        encoding_key_hash: [u8; PRIME_SIZE_BYTES],
        nonce: u32,
        rounds: usize,
    ) -> bool {
        let mut piece = piece;
        let mut expanded_iv = encoding_key_hash;
        for (i, &byte) in nonce.to_le_bytes().iter().rev().enumerate() {
            expanded_iv[PRIME_SIZE_BYTES - i - 1] ^= byte;
        }

        self.sloth.decode(&mut piece, expanded_iv, rounds);

        piece == self.genesis_piece
    }

    #[cfg(feature = "parallel")]
    pub fn is_valid_parallel(
        &self,
        piece: [u8; PIECE_SIZE_BYTES],
        encoding_key_hash: [u8; PRIME_SIZE_BYTES],
        nonce: u32,
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

        let spartan = Spartan::<32, 4096>::new(genesis_piece);
        let encoding = spartan.encode(encoding_key, nonce, 1);

        assert!(spartan.is_valid(encoding, encoding_key, nonce, 1));

        assert!(spartan.is_valid_parallel(encoding, encoding_key, nonce, 1));
    }
}
