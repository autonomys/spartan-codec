/// A pure rust implementation of Sloth with extensions for a proof-of-replication
/// https://eprint.iacr.org/2015/366
/// based on pysloth C implementation by Mathias Michno
/// https://github.com/randomchain/pysloth/blob/master/sloth.c
use rayon::prelude::*;
use rug::ops::NegAssign;
use rug::{integer::IsPrime, integer::Order, ops::BitXorFrom, Integer};
use std::iter;
use std::ops::AddAssign;

/*  ToDo
 * Ensure complies for Windows (Nazar)
 * use a different prime for each block for additional ASIC resistance
 * implement for GPU in CUDA with CGBN
 * implement for GPU in OpenCL with ff-cl-gen
 * ensure correct number of levels are applied for security guarantee
 * should this also take an IV?
 *
 * test: data larger than prime should fail
 * test: hardcode in correct prime and ensure those are generated correctly (once prime is chosen)
*/

/// Finds the next smallest prime number
fn prev_prime(prime: &mut Integer) {
    if prime.is_even() {
        *prime -= 1
    } else {
        *prime -= 2
    }
    while prime.is_probably_prime(25) == IsPrime::No {
        *prime -= 2
    }
}

/// Returns (block, feedback) tuple given block index in a piece
fn piece_to_block_and_feedback(piece: &mut [Integer], index: usize) -> (&mut Integer, &Integer) {
    let (ends_with_feedback, starts_with_block) = piece.split_at_mut(index);
    let feedback = &ends_with_feedback[ends_with_feedback.len() - 1];
    (&mut starts_with_block[0], &feedback)
}

/// Returns (block, feedback) tuple given piece and optional feedback
fn piece_to_first_block_and_feedback(piece: &mut [Integer]) -> (&mut Integer, &Integer) {
    let (first_block, remainder) = piece.split_at_mut(1);
    // At this point last block is already decoded, so we can use it as an IV to previous iteration
    let iv = &remainder[remainder.len() - 1];
    (&mut first_block[0], &iv)
}

/// Converts a 4096 byte piece from an array of GMP big integers back to raw bytes
fn write_integers_to_array(integer_piece: &[Integer], piece: &mut [u8], block_size_bytes: usize) {
    integer_piece
        .iter()
        .flat_map(|integer| {
            let integer_bytes = integer.to_digits::<u8>(Order::Lsf);
            let integer_bytes_len = integer_bytes.len();
            integer_bytes
                .into_iter()
                .chain(iter::repeat(0).take(block_size_bytes - integer_bytes_len))
        })
        .zip(piece.iter_mut())
        .for_each(|(from_byte, to_byte)| {
            *to_byte = from_byte;
        });
}

#[derive(Debug)]
pub struct DataBiggerThanPrime;

pub(super) struct Sloth<const PRIME_SIZE_BYTES: usize, const PIECE_SIZE_BYTES: usize> {
    prime: Integer,
    exponent: Integer,
}

impl<const PRIME_SIZE_BYTES: usize, const PIECE_SIZE_BYTES: usize>
    Sloth<PRIME_SIZE_BYTES, PIECE_SIZE_BYTES>
{
    /// Inits sloth for a given prime size, deterministically deriving the largest prime and computing the exponent
    pub(super) fn new() -> Self {
        let mut prime = Integer::from(Integer::u_pow_u(2, (PRIME_SIZE_BYTES * 8) as u32)) - 1;
        prev_prime(&mut prime);
        while prime.mod_u(4) != 3 {
            prev_prime(&mut prime)
        }

        let mut exponent: Integer = prime.clone() + 1;
        exponent.div_exact_u_mut(4);

        Self { prime, exponent }
    }

    /// Sequentially encodes a 4096 byte piece s.t. a minimum amount of wall clock time elapses
    pub fn encode(
        &self,
        piece: &mut [u8; PIECE_SIZE_BYTES],
        expanded_iv: [u8; PRIME_SIZE_BYTES],
        layers: usize,
    ) -> Result<(), DataBiggerThanPrime> {
        // convert piece to integer representation
        let mut integer_piece: Vec<Integer> = piece
            .chunks_exact(PRIME_SIZE_BYTES)
            .map(|block| Integer::from_digits(&block, Order::Lsf))
            .collect();

        // init feedback as expanded IV
        let mut feedback = Integer::from_digits(&expanded_iv, Order::Lsf);

        // apply the block cipher
        for _ in 0..layers {
            for block in integer_piece.iter_mut() {
                // xor block with feedback
                block.bitxor_from(feedback);

                // apply sqrt permutation
                self.sqrt_permutation(block)?;

                // carry forward the feedback
                feedback = block.clone();
            }
        }

        // transform integers back to bytes
        write_integers_to_array(&integer_piece, piece, PRIME_SIZE_BYTES);

        Ok(())
    }

    /// Sequentially decodes a 4096 byte encoding in time << encode time
    pub(super) fn decode(
        &self,
        piece: &mut [u8; PIECE_SIZE_BYTES],
        expanded_iv: [u8; PRIME_SIZE_BYTES],
        layers: usize,
    ) {
        // convert encoding to integer representation
        let mut integer_piece: Vec<Integer> = piece
            .chunks_exact(PRIME_SIZE_BYTES)
            .map(|block| Integer::from_digits(&block, Order::Lsf))
            .collect();

        for layer in 0..layers {
            for i in (1..(PIECE_SIZE_BYTES / PRIME_SIZE_BYTES)).rev() {
                let (block, feedback) = piece_to_block_and_feedback(&mut integer_piece, i);
                self.inverse_sqrt(block);
                block.bitxor_from(feedback);
            }
            let (block, feedback) = piece_to_first_block_and_feedback(&mut integer_piece);
            self.inverse_sqrt(block);
            if layer != layers - 1 {
                block.bitxor_from(feedback);
            }
        }

        // remove the IV (last round)
        integer_piece[0].bitxor_from(&Integer::from_digits(&expanded_iv, Order::Lsf));

        // transform integers back to bytes
        write_integers_to_array(&integer_piece, piece, PRIME_SIZE_BYTES);
    }

    /// Decodes a 4096 byte encoding in parallel in time << encode time
    pub(super) fn decode_parallel(
        &self,
        piece: &mut [u8; PIECE_SIZE_BYTES],
        expanded_iv: [u8; PRIME_SIZE_BYTES],
        layers: usize,
    ) {
        // convert encoding to integer representation
        let mut integer_piece: Vec<Integer> = piece
            .chunks_exact(PRIME_SIZE_BYTES)
            .map(|block| Integer::from_digits(&block, Order::Lsf))
            .collect();

        for layer in 0..layers {
            let integer_piece_copy = integer_piece.clone();
            integer_piece
                .iter_mut()
                .skip(1)
                .rev()
                .zip(integer_piece_copy.iter().rev().skip(1))
                .par_bridge()
                .for_each(|(block, feedback)| {
                    self.inverse_sqrt(block);
                    block.bitxor_from(feedback);
                });

            let (block, feedback) = piece_to_first_block_and_feedback(&mut integer_piece);
            self.inverse_sqrt(block);
            if layer != layers - 1 {
                block.bitxor_from(feedback);
            }
        }

        // remove the IV (last round)
        integer_piece[0].bitxor_from(&Integer::from_digits(&expanded_iv, Order::Lsf));

        // transform integers back to bytes
        write_integers_to_array(&integer_piece, piece, PRIME_SIZE_BYTES);
    }

    /// Computes the modular square root of data, for data smaller than prime (w.h.p.)
    fn sqrt_permutation(&self, data: &mut Integer) -> Result<(), DataBiggerThanPrime> {
        // better error handling
        if data.as_ref() >= self.prime.as_ref() {
            return Err(DataBiggerThanPrime);
        }

        if data.jacobi(&self.prime) == 1 {
            data.pow_mod_mut(&self.exponent, &self.prime).unwrap();
            if data.is_odd() {
                data.neg_assign();
                data.add_assign(&self.prime);
            }
        } else {
            data.neg_assign();
            data.add_assign(&self.prime);
            data.pow_mod_mut(&self.exponent, &self.prime).unwrap();
            if data.is_even() {
                data.neg_assign();
                data.add_assign(&self.prime);
            }
        }

        Ok(())
    }

    /// Inverts the sqrt permutation with a single squaring mod prime
    fn inverse_sqrt(&self, data: &mut Integer) {
        let is_odd = data.is_odd();
        data.square_mut();
        data.pow_mod_mut(&Integer::from(1), &self.prime).unwrap();
        if is_odd {
            data.neg_assign();
            data.add_assign(&self.prime);
        }
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

    // 256 bits
    #[test]
    fn test_random_piece_256_bits() {
        test_random_piece::<32>();
    }

    // 512 bits
    #[test]
    fn test_random_piece_512_bits() {
        test_random_piece::<64>();
    }

    // 1024 bits
    #[test]
    fn test_random_piece_1024_bits() {
        test_random_piece::<128>();
    }

    // 2048 bits
    #[test]
    fn test_random_piece_2048_bits() {
        test_random_piece::<256>();
    }

    // 4096 bits
    #[test]
    fn test_random_piece_4096_bits() {
        test_random_piece::<512>();
    }

    fn test_random_piece<const PRIME_SIZE_BYTES: usize>() {
        let expanded_iv = random_bytes();
        let piece = random_bytes();

        let sloth = Sloth::<PRIME_SIZE_BYTES, 4096>::new();
        let layers = 4096 / PRIME_SIZE_BYTES;
        let mut encoding = piece.clone();
        sloth.encode(&mut encoding, expanded_iv, layers).unwrap();
        let mut decoding = encoding.clone();
        sloth.decode(&mut decoding, expanded_iv, layers);

        // println!("\nPiece is {:?}\n", piece.to_vec());
        // println!("\nDecoding is {:?}\n", decoding.to_vec());
        // println!("\nEncoding is {:?}\n", encoding.to_vec());

        assert_eq!(piece.to_vec(), decoding.to_vec());

        let mut decoding = encoding.clone();
        sloth.decode_parallel(&mut decoding, expanded_iv, layers);

        assert_eq!(piece.to_vec(), decoding.to_vec());
    }
}
