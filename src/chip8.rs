mod error;
use self::error::Chip8Error;

pub struct Chip8 {
    program_counter: u16,
    instruction: u16,
    i_register: u16,
    v_register: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: u16,
    pixels: [u8; 64 * 32], // 2048
    key_pad: [bool; 16],
}
impl Chip8 {
    pub fn new(program: &[u8]) -> Result<Self, Chip8Error> {
        let mut chip8 = Chip8 {
            program_counter: 0,
            instruction: 0,
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
            return Err(Chip8Error::ProgramTooLarge);
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
        let opcode_index = self.program_counter as usize..self.program_counter as usize + 1;

        let [top_half, bottom_half] = match self.memory.get(opcode_index) {
            Some(&[top_half, bottom_half]) => [top_half, bottom_half],
            _ => [0, 0],
        };

        self.instruction = (top_half as u16) << 8 | bottom_half as u16
    }
    fn split_opcode_into_hex_digits(&self) -> [u8; 4] {
        [
            ((self.instruction & 0xF000) >> 12) as u8,
            ((self.instruction & 0x0F00) >> 8) as u8,
            ((self.instruction & 0x00F0) >> 4) as u8,
            (self.instruction & 0x000F) as u8,
        ]
    }
    // http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1
    fn execute_opcode(&mut self) -> Result<(), Chip8Error> {
        let opcode_hex_digits = self.split_opcode_into_hex_digits();

        let lhs_register_index = opcode_hex_digits[1] as usize; // second nybl
        let rhs_register_index = opcode_hex_digits[2] as usize; // third nybl
        let height = opcode_hex_digits[3] as u8; // last nybl
        let value = (self.instruction & 0x00FF) as u8; // last byte
        let address = self.instruction & 0x0FFF; // last three nybls

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
            [0x8,   _,   _, 0x6] => self.opcode_8xy6_right_shift_assign(lhs_register_index, rhs_register_index),
            [0x8,   _,   _, 0x7] => self.opcode_8xy7_sub_assign_reverse(lhs_register_index, rhs_register_index),
            [0x8,   _,   _, 0xE] => self.opcode_8xyE_left_shift_assign(lhs_register_index, rhs_register_index),
            [0x9,   _,   _, 0x0] => self.opcode_9xy0_skip_if_not_equal(lhs_register_index, rhs_register_index),
            [0xA,   _,   _,   _] => self.opcode_Annn_load_i(address),
            [0xB,   _,   _,   _] => self.opcode_Bnnn_jump_offset(address),
            [0xC,   _,   _,   _] => self.opcode_Cxkk_random(lhs_register_index, value),
            [0xD,   _,   _,   _] => self.opcode_Dxyn_draw_sprite(lhs_register_index, rhs_register_index, height),
            [0xE,   _, 0x9, 0xE] => self.opcode_Ex9E_skip_if_key_pressed(lhs_register_index),
            [0xE,   _, 0xA, 0x1] => self.opcode_ExA1_skip_if_key_not_pressed(lhs_register_index),
            [0xF,   _, 0x0, 0x7] => self.opcode_Fx07_load_delay_timer(lhs_register_index),
            [0xF,   _, 0x0, 0xA] => self.opcode_Fx0A_wait_for_key_press(lhs_register_index),
            [0xF,   _, 0x1, 0x5] => self.opcode_Fx15_set_delay_timer(lhs_register_index),
            [0xF,   _, 0x1, 0x8] => self.opcode_Fx18_set_sound_timer(lhs_register_index),
            [0xF,   _, 0x1, 0xE] => self.opcode_Fx1E_add_assign_i(lhs_register_index),
            [0xF,   _, 0x2, 0x9] => self.opcode_Fx299_load_sprite(lhs_register_index),
            [0xF,   _, 0x3, 0x3] => self.opcode_Fx33_store_decimal(lhs_register_index),
            [0xF,   _, 0x5, 0x5] => self.opcode_Fx55_store_registers_i(lhs_register_index),
            [0xF,   _, 0x6, 0x5] => self.opcode_Fx65_read_registers_i(lhs_register_index),
            _ => eprintln!("Unknown opcode: {}", self.instruction),
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
    fn opcode_00E0_clear(&mut self) {}

    /// Return from subroutine
    fn opcode_00EE_return(&mut self) {}

    /// Jumps to address nnn
    fn opcode_1nnn_jump(&mut self, address: u16) {}

    /// Call subroutine at nnn
    fn opcode_2nnn_subroutine(&mut self, address: u16) {}

    /// Skips the next instruction if `V[x]` is equal to last byte of the opcode
    fn opcode_3xkk_skip_if_equal_value(&mut self, register_index: usize, value: u8) {}

    /// Skips the next instruction if `V[x]` is NOT equal to last byte of the opcode
    fn opcode_4xkk_skip_if_not_equal_value(&mut self, register_index: usize, value: u8) {}

    /// Skips the next instruction if `V[x]` equals `V[y]`
    fn opcode_5xy0_skip_if_equal(&mut self, lhs_register_index: usize, rhs_register_index: usize) {}

    /// Sets `V[x]` to kk
    fn opcode_6xkk_assign_value(&mut self, register_index: usize, value: u8) {}

    /// Adds kk to `V[x]` (carry flag is not changed)
    fn opcode_7xkk_add_assign_value(&mut self, register_index: usize, value: u8) {}

    /// Sets `V[x]` to the value of `V[y]`
    fn opcode_8xy0_assign(&mut self, lhs_register_index: usize, rhs_register_index: usize) {}

    /// Sets `V[x]` to (`V[x]` or `V[y]`) bitwise
    fn opcode_8xy1_bitwise_or(&mut self, lhs_register_index: usize, rhs_register_index: usize) {}

    /// Sets `V[x]` to `V[x]` and `V[y]` (bitwise)
    fn opcode_8xy2_bitwise_and(&mut self, lhs_register_index: usize, rhs_register_index: usize) {}

    /// Sets `V[x]` to `V[x]` xor `V[y]` (bitwise)
    fn opcode_8xy3_bitwise_xor(&mut self, lhs_register_index: usize, rhs_register_index: usize) {}

    /// Adds `V[y]` to `V[x]`. `V[0xF]` is set to 1 when there's an overflow, and to 0 when there is not
    fn opcode_8xy4_add(&mut self, lhs_register_index: usize, rhs_register_index: usize) {}

    /// `V[y]` is subtracted from `V[x]`. `V[0xF]` is set to 0 when there's an underflow, and 1 when there is not
    fn opcode_8xy5_sub_assign(&mut self, lhs_register_index: usize, rhs_register_index: usize) {}

    /// Stores the least significant bit of `V[x]` in `V[0xF]` and then shifts `V[x]` to the right by 1
    fn opcode_8xy6_right_shift_assign(&mut self, lhs_register_index: usize, rhs_register_index: usize) {}

    /// Sets `V[x]` to `V[y]` minus `V[x]`. `V[0xF]` is set to 0 when there's an underflow, and 1 when there is not
    fn opcode_8xy7_sub_assign_reverse(&mut self, lhs_register_index: usize, rhs_register_index: usize) {}

    /// Stores the most significant bit of `V[x]` in `V[0xF]` and then shifts `V[x]` to the left by 1
    fn opcode_8xyE_left_shift_assign(&mut self, lhs_register_index: usize, rhs_register_index: usize) {}

    /// Skips the next instruction if `V[x]` does not equal `V[y]`
    fn opcode_9xy0_skip_if_not_equal(&mut self, lhs_register_index: usize, rhs_register_index: usize) {}

    /// Sets I to the address nnn
    fn opcode_Annn_load_i(&mut self, address: u16) {}

    /// Jumps to the address nnn plus `[0]`
    fn opcode_Bnnn_jump_offset(&mut self, address: u16) {}

    /// Sets `V[x]` to the result of a bitwise and operation on a random number (Typically: 0 to 255) and kk
    fn opcode_Cxkk_random(&mut self, register_index: usize, value: u8) {}

    /// draw a sprite
    fn opcode_Dxyn_draw_sprite(&mut self, lhs_register_index: usize, rhs_register_index: usize, height: u8) {}

    /// Skips the next instruction if the keycode stored in `V[x]` is pressed
    fn opcode_Ex9E_skip_if_key_pressed(&mut self, register_index: usize) {}

    /// Skips the next instruction if the key stored in `V[x]` is NOT pressed
    fn opcode_ExA1_skip_if_key_not_pressed(&mut self, register_index: usize) {}

    /// Sets `V[x]` to the value of the delay timer
    fn opcode_Fx07_load_delay_timer(&mut self, ressgister_index: usize) {}

    /// A key press is awaited, and then stored in `V[x]`
    fn opcode_Fx0A_wait_for_key_press(&mut self, register_index: usize) {}

    /// Sets the delay timer to `V[x]`
    fn opcode_Fx15_set_delay_timer(&mut self, register_index: usize) {}

    /// Sets the sound timer to `V[x]`
    fn opcode_Fx18_set_sound_timer(&mut self, register_index: usize) {}

    /// Adds `V[x]` to I. `V[0xF]` is not affected.
    fn opcode_Fx1E_add_assign_i(&mut self, register_index: usize) {}

    /// Sets I to the location of the sprite for the character in `V[x]`
    fn opcode_Fx299_load_sprite(&mut self, register_index: usize) {}

    /// Store BinaryCodedDecimal representation of `V[x]` in `memory[I..I+2]`
    fn opcode_Fx33_store_decimal(&mut self, register_index: usize) {}

    /// Stores from `V[0]` to `V[x]` (including `V[x]`) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified
    fn opcode_Fx55_store_registers_i(&mut self, register_index: usize) {}

    /// Fills from `V[0]` to `V[x]` (including `V[x]`) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified
    fn opcode_Fx65_read_registers_i(&mut self, register_index: usize) {}

    /// Fills from `V[0]` to `V[x]` (including `V[x]`) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified
    fn opcode_Fx65_read_registers_I(&mut self, register_index: usize) {}
}
