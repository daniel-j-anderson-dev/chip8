mod error;
use std::io::Read;

use self::error::Chip8Error;

pub struct Chip8 {
    program_counter: u16,
    i_register: u16,
    v_register: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: u8,
    pixels: [u8; 2048],
    key_pad: [bool; 16],
}
impl Chip8 {
    pub fn new(rom_path: &str) -> Result<Self, Chip8Error> {
        let mut chip8 = Chip8 {
            program_counter: 0,
            i_register: 0,
            v_register: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            memory: [0; 4096],
            stack: [0; 16],
            stack_pointer: 0,
            pixels: [0; 64 * 32],
            key_pad: [false; 16],
        };
        
        chip8.load_font();
        let program = read_file(rom_path)?;
        chip8.load_program(&program)?;

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
            return Err(Chip8Error::ProgramTooLarge);
        }

        for (i, program_byte) in program.iter().enumerate() {
            self.memory[PROGRAM_MEMORY_OFFSET + i] = *program_byte;
        }

        Ok(())
    }
}
impl Chip8 {
    fn get_opcode(&mut self) -> u16 {
        let program_counter = self.program_counter as usize; 
        let opcode_index = program_counter..program_counter + 1;

        match self.memory.get(opcode_index) {
            Some(&[first_byte, second_byte]) => ((first_byte as u16) << 8) | second_byte as u16,
            _ => 0,
        }
    }
}
impl Chip8 {
    pub fn emulate_cycle(&mut self) -> Result<(), Chip8Error> {
        self.execute_opcode()?;

        self.update_delay_timer();

        self.update_sound_timer();

        Ok(())
    }
    /// Returns current opcode
    fn execute_opcode(&mut self) -> Result<(), Chip8Error> {
        let opcode = self.get_opcode();
        
        self.program_counter += 2; // we have the op code so increment pc
        
        let opcode_hex_digits = [
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        ];
            
        let lhs_register_index = opcode_hex_digits[1] as usize; // second nybl
        let rhs_register_index = opcode_hex_digits[2] as usize; // third nybl
        let height = opcode_hex_digits[3] as u8; // last nybl
        let value = (opcode & 0x00FF) as u8; // last byte
        let address = opcode & 0x0FFF; // last three nybls
            
        // http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1
        match opcode_hex_digits {
            [0x0, 0x0, 0xE, 0x0] => self.opcode_00E0_clear(),
            [0x0, 0x0, 0xE, 0xE] => self.opcode_00EE_return(),
            [0x1,   _,   _,   _] => self.opcode_1nnn_jump(address),
            [0x2,   _,   _,   _] => self.opcode_2nnn_subroutine(address),
            [0x3,   _,   _,   _] => self.opcode_3xkk_skip_if_equal_value(lhs_register_index, value),
            [0x4,   _,   _,   _] => self.opcode_4xkk_skip_if_not_equal_value(lhs_register_index, value),
            [0x5,   _,   _, 0x0] => self.opcode_5xy0_skip_if_equal(lhs_register_index, rhs_register_index),
            [0x6,   _,   _,   _] => self.opcode_6xkk_assign_value(lhs_register_index, value),
            [0x7,   _,   _,   _] => self.opcode_7xkk_add_assign_value(lhs_register_index, value),
            [0x8,   _,   _, 0x0] => self.opcode_8xy0_assign(lhs_register_index, rhs_register_index),
            [0x8,   _,   _, 0x1] => self.opcode_8xy1_bitwise_or(lhs_register_index, rhs_register_index),
            [0x8,   _,   _, 0x2] => self.opcode_8xy2_bitwise_and(lhs_register_index, rhs_register_index),
            [0x8,   _,   _, 0x3] => self.opcode_8xy3_bitwise_xor(lhs_register_index, rhs_register_index),
            [0x8,   _,   _, 0x4] => self.opcode_8xy4_add(lhs_register_index, rhs_register_index),
            [0x8,   _,   _, 0x5] => self.opcode_8xy5_sub_assign(lhs_register_index, rhs_register_index),
            [0x8,   _,   _, 0x6] => self.opcode_8xy6(lhs_register_index, rhs_register_index),
            [0x8,   _,   _, 0x7] => self.opcode_8xy7(lhs_register_index, rhs_register_index),
            [0x8,   _,   _, 0xE] => self.opcode_8xyE(lhs_register_index, rhs_register_index),
            [0x9,   _,   _, 0x0] => self.opcode_9xy0(lhs_register_index, rhs_register_index),
            [0xA,   _,   _,   _] => self.opcode_Annn(address),
            [0xB,   _,   _,   _] => self.opcode_Bnnn(address),
            [0xC,   _,   _,   _] => self.opcode_Cxkk(lhs_register_index, value),
            [0xD,   _,   _,   _] => self.opcode_Dxyn(lhs_register_index, rhs_register_index, height),
            [0xE,   _, 0x9, 0xE] => self.opcode_Ex9E(lhs_register_index),
            [0xE,   _, 0xA, 0x1] => self.opcode_ExA1(lhs_register_index),
            [0xF,   _, 0x0, 0x7] => self.opcode_Fx07(lhs_register_index),
            [0xF,   _, 0x0, 0xA] => self.opcode_Fx0A(lhs_register_index),
            [0xF,   _, 0x1, 0x5] => self.opcode_Fx15(lhs_register_index),
            [0xF,   _, 0x1, 0x8] => self.opcode_Fx18(lhs_register_index),
            [0xF,   _, 0x1, 0xE] => self.opcode_Fx1E(lhs_register_index),
            [0xF,   _, 0x2, 0x9] => self.opcode_Fx29(lhs_register_index),
            [0xF,   _, 0x3, 0x3] => self.opcode_Fx33(lhs_register_index),
            [0xF,   _, 0x5, 0x5] => self.opcode_Fx55(lhs_register_index),
            [0xF,   _, 0x6, 0x5] => self.opcode_Fx65(lhs_register_index),
            _ => eprintln!("Unknown opcode: {:?}", opcode),
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

/// opcodes see: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1
impl Chip8 {
    /// Clear screen
    fn opcode_00E0_clear(&mut self) {
        const CLEAR_SCREEN: [u8; 2048] = [0; 64 * 32];
        self.pixels = CLEAR_SCREEN;
    }

    /// Return from subroutine
    fn opcode_00EE_return(&mut self) {
        self.program_counter = self.stack[self.stack_pointer as usize];
        self.stack_pointer -= 1;
    }

    /// Jumps to address nnn
    fn opcode_1nnn_jump(&mut self, address: u16) {
        self.program_counter = address;
    }

    /// Call subroutine at nnn
    fn opcode_2nnn_subroutine(&mut self, address: u16) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.program_counter = address;
    }

    /// Skips the next instruction if `V[x]` is equal to last byte of the opcode
    fn opcode_3xkk_skip_if_equal_value(&mut self, register_index: usize, value: u8) {
        if self.v_register[register_index] == value {
            self.program_counter += 2;
        }
    }

    /// Skips the next instruction if `V[x]` is NOT equal to last byte of the opcode
    fn opcode_4xkk_skip_if_not_equal_value(&mut self, register_index: usize, value: u8) {
        if self.v_register[register_index] != value {
            self.program_counter += 2;
        }
    }

    /// Skips the next instruction if `V[x]` equals `V[y]`
    fn opcode_5xy0_skip_if_equal(&mut self, lhs_register_index: usize, rhs_register_index: usize) {
        if self.v_register[lhs_register_index] == self.v_register[rhs_register_index] {
            self.program_counter += 2;
        }
    }

    /// Sets `V[x]` to kk
    fn opcode_6xkk_assign_value(&mut self, register_index: usize, value: u8) {
        self.v_register[register_index] = value;
    }

    /// Adds kk to `V[x]` (carry flag is not changed)
    fn opcode_7xkk_add_assign_value(&mut self, register_index: usize, value: u8) {
        self.v_register[register_index] = self.v_register[register_index].wrapping_add(value);
    }

    /// Sets `V[x]` to the value of `V[y]`
    fn opcode_8xy0_assign(&mut self, lhs_register_index: usize, rhs_register_index: usize) {
        self.v_register[lhs_register_index] = self.v_register[rhs_register_index];
    }

    /// Sets `V[x]` to (`V[x]` or `V[y]`) bitwise
    fn opcode_8xy1_bitwise_or(&mut self, lhs_register_index: usize, rhs_register_index: usize) {
        self.v_register[lhs_register_index] |= self.v_register[rhs_register_index];
    }

    /// Sets `V[x]` to `V[x]` and `V[y]` (bitwise)
    fn opcode_8xy2_bitwise_and(&mut self, lhs_register_index: usize, rhs_register_index: usize) {
        self.v_register[lhs_register_index] &= self.v_register[rhs_register_index];
    }

    /// Sets `V[x]` to `V[x]` xor `V[y]` (bitwise)
    fn opcode_8xy3_bitwise_xor(&mut self, lhs_register_index: usize, rhs_register_index: usize) {
        self.v_register[lhs_register_index] ^= self.v_register[rhs_register_index];
    }

    /// Adds `V[y]` to `V[x]`. `V[0xF]` is set to 1 when there's an overflow, and to 0 when there is not
    fn opcode_8xy4_add(&mut self, lhs_register_index: usize, rhs_register_index: usize) {

    }

    /// `V[y]` is subtracted from `V[x]`. `V[0xF]` is set to 0 when there's an underflow, and 1 when there is not
    fn opcode_8xy5_sub_assign(&mut self, lhs_register_index: usize, rhs_register_index: usize) {
        unimplemented!()
    }

    /// Stores the least significant bit of `V[x]` in `V[0xF]` and then shifts `V[x]` to the right by 1
    fn opcode_8xy6(&mut self, lhs_register_index: usize, rhs_register_index: usize) {
        unimplemented!()
    }

    /// Sets `V[x]` to `V[y]` minus `V[x]`. `V[0xF]` is set to 0 when there's an underflow, and 1 when there is not
    fn opcode_8xy7(&mut self, lhs_register_index: usize, rhs_register_index: usize) {
        unimplemented!()
    }

    /// Stores the most significant bit of `V[x]` in `V[0xF]` and then shifts `V[x]` to the left by 1
    fn opcode_8xyE(&mut self, lhs_register_index: usize, rhs_register_index: usize) {
        unimplemented!()
    }

    /// Skips the next instruction if `V[x]` does not equal `V[y]`
    fn opcode_9xy0(&mut self, lhs_register_index: usize, rhs_register_index: usize) {
        unimplemented!()
    }

    /// Sets I to the address nnn
    fn opcode_Annn(&mut self, address: u16) {
        unimplemented!()
    }

    /// Jumps to the address nnn plus V[0]
    fn opcode_Bnnn(&mut self, address: u16) {
        unimplemented!()
    }

    /// Sets `V[x]` to the result of a bitwise and operation on a random number (Typically: 0 to 255) and kk
    fn opcode_Cxkk(&mut self, register_index: usize, value: u8) {
        unimplemented!()
    }

    /// draw a sprite
    fn opcode_Dxyn(&mut self, lhs_register_index: usize, rhs_register_index: usize, height: u8) {
        unimplemented!()
    }

    /// Skips the next instruction if the key stored in `V[x]` is pressed
    fn opcode_Ex9E(&mut self, register_index: usize) {
        unimplemented!()
    }

    /// Skips the next instruction if the key stored in `V[x]` is NOT pressed
    fn opcode_ExA1(&mut self, register_index: usize) {
        unimplemented!()
    }

    /// Sets `V[x]` to the value of the delay timer
    fn opcode_Fx07(&mut self, register_index: usize) {
        unimplemented!()
    }

    /// A key press is awaited, and then stored in `V[x]`
    fn opcode_Fx0A(&mut self, register_index: usize) {
        unimplemented!()
    }

    /// Sets the delay timer to `V[x]`
    fn opcode_Fx15(&mut self, register_index: usize) {
        unimplemented!()
    }

    /// Sets the sound timer to `V[x]`
    fn opcode_Fx18(&mut self, register_index: usize) {
        unimplemented!()
    }

    /// Adds `V[x]` to I. `V[0xF]` is not affected.
    fn opcode_Fx1E(&mut self, register_index: usize) {
        unimplemented!()
    }

    /// Sets I to the location of the sprite for the character in `V[x]`
    fn opcode_Fx29(&mut self, register_index: usize) {
        unimplemented!()
    }

    /// Stores the binary-coded decimal representation of V  _, with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2
    fn opcode_Fx33(&mut self, register_index: usize) {
        unimplemented!()
    }

    /// Stores from `V[0]` to `V[x]` (including `V[x]`) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified
    fn opcode_Fx55(&mut self, register_index: usize) {
        unimplemented!()
    }

    /// Fills from `V[0]` to `V[x]` (including `V[x]`) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified
    fn opcode_Fx65(&mut self, register_index: usize) {
        unimplemented!()
    }
}

fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    use std::{io::BufReader, fs::File};
    let mut file = BufReader::new(File::open(path)?);
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}
