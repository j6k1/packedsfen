pub mod traits;
pub mod error;
pub mod yaneuraou;
pub mod hcpe;

use error::*;

pub struct BitStreamReader<'a> {
    buf:&'a [u8],
    current_index:usize,
    current_bits:usize
}
impl<'a> BitStreamReader<'a> {
    pub fn new(buf:&'a [u8]) -> BitStreamReader<'a> {
        BitStreamReader {
            buf:buf,
            current_index:0,
            current_bits:0
        }
    }

    pub fn get_bit_from_lsb(&mut self) -> Result<u8,ReadError> {
        if self.current_index >= self.buf.len() {
            Err(ReadError::InvalidState(String::from("End of the input has been exceeded.")))
        } else {
            let bit = if self.buf[self.current_index] & (1u8 << self.current_bits as u8) == 0 {
                0
            } else {
                1
            };

            self.current_bits += 1;

            if self.current_bits >= 8 {
                self.current_index += 1;
                self.current_bits = 0;
            }

            Ok(bit)
        }
    }

    pub fn get_bits_from_lsb(&mut self, size:usize) -> Result<u8,ReadError> {
        let mut bits = 0;

        for i in 0..size {
            bits |= self.get_bit_from_lsb()? << i;
        }
        Ok(bits)
    }

    pub fn get_cursor(&self) -> usize {
        self.current_index * 8 + self.current_bits
    }
}