use crate::interpreter::Chip8;

impl Chip8 {
    fn clear_screen(&mut self) {
        unimplemented!();
    }

    fn return_subroutine(&mut self) {
        unimplemented!();
    }

    fn jump(&mut self, address: u16) {
        unimplemented!();
    }

    fn call_subroutine(&mut self, address: u16) {
        unimplemented!();
    }

    /// Skips the next instruction if
    /// `variable_register[register_index]` is equal to last byte of the opcode
    fn skip_if_equal_value(&mut self, register_index: usize, value: u8) {
        unimplemented!();
    }

    /// Skips the next instruction if
    /// `variable_register[register_index]` is NOT equal to last byte of the opcode
    fn skip_if_not_equal_value(&mut self, register_index: usize, value: u8) {
        unimplemented!();
    }

    /// Skips the next instruction if
    /// `variable_register[x_register_index]` equals `variable_register[y_register_index]`
    fn skip_if_equal(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Sets `variable_register[register_index]` to value
    fn assign_value(&mut self, register_index: usize, value: u8) {
        unimplemented!();
    }

    /// Adds value to `variable_register[register_index]` (carry flag is not changed)
    fn add_assign_value(&mut self, register_index: usize, value: u8) {
        unimplemented!();
    }

    /// Sets `variable_register[x_register_index]` to the value of `variable_register[y_register_index]`
    fn assign(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Sets `variable_register[x_register_index]` to (`variable_register[x_register_index]` or `variable_register[y_register_index]`) bitwise
    fn bitwise_or(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Sets `variable_register[x_register_index]` to `variable_register[x_register_index]` and `variable_register[y_register_index]` (bitwise)
    fn bitwise_and(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Sets `variable_register[x_register_index]` to `variable_register[x_register_index]` xor `variable_register[y_register_index]` (bitwise)
    fn bitwise_xor(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Adds `variable_register[y_register_index]` to `variable_register[x_register_index]`. `variable_register[0xF]` is set to 1 when there's an overflow, and to 0 when there is not
    fn add_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// `variable_register[y_register_index]` is subtracted from `variable_register[x_register_index]`. `variable_register[0xF]` is set to 0 when there's an underflow, and 1 when there is not
    fn sub_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Stores the least significant bit of `variable_register[x_register_index]` in `variable_register[0xF]` and then shifts `variable_register[x_register_index]` to the right by 1
    fn right_shift_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Sets `variable_register[x_register_index]` to `variable_register[y_register_index]` minus `variable_register[x_register_index]`. `variable_register[0xF]` is set to 0 when there's an underflow, and 1 when there is not
    fn sub_assign_swapped(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Stores the most significant bit of `variable_register[x_register_index]` in `variable_register[0xF]` and then shifts `variable_register[x_register_index]` to the left by 1
    fn left_shift_assign(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Skips the next instruction if `variable_register[x_register_index]` does not equal `variable_register[y_register_index]`
    fn skip_if_not_equal(&mut self, x_register_index: usize, y_register_index: usize) {
        unimplemented!();
    }

    /// Sets address_register to the address address
    fn set_address_register(&mut self, address: u16) {
        unimplemented!();
    }

    /// Jumps to the address address plus variable_register[0]
    fn jump_offset(&mut self, address: u16) {
        unimplemented!();
    }

    /// Sets `variable_register[register_index]` to the result of a bitwise and operation on a random number (Typically: 0 to 255) and value
    fn random_number_assign(&mut self, register_index: usize, value: u8) {
        unimplemented!();
    }

    fn draw_sprite(&mut self, x_register_index: usize, y_register_index: usize, n: u8) {
        unimplemented!();
    }

    /// Skips the next instruction if the key stored in `variable_register[register_index]` is pressed
    fn skip_on_key_pressed(&mut self, register_index: usize) {
        unimplemented!();
    }

    /// Skips the next instruction if the key stored in `variable_register[register_index]` is NOT pressed
    fn skip_on_key_not_pressed(&mut self, register_index: usize) {
        unimplemented!();
    }

    /// Sets `variable_register[register_index]` to the value of the delay timer
    fn store_delay_timer(&mut self, register_index: usize) {
        unimplemented!();
    }

    /// A key press is awaited, and then stored in `variable_register[register_index]`
    fn wait_for_key_press(&mut self, register_index: usize) {
        unimplemented!();
    }

    /// Sets the delay timer to `variable_register[register_index]`
    fn set_delay_timer(&mut self, register_index: usize) {
        unimplemented!();
    }

    /// Sets the sound timer to `variable_register[register_index]`
    fn set_sound_timer(&mut self, register_index: usize) {
        unimplemented!();
    }

    /// Adds `variable_register[register_index]` to address_register. `variable_register[0xF]` is not affected.
    fn address_register_add_assign(&mut self, register_index: usize) {
        unimplemented!();
    }

    /// Sets address_register to the location of the sprite for the character in `variable_register[register_index]`
    /// Font starts at memory address 0
    fn set_address_register_to_font_address(&mut self, register_index: usize) {
        unimplemented!();
    }

    /// Stores the binary-coded decimal representation of variable_register[register_index], with the hundreds digit in memory at location in address_register, the tens digit at location address_register+1, and the ones digit at location address_register+2
    fn store_binary_coded_decimal_at_address_register(&mut self, register_index: usize) {
        unimplemented!();
    }

    /// Stores from `variable_register[0]` to `variable_register[register_index]` (including `variable_register[register_index]`) in memory, starting at address address_register. The offset from address_register is increased by 1 for each value written, but address_register itself is left unmodified
    fn store_variable_registers(&mut self, register_index: usize) {
        unimplemented!();
    }

    /// Fills from `variable_register[0]` to `variable_register[x_register_index]` (including `variable_register[x_register_index]`) with values from memory, starting at address address_register. The offset from address_register is increased by 1 for each value read, but address_register itself is left unmodified
    fn load_variable_registers(&mut self, register_index: usize) {
        unimplemented!();
    }
}