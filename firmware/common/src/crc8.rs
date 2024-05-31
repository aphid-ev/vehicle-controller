
/// `const` function to generate a CRC8 lookuptable at compile time for a given polynomial.
/// 
/// # Example
/// 
/// ```
/// use common::crc8::generate_lookup;
/// const CRC8_LOOKUP: [u8; 256] = generate_lookup(0x07);
/// ```
pub const fn generate_lookup(polynomial: u8) -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        let mut value = i as u8;
        let mut bit = 0;
        while bit < 8 {
            if (value & 0x80) != 0 {
                value <<= 1;
                value ^= polynomial;
            } else {
                value <<= 1;
            }
            bit += 1;
        }

        table[i] = value;
        i += 1;
    }

    table
}

/// Calculates the CRC8 of a byte slice using a pre-calculated lookup table.
/// 
/// # Example
/// ```
/// use common::crc8::{generate_lookup, calc_crc8};
/// 
/// const CRC8_LOOKUP: [u8; 256] = generate_lookup(0x07);
/// let crc = calc_crc8(&[0x00, 0x11, 0x22], &CRC8_LOOKUP);
/// assert_eq!(crc, 0xac);
/// ```
pub fn calc_crc8(bytes: &[u8], lookup: &[u8; 256]) -> u8 {
    let mut crc  = 0;

    for &byte in bytes {
        crc = lookup[(crc ^ byte) as usize];
    }

    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup() {
        let crc8_lookup = generate_lookup(0x07);

        assert_eq!(crc8_lookup[0], 0x00);
        assert_eq!(crc8_lookup[8], 0x38);
        assert_eq!(crc8_lookup[16], 0x70);
        assert_eq!(crc8_lookup[255], 0xf3);
    }

    #[test]
    fn crc8() {
        let data = [0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39];
        let crc8_lookup = generate_lookup(0x07);

        let crc = calc_crc8(&data, &crc8_lookup);

        assert_eq!(crc, 0xf4);
    }
}