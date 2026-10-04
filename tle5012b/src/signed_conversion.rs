use bilge::prelude::*;
// 1100 1101 1001 0011
pub fn to_signed<const NUMBIT: usize>(raw: u15) -> i32 {
    debug_assert!(NUMBIT > 0 && NUMBIT <= 15);
    let mut value: i32 = 0;
    let msb_bit = (u16::from(raw >> (NUMBIT - 1)) & 1) as i32;

    let msb_bit_val = -msb_bit * (2i32.pow((NUMBIT - 1) as u32));
    value += msb_bit_val;

    (0..NUMBIT - 2).for_each(|bit| {
        let bit_val = (u16::from(raw >> bit) & 1) as i32;
        let pow_val = 2i32.pow(bit as u32) * bit_val;
        value += pow_val;
    });

    value
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_to_signed() {
        // value choose based on tle5012b manual on page 70
        let byte = u15::new(0b0100_1101_1001_0011);
        let signed_val = to_signed::<15>(byte);

        assert_eq!(signed_val, -12909);
    }
}
