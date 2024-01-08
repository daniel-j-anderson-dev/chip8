/*
memory map
0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
0x200-0xFFF - Program ROM and work RAM
*/
#[derive(Debug)]
pub enum Chip8Error {
    UnknownOpcode(u16),
    ProgramTooLarge,
}
impl std::fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chip8 Error: {},", match self {
            Chip8Error::UnknownOpcode(opcode) => format!("unknown opcode of 0x{:x}", opcode),
            Chip8Error::ProgramTooLarge => String::from("The program could not fit in the 3584 bytes of program memory"),
        })
    }
}
impl std::error::Error for Chip8Error {}

pub struct Chip8 {
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: u16,
    current_opcode: u16,
    program_counter: u16,
    index_register: u16,
    v_register: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    pixels: [u8; 64 * 32], // 2048
    key_pad: [bool; 16],
}
impl Chip8 {
    pub fn new(program: &[u8]) -> Result<Self, Chip8Error> {
        let mut chip8 = Chip8 {
            program_counter: 0,
            current_opcode: 0,
            index_register: 0,
            stack_pointer: 0,
            v_register: [0; 16],
            memory: [0; 4096],
            stack: [0; 16],
            pixels: [0; 64 * 32],
            key_pad: [false; 16],
            delay_timer: 0,
            sound_timer: 0,
        };

        chip8.load_font();
        chip8.load_program(program)?;

        Ok(chip8)
    }
    fn load_font(&mut self) {
        const FONT: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for (i, font_byte) in FONT.iter().enumerate() {
            self.memory[i] = *font_byte;
        }
    }
    fn load_program(&mut self, program: &[u8]) -> Result<(), Chip8Error> {
        const PROGRAM_MEMORY_OFFSET: usize = 512;
        if program.len() > self.memory.len() - PROGRAM_MEMORY_OFFSET {
            return Err(Chip8Error::ProgramTooLarge)
        }
        
        for (i, program_byte) in program.iter().enumerate() {
            self.memory[PROGRAM_MEMORY_OFFSET + i] = *program_byte;
        }

        Ok(())
    }
}
impl Chip8 {
    pub fn emulate_cycle(&mut self) -> Result<(), Chip8Error> {
        
        self.update_opcode();

        self.execute_opcode()?;

        self.update_delay_timer();

        self.update_sound_timer();

        Ok(())
    }
    fn update_opcode(&mut self) {
        let first_byte = self.memory[self.program_counter as usize];
        let second_byte = self.memory[self.program_counter as usize + 1];
        self.current_opcode = combine_bytes(first_byte,second_byte);
    }
    fn execute_opcode(&mut self) -> Result<(), Chip8Error> {
        let opcode_hex_digits: [u8; 4] = [
            ((self.current_opcode & 0xF000) >> 12) as u8,
            ((self.current_opcode & 0x0F00) >> 8) as u8,
            ((self.current_opcode & 0x00F0) >> 4) as u8,
            (self.current_opcode & 0x000F) as u8,
        ];

        match opcode_hex_digits {
            [0x0, 0x0, 0xE, 0x0] => unimplemented!(), // clear the screen
            [0x0, 0x0, 0xE, 0xE] => unimplemented!(), // return from subroutine
            [0x1,  n1,  n2,  n3] => unimplemented!(), // Jumps to address NNN
            [0x2,  n1,  n2,  n3] => unimplemented!(), // Calls subroutine at NNN
            [0x3,   x,  n2,  n3] => unimplemented!(), // Skips the next instruction if VX is equal to last two nybls
            [0x4,   x,  n2,  n3] => unimplemented!(), // Skips the next instruction if VX is NOT equal to last two nybls
            [0x5,   x,   y, 0x0] => unimplemented!(), // Skips the next instruction if VX equals VY
            [0x6,   x,  n2,  n3] => unimplemented!(), // Sets VX to NN
            [0x7,   x,  n2,  n3] => unimplemented!(), // Adds NN to VX (carry flag is not changed)
            [0x8,   x,   y, 0x0] => unimplemented!(), // Sets VX to the value of VY (bitwise)
            [0x8,   x,   y, 0x1] => unimplemented!(), // Sets VX to VX or VY (bitwise)
            [0x8,   x,   y, 0x2] => unimplemented!(), // Sets VX to VX and VY (bitwise)
            [0x8,   x,   y, 0x3] => unimplemented!(), // Sets VX to VX xor VY (bitwise)
            [0x8,   x,   y, 0x4] => unimplemented!(), // Adds VY to VX. VF is set to 1 when there's an overflow, and to 0 when there is not
            [0x8,   x,   y, 0x5] => unimplemented!(), // Y is subtracted from VX. VF is set to 0 when there's an underflow, and 1 when there is not
            [0x8,   x,   y, 0x6] => unimplemented!(), // Stores the least significant bit of VX in VF and then shifts VX to the right by 1
            [0x8,   x,   y, 0x7] => unimplemented!(), // Sets VX to VY minus VX. VF is set to 0 when there's an underflow, and 1 when there is not
            [0x8,   x,   y, 0xE] => unimplemented!(), // Stores the most significant bit of VX in VF and then shifts VX to the left by 1
            [0x9,   x,   y, 0x0] => unimplemented!(), // Skips the next instruction if VX does not equal VY
            [0xA,  n1,  n2,  n3] => unimplemented!(), // Sets I to the address NNN
            [0xB,  n1,  n2,  n3] => unimplemented!(), // Jumps to the address NNN plus V0
            [0xC,   x,  n2,  n3] => unimplemented!(), // Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN
            [0xD,   x,   y,  n3] => unimplemented!(), // draw a sprite
            [0xE,   x, 0x9, 0xE] => unimplemented!(), // Skips the next instruction if the key stored in VX is pressed 
            [0xE,   x, 0xA, 0x1] => unimplemented!(), // Skips the next instruction if the key stored in VX is NOT pressed
            [0xF,   x, 0x0, 0x7] => unimplemented!(), // Sets VX to the value of the delay timer
            [0xF,   x, 0x0, 0xA] => unimplemented!(), // A key press is awaited, and then stored in VX
            [0xF,   x, 0x1, 0x5] => unimplemented!(), // Sets the delay timer to VX
            [0xF,   x, 0x1, 0x8] => unimplemented!(), // Sets the sound timer to VX
            [0xF,   x, 0x1, 0xE] => unimplemented!(), // Adds VX to I. VF is not affected.
            [0xF,   x, 0x2, 0x9] => unimplemented!(), // Sets I to the location of the sprite for the character in VX
            [0xF,   x, 0x3, 0x3] => unimplemented!(), // Stores the binary-coded decimal representation of VX, with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2
            [0xF,   x, 0x5, 0x5] => unimplemented!(), // Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified
            [0xF,   x, 0x6, 0x5] => unimplemented!(), // Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified 
            unknown_opcode => return Err(Chip8Error::UnknownOpcode(self.current_opcode)),
        }

        Ok(())
    }
    fn update_delay_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }
    fn update_sound_timer(&mut self) {
        if self.sound_timer > 0 {
            
            if self.sound_timer == 1 {
                // TODO: make sound
                println!("BEEP!");
            }

            self.sound_timer -= 1;
        }
    }
}
fn combine_bytes(top_half: u8, bottom_half: u8) -> u16 {
    (top_half as u16) << 8 | bottom_half as u16
}
