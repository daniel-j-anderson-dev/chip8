mod error;

use self::error::Chip8Error;

use rand::{rngs::ThreadRng, Rng};

pub struct Chip8 {
    program_counter: u16,
    i_register: u16,
    v_register: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: u8,
    pixels: [bool; Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT],
    key_pad: [bool; 16],
    rng: ThreadRng,
}
impl Chip8 {
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
    const PROGRAM_MEMORY_OFFSET: usize = 512;
    const CLEAR_SCREEN: [bool; 2048] = [false; Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT];
    pub const SCREEN_WIDTH: usize = 64;
    pub const SCREEN_HEIGHT: usize = 32;
}
impl Chip8 {
    pub fn initialize() -> Result<Self, Chip8Error> {
        let mut chip8 = Chip8 {
            program_counter: 0,
            i_register: 0,
            v_register: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            memory: [0; 4096],
            stack: [0; 16],
            stack_pointer: 0,
            pixels: Self::CLEAR_SCREEN,
            key_pad: [false; 16],
            rng: rand::thread_rng(),
        };

        chip8.memory[..Self::FONT.len()].copy_from_slice(&Self::FONT);

        Ok(chip8)
    }
    pub fn load_program(&mut self, program_path: &str) -> Result<(), Chip8Error> {
        let program = read_file(program_path)?;

        if program.len() > self.memory.len() - Self::PROGRAM_MEMORY_OFFSET {
            return Err(Chip8Error::ProgramTooLarge);
        }

        self.memory[Self::PROGRAM_MEMORY_OFFSET..].copy_from_slice(&program);

        Ok(())
    }
}
impl Chip8 {
    fn get_opcode(&self) -> u16 {
        let program_counter = self.program_counter as usize;
        let opcode_index = program_counter..program_counter + 1;

        match self.memory.get(opcode_index) {
            Some(&[first_byte, second_byte]) => ((first_byte as u16) << 8) | second_byte as u16,
            _ => 0,
        }
    }
}
impl Chip8 {
    pub fn step_execution(&mut self) {
        let opcode = self.get_opcode();
        self.program_counter += 2; // we have the op code so increment pc

        self.execute_opcode(opcode);

        self.update_timers();
    }
    fn execute_opcode(&mut self, opcode: u16) {
        let opcode_hex_digits = [
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        ];

        let x_register_index = opcode_hex_digits[1] as usize; // second nybl left hand side register index
        let y_register_index = opcode_hex_digits[2] as usize; // third nybl right hand side register index
        let height = opcode_hex_digits[3] as u8; // last nybl
        let value = (opcode & 0x00FF) as u8; // last byte
        let address = opcode & 0x0FFF; // last three nybls

        // http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1
        match opcode_hex_digits {
            [0x0, 0x0, 0x0, 0x0] => {}
            [0x0, 0x0, 0xE, 0x0] => self.opcode_00E0_clear(),
            [0x0, 0x0, 0xE, 0xE] => self.opcode_00EE_return(),
            [0x1,   _,   _,   _] => self.opcode_1nnn_jump(address),
            [0x2,   _,   _,   _] => self.opcode_2nnn_call_subroutine(address),
            [0x3,   _,   _,   _] => self.opcode_3xkk_skip_if_equal_value(x_register_index, value),
            [0x4,   _,   _,   _] => self.opcode_4xkk_skip_if_not_equal_value(x_register_index, value),
            [0x5,   _,   _, 0x0] => self.opcode_5xy0_skip_if_equal(x_register_index, y_register_index),
            [0x6,   _,   _,   _] => self.opcode_6xkk_assign_value(x_register_index, value),
            [0x7,   _,   _,   _] => self.opcode_7xkk_add_assign_value(x_register_index, value),
            [0x8,   _,   _, 0x0] => self.opcode_8xy0_assign(x_register_index, y_register_index),
            [0x8,   _,   _, 0x1] => self.opcode_8xy1_bitwise_or(x_register_index, y_register_index),
            [0x8,   _,   _, 0x2] => self.opcode_8xy2_bitwise_and(x_register_index, y_register_index),
            [0x8,   _,   _, 0x3] => self.opcode_8xy3_bitwise_xor(x_register_index, y_register_index),
            [0x8,   _,   _, 0x4] => self.opcode_8xy4_add_assign(x_register_index, y_register_index),
            [0x8,   _,   _, 0x5] => self.opcode_8xy5_sub_assign(x_register_index, y_register_index),
            [0x8,   _,   _, 0x6] => self.opcode_8xy6_shift_right_assign(x_register_index, y_register_index),
            [0x8,   _,   _, 0x7] => self.opcode_8xy7_sub_assign_swapped(x_register_index, y_register_index),
            [0x8,   _,   _, 0xE] => self.opcode_8xyE_left_shift_assign(x_register_index, y_register_index),
            [0x9,   _,   _, 0x0] => self.opcode_9xy0_skip_if_not_equal(x_register_index, y_register_index),
            [0xA,   _,   _,   _] => self.opcode_Annn_set_i_register(address),
            [0xB,   _,   _,   _] => self.opcode_Bnnn_jump_offset(address),
            [0xC,   _,   _,   _] => self.opcode_Cxkk_random_number_assign(x_register_index, value),
            [0xD,   _,   _,   _] => self.opcode_Dxyn_draw_sprite(x_register_index, y_register_index, height),
            [0xE,   _, 0x9, 0xE] => self.opcode_Ex9E_skip_on_key_pressed(x_register_index),
            [0xE,   _, 0xA, 0x1] => self.opcode_ExA1_skip_on_key_not_pressed(x_register_index),
            [0xF,   _, 0x0, 0x7] => self.opcode_Fx07_store_delay_timer(x_register_index),
            [0xF,   _, 0x0, 0xA] => self.opcode_Fx0A_wait_for_key_press(x_register_index),
            [0xF,   _, 0x1, 0x5] => self.opcode_Fx15_set_delay_timer(x_register_index),
            [0xF,   _, 0x1, 0x8] => self.opcode_Fx18_set_sound_timer(x_register_index),
            [0xF,   _, 0x1, 0xE] => self.opcode_Fx1E_i_register_add_assaign(x_register_index),
            [0xF,   _, 0x2, 0x9] => self.opcode_Fx29_set_i_to_font_address(x_register_index),
            [0xF,   _, 0x3, 0x3] => self.opcode_Fx33_store_bcd_at_i(x_register_index),
            [0xF,   _, 0x5, 0x5] => self.opcode_Fx55_store_v_registers(x_register_index),
            [0xF,   _, 0x6, 0x5] => self.opcode_Fx65_load_v_registers(x_register_index),
            _ => eprintln!("Unknown opcode: {:?}", opcode),
        }
    }
    fn update_timers(&mut self) {
        self.update_delay_timer();
        self.update_sound_timer();
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
        self.pixels = Self::CLEAR_SCREEN;
    }

    /// Return from subroutine
    fn opcode_00EE_return(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];
    }

    /// Jumps to address address
    fn opcode_1nnn_jump(&mut self, address: u16) {
        self.program_counter = address;
    }

    /// Call subroutine at address
    fn opcode_2nnn_call_subroutine(&mut self, address: u16) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = address;
    }

    /// Skips the next instruction if `v_register[x_register_index]` is equal to last byte of the opcode
    fn opcode_3xkk_skip_if_equal_value(&mut self, register_index: usize, value: u8) {
        if self.v_register[register_index] == value {
            self.program_counter += 2;
        }
    }

    /// Skips the next instruction if `v_register[x_register_index]` is NOT equal to last byte of the opcode
    fn opcode_4xkk_skip_if_not_equal_value(&mut self, register_index: usize, value: u8) {
        if self.v_register[register_index] != value {
            self.program_counter += 2;
        }
    }

    /// Skips the next instruction if `v_register[x_register_index]` equals `v_register[y_register_index]`
    fn opcode_5xy0_skip_if_equal(&mut self, x_register_index: usize, y_register_index: usize) {
        if self.v_register[x_register_index] == self.v_register[y_register_index] {
            self.program_counter += 2;
        }
    }

    /// Sets `v_register[x_register_index]` to value
    fn opcode_6xkk_assign_value(&mut self, register_index: usize, value: u8) {
        self.v_register[register_index] = value;
    }

    /// Adds value to `v_register[x_register_index]` (carry flag is not changed)
    fn opcode_7xkk_add_assign_value(&mut self, register_index: usize, value: u8) {
        self.v_register[register_index] = self.v_register[register_index].wrapping_add(value);
    }

    /// Sets `v_register[x_register_index]` to the value of `v_register[y_register_index]`
    fn opcode_8xy0_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.v_register[x_register_index] = self.v_register[y_register_index];
    }

    /// Sets `v_register[x_register_index]` to (`v_register[x_register_index]` or `v_register[y_register_index]`) bitwise
    fn opcode_8xy1_bitwise_or(&mut self, x_register_index: usize, y_register_index: usize) {
        self.v_register[x_register_index] |= self.v_register[y_register_index];
    }

    /// Sets `v_register[x_register_index]` to `v_register[x_register_index]` and `v_register[y_register_index]` (bitwise)
    fn opcode_8xy2_bitwise_and(&mut self, x_register_index: usize, y_register_index: usize) {
        self.v_register[x_register_index] &= self.v_register[y_register_index];
    }

    /// Sets `v_register[x_register_index]` to `v_register[x_register_index]` xor `v_register[y_register_index]` (bitwise)
    fn opcode_8xy3_bitwise_xor(&mut self, x_register_index: usize, y_register_index: usize) {
        self.v_register[x_register_index] ^= self.v_register[y_register_index];
    }

    /// Adds `v_register[y_register_index]` to `v_register[x_register_index]`. `v_register[0xF]` is set to 1 when there's an overflow, and to 0 when there is not
    fn opcode_8xy4_add_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        let (sum, overflow_occurred) =
            self.v_register[x_register_index].overflowing_add(self.v_register[y_register_index]);

        self.v_register[x_register_index] = sum;
        self.v_register[0xF] = if overflow_occurred { 1 } else { 0 };
    }

    /// `v_register[y_register_index]` is subtracted from `v_register[x_register_index]`. `v_register[0xF]` is set to 0 when there's an underflow, and 1 when there is not
    fn opcode_8xy5_sub_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        let (difference, underflow_occurred) =
            self.v_register[x_register_index].overflowing_sub(self.v_register[y_register_index]);

        self.v_register[x_register_index] = difference;
        self.v_register[0xF] = if underflow_occurred { 0 } else { 1 };
    }

    /// Stores the least significant bit of `v_register[x_register_index]` in `v_register[0xF]` and then shifts `v_register[x_register_index]` to the right by 1
    fn opcode_8xy6_shift_right_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.v_register[0xF] = self.v_register[x_register_index] & 0b00000001;
        self.v_register[x_register_index] >>= 1;
    }

    /// Sets `v_register[x_register_index]` to `v_register[y_register_index]` minus `v_register[x_register_index]`. `v_register[0xF]` is set to 0 when there's an underflow, and 1 when there is not
    fn opcode_8xy7_sub_assign_swapped(&mut self, x_register_index: usize, y_register_index: usize) {
        let (difference, underflow_occurred) =
            self.v_register[y_register_index].overflowing_sub(self.v_register[x_register_index]);

        self.v_register[x_register_index] = difference;
        self.v_register[0xF] = if underflow_occurred { 0 } else { 1 };
    }

    /// Stores the most significant bit of `v_register[x_register_index]` in `v_register[0xF]` and then shifts `v_register[x_register_index]` to the left by 1
    fn opcode_8xyE_left_shift_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.v_register[0xF] = self.v_register[x_register_index].reverse_bits() & 0b00000001;
        self.v_register[x_register_index] <<= 1;
    }

    /// Skips the next instruction if `v_register[x_register_index]` does not equal `v_register[y_register_index]`
    fn opcode_9xy0_skip_if_not_equal(&mut self, x_register_index: usize, y_register_index: usize) {
        if self.v_register[x_register_index] != self.v_register[y_register_index] {
            self.program_counter += 2;
        }
    }

    /// Sets I to the address address
    fn opcode_Annn_set_i_register(&mut self, address: u16) {
        self.i_register = address;
    }

    /// Jumps to the address address plus v_register[0]
    fn opcode_Bnnn_jump_offset(&mut self, address: u16) {
        self.program_counter = address + (self.v_register[0] as u16);
    }

    /// Sets `v_register[x_register_index]` to the result of a bitwise and operation on a random number (Typically: 0 to 255) and value
    fn opcode_Cxkk_random_number_assign(&mut self, register_index: usize, value: u8) {
        self.v_register[register_index] = self.rng.gen::<u8>() & value;
    }

    /// draw a sprite
    fn opcode_Dxyn_draw_sprite(&mut self, x_register_index: usize, y_register_index: usize, n: u8) {
        // get the position from the two registers
        let initial_x = self.v_register[x_register_index] as usize;
        let initial_y = self.v_register[y_register_index] as usize;

        // width is always 8, height is the last nybl of the opcode
        let height = n as usize;
        const WIDTH: usize = 8;

        let mut white_to_black_occurred = false;

        for row_index in 0..height {
            // select the byte specified by the row index
            let row_address = self.i_register as usize + row_index; // calculate the row address
            let current_row = self.memory[row_address];

            for column_index in 0..WIDTH {
                // select the bit specified by
                let current_pixel_mask = 0b10000000u8 >> column_index;
                let current_pixel = current_row & current_pixel_mask;

                // change color of pixel when 1
                if current_pixel != 0 {
                    // sprites wrap at screen boundaries
                    let pixel_x = (initial_x + column_index) % 64;
                    let pixel_y = (initial_y + row_index) % 32;

                    // calculate the 1D screen index
                    let pixel_index = (pixel_y * 64) + pixel_x;

                    white_to_black_occurred |= self.pixels[pixel_index];
                    self.pixels[pixel_index] ^= true;
                }
            }
        }

        self.v_register[0xF] = if white_to_black_occurred { 1 } else { 0 };
    }

    /// Skips the next instruction if the key stored in `v_register[register_index]` is pressed
    fn opcode_Ex9E_skip_on_key_pressed(&mut self, register_index: usize) {
        let key_index = self.v_register[register_index] as usize;
        let is_key_pressed = self.key_pad[key_index];
        if is_key_pressed {
            self.program_counter += 2;
        }
    }

    /// Skips the next instruction if the key stored in `v_register[register_index]` is NOT pressed
    fn opcode_ExA1_skip_on_key_not_pressed(&mut self, register_index: usize) {
        let key_index = self.v_register[register_index] as usize;
        let is_key_pressed = self.key_pad[key_index];
        if !is_key_pressed {
            self.program_counter += 2;
        }
    }

    /// Sets `v_register[register_index]` to the value of the delay timer
    fn opcode_Fx07_store_delay_timer(&mut self, register_index: usize) {
        self.v_register[register_index] = self.delay_timer;
    }

    /// A key press is awaited, and then stored in `v_register[register_index]`
    fn opcode_Fx0A_wait_for_key_press(&mut self, register_index: usize) {
        let mut key_press_occurred = false;

        // check each key
        for (key_index, is_key_pressed) in self.key_pad.iter().enumerate() {
            if *is_key_pressed {
                // store the index of the first key press found
                self.v_register[register_index] = key_index as u8;

                // stop at the first key press
                key_press_occurred = true;
                break;
            }
        }

        if !key_press_occurred {
            // try this opcode again if none of the keys were pressed
            self.program_counter -= 2;
        }
    }

    /// Sets the delay timer to `v_register[register_index]`
    fn opcode_Fx15_set_delay_timer(&mut self, register_index: usize) {
        self.delay_timer = self.v_register[register_index];
    }

    /// Sets the sound timer to `v_register[register_index]`
    fn opcode_Fx18_set_sound_timer(&mut self, register_index: usize) {
        self.sound_timer = self.v_register[register_index];
    }

    /// Adds `v_register[register_index]` to I. `v_register[0xF]` is not affected.
    fn opcode_Fx1E_i_register_add_assaign(&mut self, register_index: usize) {
        let addend = self.v_register[register_index] as u16;
        self.i_register = self.i_register.wrapping_add(addend);
    }

    /// Sets I to the location of the sprite for the character in `v_register[register_index]`
    /// Font starts at memory address 0
    fn opcode_Fx29_set_i_to_font_address(&mut self, register_index: usize) {
        let character = self.v_register[register_index] as u16;

        // each character is 5 bytes apart and the first character is at address 0
        let character_sprite_address = character * 5;

        self.i_register = character_sprite_address;
    }

    /// Stores the binary-coded decimal representation of v_register[register_index], with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2
    fn opcode_Fx33_store_bcd_at_i(&mut self, register_index: usize) {
        let value = self.v_register[register_index];

        let decimal_digits = [
            (value / 100) % 10, //
            (value / 10) % 10,
            (value / 1) % 10,
        ];

        let i_register = self.i_register as usize;

        self.memory[i_register..=i_register + 2].copy_from_slice(&decimal_digits);
    }

    /// Stores from `v_register[0]` to `v_register[register_index]` (including `v_register[register_index]`) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified
    fn opcode_Fx55_store_v_registers(&mut self, register_index: usize) {
        let offset = self.i_register as usize;
        self.memory[offset..offset + register_index]
            .copy_from_slice(&self.v_register[..register_index]);
    }

    /// Fills from `v_register[0]` to `v_register[x_register_index]` (including `v_register[x_register_index]`) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified
    fn opcode_Fx65_load_v_registers(&mut self, register_index: usize) {
        let offset = self.i_register as usize;
        self.v_register[..register_index]
            .copy_from_slice(&self.memory[offset..offset + register_index]);
    }
}

fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    use std::{fs::File, io::Read};
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

#[test]
fn binary_coded_decimal() {
    let mut memory = [0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let value = 173u8;

    let i_register = 10usize;
    let address = i_register..=i_register + 2;

    let decimal_digits = [
        (value / 100) % 10, //
        (value / 10) % 10,
        (value / 1) % 10,
    ];

    memory[address].copy_from_slice(&decimal_digits);

    println!(
        "{}\n{:?}\n{:>2?}\n{:>2?}",
        value,
        decimal_digits,
        (0..20).collect::<Vec<_>>(),
        memory
    )
}
