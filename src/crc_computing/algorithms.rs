
const CRC_WIDTH: u32 = 32;
const CRC_TOPBIT: u32 = 1 << (CRC_WIDTH -1);

pub fn bit_by_bit_compute_crc32(dividend: &Vec<u8>, dividor: u32 ) -> Option<u32>
{
    let mut remainder = 0_u32;
    println!("dividor {:#08x}", dividor);

    // modulo division per byte
    for byte_dividend in dividend 
    {
        //doing two things: 1) using the next byte 2) XOR with the previous remainder content
        println!("remainder bef {:#08x}", remainder);
        remainder ^= (*byte_dividend as u32) << (CRC_WIDTH - 8);
        println!("byte_dividend: {:#08x}", byte_dividend);
        println!("remainder after {:#08x}", remainder);

        for _ in 0..8_u8
        {
            // try to divide current data bit
            if(remainder & CRC_TOPBIT) > 0
            {
                println!("one shift");
                remainder <<= 1;
                remainder ^= dividor;
            }
            else 
            {
                println!("zero shift");
                remainder <<= 1;
            }
        }
    }

    Some(remainder)
}
