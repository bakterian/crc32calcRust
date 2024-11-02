mod algorithms;
mod crc32_wikipedia;

use algorithms::bit_by_bit_compute_crc32;
use crc32_wikipedia::PolynomialStore;

// library public data:
pub use crc32_wikipedia::PolynomialType;

pub fn compute_crc32(input: &Vec<u8>, generator: &PolynomialType ) -> Option<u32>
{
    let dividor = PolynomialStore::to_number(generator);
    bit_by_bit_compute_crc32(input, dividor)
}
