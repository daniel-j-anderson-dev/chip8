#![forbid(unsafe_code)]
#![allow(unused)]

pub struct Chip8 {
    memory: [u8; 4096],

    /// Index to the current byte in memory.
    program_counter: u16,

    /// Often called `I`.
    /// Also called memory index register.
    address_register: u16,

    /// General purpose registers often called `VX` where X is the index.
    /// The last byte (`VF`) is also used as a flag for carries or other purposes
    variable_register: [u8; 16],

    /// Keeps track of return memory locations when a subroutine is called
    call_stack: [u16; 16],

    /// Keeps track of which entry in the stack should be returned to.
    /// Determines current position in stack.
    call_stack_index: usize,

    /// Decrements at 60hz until zero
    delay_timer: u8,

    /// Decrements at 60hz until zero when a sound is played
    sound_timer: u8,

    /// `false` represents a black pixel. `true` represents a white pixel
    display: [[bool; 64]; 32],
}

impl Chip8 {
    /// Offset is commonly done because of old standards.
    /// Most programs written for Chip8 expect programs to start here.
    pub const PROGRAM_MEMORY_OFFSET: u16 = 200;

    pub fn new() -> Chip8 {
        Self {
            memory: [0; 4096],
            program_counter: Self::PROGRAM_MEMORY_OFFSET,
            address_register: 0,
            variable_register: [0; 16],
            call_stack: [0; 16],
            call_stack_index: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [[false; 64]; 32],
        }
    }

    /// Returns an array contain the four nibbles of an opcode.
    /// (a nibble is a four bit number or single hexadecimal digit)
    pub fn get_current_opcode(&self) -> [u8; 4] {
        unimplemented!();
    }
}
