pub enum PolynomialType 
{
    Crc32Normal,
    Crc32Reversed,
    Crc32Reciprocal,
    Crc32ReversedReciprocal,
    Crc32CNormal,
    Crc32CReversed,
    Crc32CReciprocal,
    Crc32CReversedReciprocal,
    Crc32KNormal,
    Crc32KReversed,
    Crc32KReciprocal,
    Crc32KReversedReciprocal,
    Crc32K2Normal,
    Crc32K2Reversed,
    Crc32K2Reciprocal,
    Crc32K2ReversedReciprocal,
    Crc32QNormal,
    Crc32QReversed,
    Crc32QReciprocal,
    Crc32QReversedReciprocal,
}

pub struct PolynomialStore 
{
}

impl PolynomialStore 
{
    // The polynomials actually take 33 bits but the most significant bit of a polynomial is always 1
    // and is not shown in the hex representations.
    // i.e Crc32Normal x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1
    // the "1" bit for the x^32 is omitted
    // Why is done like this?
    // This done to have an easier polynomial divding on the 8-bit boundry
    // First, it is always 1. Second, because the divisor is always aligned in such a manner that this leading '1' 
    // alignes with the next '1' of the dividend, the XOR result for this bit is always 0.
    pub fn to_number(poly: &PolynomialType) -> u32
    {
        match poly {
            PolynomialType::Crc32Normal => 0x04C11DB7_u32,
            PolynomialType::Crc32Reversed => 0xEDB88320_u32,
            PolynomialType::Crc32Reciprocal => 0xDB710641_u32,
            PolynomialType::Crc32ReversedReciprocal => 0x82608EDB_u32,
            PolynomialType::Crc32CNormal => 0x1EDC6F41_u32,
            PolynomialType::Crc32CReversed => 0x82F63B78_u32,
            PolynomialType::Crc32CReciprocal => 0x05EC76F1_u32,
            PolynomialType::Crc32CReversedReciprocal => 0x8F6E37A0_u32,
            PolynomialType::Crc32KNormal => 0x741B8CD7_u32,
            PolynomialType::Crc32KReversed => 0xEB31D82E_u32,
            PolynomialType::Crc32KReciprocal => 0xD663B05D_u32,
            PolynomialType::Crc32KReversedReciprocal => 0xBA0DC66B_u32,
            PolynomialType::Crc32K2Normal => 0x32583499_u32,
            PolynomialType::Crc32K2Reversed => 0x992C1A4C_u32,
            PolynomialType::Crc32K2Reciprocal => 0x32583499_u32,
            PolynomialType::Crc32K2ReversedReciprocal => 0x992C1A4C_u32,
            PolynomialType::Crc32QNormal => 0x814141AB_u32,
            PolynomialType::Crc32QReversed => 0xD5828281_u32,
            PolynomialType::Crc32QReciprocal => 0xAB050503_u32,
            PolynomialType::Crc32QReversedReciprocal => 0xC0A0A0D5_u32,
        }
    }
}
