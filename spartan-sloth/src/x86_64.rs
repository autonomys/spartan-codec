//! Optimized implementation of Sloth in x86-64 assembly for 256-bit prime
//! 115792089237316195423570985008687907853269984665640564039457584007913129639747

#[derive(Debug)]
pub struct DataBiggerThanPrime;

#[derive(Debug, Clone)]
pub struct Sloth {}

impl Sloth {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        todo!()
    }

    /// Sequentially encodes a 4096 byte piece
    pub fn encode(
        &self,
        _piece: &mut [u8; 4096],
        _expanded_iv: [u8; 32],
    ) -> Result<(), DataBiggerThanPrime> {
        todo!()
    }

    /// Decodes a 4096 byte encoding in time << encode time
    pub fn decode(&self, _piece: &mut [u8; 4096], _expanded_iv: [u8; 32]) {
        todo!()
    }
}
