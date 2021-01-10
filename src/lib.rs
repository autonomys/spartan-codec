mod sloth;

pub const PRIME_SIZE_BYTES: usize = 256 / 8;
pub const PIECE_SIZE: usize = 4096;
pub const IV_SIZE: usize = 32;
pub type Piece = [u8; PIECE_SIZE];
pub type ExpandedIV = [u8; PRIME_SIZE_BYTES];
pub type IV = [u8; IV_SIZE];

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
