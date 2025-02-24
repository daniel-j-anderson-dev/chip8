use crate::interpreter::Interpreter;

impl Interpreter {
    /// Opcode: 00E0
    ///
    /// Clears the display.
    pub(super) fn clear_display(&mut self) {
        self.display.iter_mut().flatten().for_each(|p| *p = false);
    }

    /// Opcode: 00EE
    ///
    /// Return from a subroutine.
    pub(super) fn return_subroutine(&mut self) {
        self.program_counter = self.call_stack[self.call_stack_index];
        self.call_stack_index -= 1;
    }

    /// Opcode: 1nnn
    ///
    /// Assigns `program_counter` to given address.
    pub(super) fn jump(&mut self, address: u16) {
        self.program_counter = address;
    }

    /// Opcode: 2nnn
    ///
    /// Calls subroutine at given address.
    pub(super) fn call_subroutine(&mut self, address: u16) {
        self.call_stack_index += 1;
        self.call_stack[self.call_stack_index] = self.program_counter;
        self.program_counter = address;
    }

    /// Opcode: 3xkk
    ///
    /// Skips the next instruction if
    /// `VX` is equal to last byte of the instruction
    pub(super) fn skip_if_equal_value(&mut self, x_register_index: usize, value: u8) {
        if self.variable_register[x_register_index] == value {
            self.program_counter += 2;
        }
    }

    /// Opcode: 4xkk
    ///
    /// Skips the next instruction if
    /// `VX` is NOT equal to last byte of the instruction
    pub(super) fn skip_if_not_equal_value(&mut self, x_register_index: usize, value: u8) {
        if self.variable_register[x_register_index] != value {
            self.program_counter += 2;
        }
    }

    /// Opcode: 5xy0
    ///
    /// Skips the next instruction if
    /// `VX` equals `VY`
    pub(super) fn skip_if_equal(&mut self, x_register_index: usize, y_register_index: usize) {
        if self.variable_register[x_register_index] == self.variable_register[y_register_index] {
            self.program_counter += 2;
        }
    }

    /// Opcode: 6xkk
    ///
    /// Sets `VX` to `value`
    pub(super) fn assign_value(&mut self, x_register_index: usize, value: u8) {
        self.variable_register[x_register_index] = value;
    }

    /// Opcode: 7xkk
    ///
    /// Adds `value` to `VX` (carry flag is not changed)
    pub(super) fn add_assign_value(&mut self, x_register_index: usize, value: u8) {
        self.variable_register[x_register_index] =
            self.variable_register[x_register_index].wrapping_add(value);
    }

    /// Opcode: 8xy0
    ///
    /// Sets `VX` to the `value` of `VY`
    pub(super) fn assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.variable_register[x_register_index] = self.variable_register[y_register_index];
    }

    /// Opcode: 8xy1
    ///
    /// Sets `VX` to (`VX` or `VY`) bitwise
    pub(super) fn bitwise_or_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.variable_register[x_register_index] |= self.variable_register[y_register_index];
    }

    /// Opcode: 8xy2
    ///
    /// Sets `VX` to `VX` and `VY` (bitwise)
    pub(super) fn bitwise_and_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.variable_register[x_register_index] &= self.variable_register[y_register_index];
    }

    /// Opcode: 8xy3
    ///
    /// Sets `VX` to `VX` xor `VY` (bitwise)
    pub(super) fn bitwise_xor_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.variable_register[x_register_index] ^= self.variable_register[y_register_index];
    }

    /// Opcode: 8xy4
    ///
    /// Adds `VY` to `VX`. `variable_register[0xF]` is set to 1 when there's an overflow, and to 0 when there is not
    pub(super) fn add_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        let (sum, overflow) = self.variable_register[x_register_index]
            .overflowing_add(self.variable_register[y_register_index]);
        self.variable_register[0xF] = if overflow { 1 } else { 0 };
        self.variable_register[x_register_index] = sum;
    }

    /// Opcode: 8xy5
    ///
    /// `VY` is subtracted from `VX`. `variable_register[0xF]` is set to 0 when there's an underflow, and 1 when there is not
    pub(super) fn sub_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        let (difference, borrow) = self.variable_register[x_register_index]
            .overflowing_sub(self.variable_register[y_register_index]);
        self.variable_register[0xF] = if borrow { 0 } else { 1 };
        self.variable_register[x_register_index] = difference;
    }

    /// Opcode: 8xy6
    ///
    /// Stores the least significant bit of `VX` in `variable_register[0xF]` and then shifts `VX` to the right by 1
    pub(super) fn right_shift_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.variable_register[0xF] = self.variable_register[x_register_index] & 0b00000001;
        self.variable_register[x_register_index] >>= 1;
    }

    /// Opcode: 8xy7
    ///
    /// Sets `VX` to `VY` minus `VX`. `variable_register[0xF]` is set to 0 when there's an underflow, and 1 when there is not
    pub(super) fn sub_assign_swapped(&mut self, x_register_index: usize, y_register_index: usize) {
        let (difference, borrow) = self.variable_register[y_register_index]
            .overflowing_sub(self.variable_register[x_register_index]);
        self.variable_register[0xF] = if borrow { 0 } else { 1 };
        self.variable_register[x_register_index] = difference;
    }

    /// Opcode: 8xyE
    ///
    /// Stores the most significant bit of `VX` in `variable_register[0xF]` and then shifts `VX` to the left by 1
    pub(super) fn left_shift_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.variable_register[0xF] = (self.variable_register[x_register_index] & 0b10000000) >> 7;
        self.variable_register[x_register_index] <<= 1;
    }

    /// Opcode: 9xy0
    ///
    /// Skips the next instruction if `VX` does not equal `VY`
    pub(super) fn skip_if_not_equal(&mut self, x_register_index: usize, y_register_index: usize) {
        if self.variable_register[x_register_index] != self.variable_register[y_register_index] {
            self.program_counter += 2;
        }
    }

    /// Opcode: Annn
    ///
    /// Sets `I` to the address address
    pub(super) fn address_register_assign(&mut self, address: u16) {
        self.address_register = address
    }

    /// Opcode: Bxnn
    ///
    /// Jumps to the address address plus `VX`
    /// Bnnn: option to add from `V0` only.
    pub(super) fn jump_offset(&mut self, x_register_index: usize, address: u16) {
        self.program_counter = address + self.variable_register[x_register_index] as u16;
    }

    /// Opcode: Cxkk
    ///
    /// Sets `VX` to the result of a bitwise and operation on a random number (Typically: 0 to 255) and `KK`
    pub(super) fn random_number_assign(&mut self, x_register_index: usize, value: u8) {
        self.random_state ^= self.random_state << 13;
        self.random_state ^= self.random_state >> 17;
        self.random_state ^= self.random_state << 5;
        let random_number: u8 = (self.random_state & 0xFF) as u8;

        self.variable_register[x_register_index] = random_number & value;
    }

    /// Opcode: Dxyn
    ///
    /// Draws a sprite at coordinate (`VX`, `VY`) that has a width of 8 pixels and a height of `sprite_height` pixels. Each row of 8 pixels is read as bit-coded starting from memory location ``I``; `variable_register[0xF]` is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen
    pub(super) fn draw_sprite(
        &mut self,
        x_register_index: usize,
        y_register_index: usize,
        sprite_height: u8,
    ) {
        let display_width = self.configuration.display_width();
        let display_height = self.configuration.display_height();
        let address_register = self.address_register as usize;
        let x_position = self.variable_register[x_register_index] as usize % display_width;
        let y_position = self.variable_register[y_register_index] as usize % display_height;

        self.variable_register[0xF] = 0;

        for sprite_row_index in 0..sprite_height as usize {
            let display_row_index = (y_position + sprite_row_index) % display_height;

            let sprite_byte = self.memory[address_register + sprite_row_index];

            for sprite_column_index in 0..8 {
                let display_column_index = (x_position + sprite_column_index) % display_width;

                let pixel_bitmask = 0b10000000 >> sprite_column_index as u8;
                let sprite_pixel = (sprite_byte & pixel_bitmask) > 0;

                let display_pixel = &mut self.display[display_row_index][display_column_index];

                if sprite_pixel {
                    if *display_pixel {
                        self.variable_register[0xF] = 1;
                    }

                    *display_pixel ^= true;
                }
            }
        }
    }

    /// Opcode: Ex9E
    ///
    /// Skips the next instruction if the key stored in `VX` is pressed
    pub(super) fn skip_on_key_pressed(&mut self, x_register_index: usize) {
        let key = self.variable_register[x_register_index] as usize;

        if self.keypad[key] {
            self.program_counter += 2;
        }
    }

    /// Opcode: ExA1
    ///
    /// Skips the next instruction if the key stored in `VX` is NOT pressed
    pub(super) fn skip_on_key_not_pressed(&mut self, x_register_index: usize) {
        let key = self.variable_register[x_register_index] as usize;

        if !self.keypad[key] {
            self.program_counter += 2;
        }
    }

    /// Opcode: Fx07
    ///
    /// Sets `VX` to the `value` of the `delay_timer`
    pub(super) fn store_delay_timer(&mut self, x_register_index: usize) {
        self.variable_register[x_register_index] = self.delay_timer;
    }

    /// Opcode: Fx0A
    ///
    /// A key press is awaited, and then stored in `VX`
    pub(super) fn wait_for_key_press(&mut self, x_register_index: usize) {
        match self
            .keypad
            .iter()
            .position(|&is_key_pressed| is_key_pressed)
        {
            Some(key) => self.variable_register[x_register_index] = key as u8,
            None => self.program_counter -= 2,
        }
    }

    /// Opcode: Fx15
    ///
    /// Sets the `delay_timer` to `VX`
    pub(super) fn delay_timer_assign(&mut self, x_register_index: usize) {
        self.delay_timer = self.variable_register[x_register_index];
    }

    /// Opcode: Fx18
    ///
    /// Sets the `sound_timer` to `VX`
    pub(super) fn sound_timer_assign(&mut self, x_register_index: usize) {
        self.sound_timer = self.variable_register[x_register_index];
    }

    /// Opcode: Fx1E
    ///
    /// Adds `VX` to `I`. `VF` is not affected.
    pub(super) fn address_register_add_assign(&mut self, x_register_index: usize) {
        self.address_register += self.variable_register[x_register_index] as u16;
    }

    /// Opcode: Fx29
    ///
    /// Sets `I` to the location of the sprite for the character in `VX`
    /// Font starts at memory address 0
    pub(super) fn address_register_assign_character_address(&mut self, x_register_index: usize) {
        self.address_register = self.variable_register[x_register_index] as u16 & 0x0F;
    }

    /// Opcode: Fx33
    ///
    /// Stores the binary-coded decimal representation of variable_register[x_register_index], with the hundreds digit in memory at location in `I`, the tens digit at location `I`+1, and the ones digit at location `I`+2
    pub(super) fn store_binary_coded_decimal_address(&mut self, x_register_index: usize) {
        let mut accumulator = self.variable_register[x_register_index];

        self.memory[self.address_register as usize + 2] = accumulator % 10;
        accumulator /= 10;

        self.memory[self.address_register as usize + 1] = accumulator % 10;
        accumulator /= 10;

        self.memory[self.address_register as usize] = accumulator;
    }

    /// Opcode: Fx55
    ///
    /// Stores from `V0` to `VX` (including `VX`) in memory, starting at address `I`.
    /// The offset from `I` is increased by 1 for each value written, but `I` itself is left unmodified.
    pub(super) fn store_variable_registers(&mut self, x_register_index: usize) {
        let address = self.address_register as usize;

        for offset in 0..=x_register_index {
            self.memory[address + offset] = self.variable_register[offset];
        }
    }

    /// Opcode: Fx65
    ///
    /// Fills from `V0` to `VX` (including `VX`) with values from memory, starting at address `I`.
    /// The offset from `I` is increased by 1 for each value read, but `I` itself is left unmodified.
    pub(super) fn load_variable_registers(&mut self, x_register_index: usize) {
        let address = self.address_register as usize;

        for offset in 0..=x_register_index {
            self.variable_register[offset] = self.memory[address + offset];
        }
    }
}
