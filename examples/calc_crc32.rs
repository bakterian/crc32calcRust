extern crate crc32_calculator;

use crc32_calculator::{PolynomialType, compute_crc32};

fn main()
{
    // decimal 123456789 as byte stream MSB first:
    let input_data: Vec<u8> = vec![0x01, 0x01, 0x01, 0x01];

    let crc32 = compute_crc32(&input_data, &PolynomialType::Crc32Normal);

    println!("crc32: {:#08x}", crc32.unwrap());
}