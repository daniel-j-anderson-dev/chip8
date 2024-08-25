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

    /// `true` represents a pressed button. `false` represents a unpressed button
    /// ```
    ///   0   1   2   3
    /// ╔═══╦═══╦═══╦═══╗
    /// ║ 1 ║ 2 ║ 3 ║ C ║ 0
    /// ╠═══╬═══╬═══╬═══╣
    /// ║ 4 ║ 5 ║ 6 ║ D ║ 1
    /// ╠═══╬═══╬═══╬═══╣
    /// ║ 7 ║ 8 ║ 9 ║ E ║ 2
    /// ╠═══╬═══╬═══╬═══╣
    /// ║ A ║ 0 ║ B ║ F ║ 3
    /// ╚═══╩═══╩═══╩═══╝
    /// ```
    keypad: [[bool; 4]; 4],
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
            keypad: [[false; 4]; 4],
        }
    }

    /// Returns an array contain the four nibbles of an opcode.
    /// (a nibble is a four bit number or single hexadecimal digit)
    /// 
    /// TODO: Add bounds checking
    fn get_current_instruction(&self) -> [u8; 4] {
        let program_counter = self.program_counter as usize;

        let most_significant_byte = self.memory[program_counter];
        let least_significant_byte = self.memory[program_counter + 1];

        [
            get_first_nibble(most_significant_byte),
            get_second_nibble(most_significant_byte),
            get_first_nibble(least_significant_byte),
            get_second_nibble(least_significant_byte),
        ]
    }

    fn execute_current_instruction(&mut self) {
        let current_instruction = self.get_current_instruction();

        match current_instruction {
            _ => {}
        }

        unimplemented!();
    }
    
    /*
        All the steps of execution in chip8, if it is anything like modern processors:
            fetch
                get the memory[program_counter]
                get the memory[program_counter + 1]
                bitmagic to get 4 nibbles of instruction
            decode
                literally a gauntlet of conditionals and or pattern matching.
                the corresponding function is done based on the output
            execute
    */
    
}

fn get_first_nibble(value: u8) -> u8 {
    const FIRST_NIBBLE_BIT_MASK: u8 = 0xF0;
    (value & FIRST_NIBBLE_BIT_MASK) >> 4
}
fn get_second_nibble(value: u8) -> u8 {
    const SECOND_NIBBLE_BIT_MASK: u8 = 0x0F;
    value & SECOND_NIBBLE_BIT_MASK
}