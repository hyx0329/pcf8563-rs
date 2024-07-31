//! convenient toolbox to convert values

/// Convert raw decimal value to corresponding Binary Coded Decimal value.
///
/// Only supports u8
#[inline]
pub fn u8_bcd_encode(value: u8) -> u8 {
    value % 10 + (value / 10 << 4)
}

/// Convert Binary Coded Decimal value to original value.
///
/// Only supports u8
#[inline]
pub fn u8_bcd_decode(value: u8) -> u8 {
    (value & 0xf) + ((value >> 4) * 10)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bcd_encode() {
        assert_eq!(0b10010111, u8_bcd_encode(97));
        assert_eq!(0b01110100, u8_bcd_encode(74));
    }

    #[test]
    fn test_bcd_decode() {
        assert_eq!(97, u8_bcd_decode(0b10010111));
        assert_eq!(74, u8_bcd_decode(0b01110100));
    }
}
