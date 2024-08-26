
/// # Example
/// ```
/// let byte = 0xAB;
/// let first_nibble = chip8::nibbles::get_first_nibble(byte);
/// assert_eq!(first_nibble, 0x0A);
/// ```
pub fn get_first_nibble(value: u8) -> u8 {
    const FIRST_NIBBLE_BIT_MASK: u8 = 0xF0;
    (value & FIRST_NIBBLE_BIT_MASK) >> 4
}
/// # Example
/// ```
/// let byte = 0xAB;
/// let second_nibble = chip8::nibbles::get_second_nibble(byte);
/// assert_eq!(second_nibble, 0x0B);
/// ```
pub fn get_second_nibble(value: u8) -> u8 {
    const SECOND_NIBBLE_BIT_MASK: u8 = 0x0F;
    value & SECOND_NIBBLE_BIT_MASK
}
/// convert three nibbles to a single [u16] with the left most nibble being 0.
/// 
/// `value[0]` is the most significant nibble. `value[2]` least significant nibble. (big-endian)
/// 
/// # Example
/// ```
/// let nibbles = [0xA, 0xB, 0xC];
/// let combined = chip8::nibbles::combine_three_nibbles(nibbles[0], nibbles[1], nibbles[2]);
/// assert_eq!(combined, 0x0ABC);
/// ```
pub fn combine_three_nibbles(first: u8, second: u8, third: u8) -> u16 {
    ((first as u16) << 8) | ((second as u16) << 4) | (third as u16)
}
/// # Example 
/// ```
/// let nibbles = [0xA, 0xB];
/// let combined = chip8::nibbles::combine_two_nibbles(nibbles[0], nibbles[1]);
/// assert_eq!(combined, 0xAB);
/// ```
pub fn combine_two_nibbles(first: u8, second: u8) -> u8 {
    (first << 4) | second
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
    let combined = combine_three_nibbles(nibbles[0], nibbles[1], nibbles[2]);
    let expected = 0x0ABC;
    assert_eq!(combined, expected);
}
#[test]
fn test_combine_two_nibbles() {
    let nibbles = [0xA, 0xB];
    let combined = combine_two_nibbles(nibbles[0], nibbles[1]);
    let expected = 0xAB;
    assert_eq!(combined, expected);
}