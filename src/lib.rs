#![forbid(unsafe_code)]
#![allow(unused)]

pub mod interpreter;

fn get_first_nibble(value: u8) -> u8 {
    const FIRST_NIBBLE_BIT_MASK: u8 = 0xF0;
    (value & FIRST_NIBBLE_BIT_MASK) >> 4
}
fn get_second_nibble(value: u8) -> u8 {
    const SECOND_NIBBLE_BIT_MASK: u8 = 0x0F;
    value & SECOND_NIBBLE_BIT_MASK
}
