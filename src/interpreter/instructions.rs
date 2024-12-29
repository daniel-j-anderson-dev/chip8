use crate::interpreter::Interpreter;

impl Interpreter {

    /// Clears the display.
    pub(super) fn clear_display(&mut self) {
        self.display = Self::BLACK_DISPLAY;
    }

    /// Return from a subroutine.
    pub(super) fn return_subroutine(&mut self) {
        self.program_counter = self.call_stack[self.call_stack_index];
        self.call_stack_index -= 1;
    }

    /// Jumps to the given immediate address
    pub(super) fn jump(&mut self, address: u16) {
        self.program_counter = address;
    }

    pub(super) fn call_subroutine(&mut self, address: u16) {
        self.call_stack_index += 1;
        self.call_stack[self.call_stack_index] = self.program_counter;
        self.program_counter = address;
    }

    /// Skips the next instruction if
    /// `variable_register[x_register_index]` is equal to last byte of the instruction
    pub(super) fn skip_if_equal_value(&mut self, x_register_index: usize, value: u8) {
        if self.variable_register[x_register_index] == value {
            self.program_counter += 2;
        }
    }

    /// Skips the next instruction if
    /// `variable_register[x_register_index]` is NOT equal to last byte of the instruction
    pub(super) fn skip_if_not_equal_value(&mut self, x_register_index: usize, value: u8) {
        if self.variable_register[x_register_index] != value {
            self.program_counter += 2;
        }
    }

    /// Skips the next instruction if
    /// `variable_register[x_register_index]` equals `variable_register[y_register_index]`
    pub(super) fn skip_if_equal(&mut self, x_register_index: usize, y_register_index: usize) {
        if self.variable_register[x_register_index] == self.variable_register[y_register_index] {
            self.program_counter += 2;
        }
    }

    /// Sets `variable_register[x_register_index]` to value
    pub(super) fn assign_value(&mut self, x_register_index: usize, value: u8) {
        self.variable_register[x_register_index] = value;
    }

    /// Adds value to `variable_register[x_register_index]` (carry flag is not changed)
    pub(super) fn add_assign_value(&mut self, x_register_index: usize, value: u8) {
        self.variable_register[x_register_index] += value;
    }

    /// Sets `variable_register[x_register_index]` to the value of `variable_register[y_register_index]`
    pub(super) fn assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.variable_register[x_register_index] = self.variable_register[y_register_index];
    }

    /// Sets `variable_register[x_register_index]` to (`variable_register[x_register_index]` or `variable_register[y_register_index]`) bitwise
    pub(super) fn bitwise_or_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.variable_register[x_register_index] |= self.variable_register[y_register_index];
    }

    /// Sets `variable_register[x_register_index]` to `variable_register[x_register_index]` and `variable_register[y_register_index]` (bitwise)
    pub(super) fn bitwise_and_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.variable_register[x_register_index] &= self.variable_register[y_register_index];
    }

    /// Sets `variable_register[x_register_index]` to `variable_register[x_register_index]` xor `variable_register[y_register_index]` (bitwise)
    pub(super) fn bitwise_xor_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        self.variable_register[x_register_index] ^= self.variable_register[y_register_index];
    }

    /// Adds `variable_register[y_register_index]` to `variable_register[x_register_index]`. `variable_register[0xF]` is set to 1 when there's an overflow, and to 0 when there is not
    pub(super) fn add_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// `variable_register[y_register_index]` is subtracted from `variable_register[x_register_index]`. `variable_register[0xF]` is set to 0 when there's an underflow, and 1 when there is not
    pub(super) fn sub_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Stores the least significant bit of `variable_register[x_register_index]` in `variable_register[0xF]` and then shifts `variable_register[x_register_index]` to the right by 1
    pub(super) fn right_shift_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Sets `variable_register[x_register_index]` to `variable_register[y_register_index]` minus `variable_register[x_register_index]`. `variable_register[0xF]` is set to 0 when there's an underflow, and 1 when there is not
    pub(super) fn sub_assign_swapped(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Stores the most significant bit of `variable_register[x_register_index]` in `variable_register[0xF]` and then shifts `variable_register[x_register_index]` to the left by 1
    pub(super) fn left_shift_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Skips the next instruction if `variable_register[x_register_index]` does not equal `variable_register[y_register_index]`
    pub(super) fn skip_if_not_equal(&mut self, x_register_index: usize, y_register_index: usize) {
        if self.variable_register[x_register_index] != self.variable_register[y_register_index] {
            self.program_counter += 2;
        }
    }

    /// Sets address_register to the address address
    pub(super) fn address_register_assign(&mut self, address: u16) {
        self.address_register = address
    }

    /// Jumps to the address address plus variable_register[0]
    pub(super) fn jump_offset(&mut self, address: u16) {
        unimplemented!();
    }

    /// Sets `variable_register[x_register_index]` to the result of a bitwise and operation on a random number (Typically: 0 to 255) and value
    pub(super) fn random_number_assign(&mut self, x_register_index: usize, value: u8) {
        unimplemented!();
    }

    pub(super) fn draw_sprite(
        &mut self,
        x_register_index: usize,
        y_register_index: usize,
        sprite_height: u8,
    ) {
        let address_register = self.address_register as usize;
        let x_position = self.variable_register[x_register_index] as usize % Self::DISPLAY_WIDTH;
        let y_position = self.variable_register[y_register_index] as usize % Self::DISPLAY_HEIGHT;

        self.variable_register[0xF] = 0;

        for sprite_row_index in 0..sprite_height as usize {
            let display_row_index = y_position + sprite_row_index;

            let sprite_byte = self.memory[address_register + sprite_row_index];

            for sprite_column_index in 0..8 {
                let display_column_index = x_position + sprite_column_index;

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

    /// Skips the next instruction if the key stored in `variable_register[x_register_index]` is pressed
    pub(super) fn skip_on_key_pressed(&mut self, x_register_index: usize) {
        unimplemented!();
    }

    /// Skips the next instruction if the key stored in `variable_register[x_register_index]` is NOT pressed
    pub(super) fn skip_on_key_not_pressed(&mut self, x_register_index: usize) {
        unimplemented!();
    }

    /// Sets `variable_register[x_register_index]` to the value of the delay timer
    pub(super) fn store_delay_timer(&mut self, x_register_index: usize) {
        self.variable_register[x_register_index] = self.delay_timer;
    }

    /// A key press is awaited, and then stored in `variable_register[x_register_index]`
    pub(super) fn wait_for_key_press(&mut self, x_register_index: usize) {
        unimplemented!();
    }

    /// Sets the delay timer to `variable_register[x_register_index]`
    pub(super) fn delay_timer_assign(&mut self, x_register_index: usize) {
        self.delay_timer = self.variable_register[x_register_index];
    }

    /// Sets the sound timer to `variable_register[x_register_index]`
    pub(super) fn sound_timer_assign(&mut self, x_register_index: usize) {
        self.sound_timer = self.variable_register[x_register_index];
    }

    /// Adds `variable_register[x_register_index]` to address_register. `variable_register[0xF]` is not affected.
    pub(super) fn address_register_add_assign(&mut self, x_register_index: usize) {
        unimplemented!();
    }

    /// Sets address_register to the location of the sprite for the character in `variable_register[x_register_index]`
    /// Font starts at memory address 0
    pub(super) fn address_register_assign_character_address(&mut self, x_register_index: usize) {
        unimplemented!();
    }

    /// Stores the binary-coded decimal representation of variable_register[x_register_index], with the hundreds digit in memory at location in address_register, the tens digit at location address_register+1, and the ones digit at location address_register+2
    pub(super) fn store_binary_coded_decimal_address(&mut self, x_register_index: usize) {
        unimplemented!();
    }

    /// Stores from `variable_register[0]` to `variable_register[x_register_index]` (including `variable_register[x_register_index]`) in memory, starting at address address_register. The offset from address_register is increased by 1 for each value written, but address_register itself is left unmodified
    pub(super) fn store_variable_registers(&mut self, x_register_index: usize) {
        unimplemented!();
    }

    /// Fills from `variable_register[0]` to `variable_register[x_register_index]` (including `variable_register[x_register_index]`) with values from memory, starting at address address_register. The offset from address_register is increased by 1 for each value read, but address_register itself is left unmodified
    pub(super) fn load_variable_registers(&mut self, x_register_index: usize) {
        unimplemented!();
    }
}
