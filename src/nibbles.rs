
pub fn get_first_nibble(value: u8) -> u8 {
    const FIRST_NIBBLE_BIT_MASK: u8 = 0xF0;
    (value & FIRST_NIBBLE_BIT_MASK) >> 4
}
pub fn get_second_nibble(value: u8) -> u8 {
    const SECOND_NIBBLE_BIT_MASK: u8 = 0x0F;
    value & SECOND_NIBBLE_BIT_MASK
}
/// convert three nibbles to a single [u16] with the most significant nibble being 0.
/// 
/// `value[0]` is the most significant nibble. `value[2]` least significant nibble. (big-endian)
pub fn combine_three_nibbles(value: [u8; 3]) -> u16 {
    ((value[0] as u16) << 8) | ((value[1] as u16) << 4) | (value[2] as u16)
}

#[test]
fn test_get_first_nibble() {
    let byte = 0xAB;
    let first_nibble = get_first_nibble(byte);
    let expected = 0x0A;
    assert_eq!(first_nibble, expected);
}
#[test]
fn test_get_second_nibble() {
    let byte = 0xAB;
    let second_nibble = get_second_nibble(byte);
    let expected = 0x0B;
    assert_eq!(second_nibble, expected);
}
#[test]
fn test_combine_three_nibbles() {
    let nibbles = [0xA, 0xB, 0xC];
    let combined = combine_three_nibbles(nibbles);
    let expected = 0x0ABC;
    assert_eq!(combined, expected);
}