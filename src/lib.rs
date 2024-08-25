#![forbid(unsafe_code)]
#![allow(unused)]

pub struct Chip8 {
    memory: [u8; 4096],
    /// Index to the current byte in memory
    program_counter: u16,
    /// Often called `I`
    address_register: u16,
    /// General purpose registers often called `VX` where X is the index.
    /// The last byte (`VF`) is also used as a flag for carries or other purposes
    variable_register: [u8; 16],
    /// `false` represents a black pixel. `true` represents a white pixel
    display: [[bool; 64]; 32],
    /// Keeps track of where in memory to return to
    call_stack: [u16; 16],
    /// Decrements at 60hz until zero
    delay_timer: u8,
    /// Decrements at 60hz until zero when a sound is played
    sound_timer: u8,
}