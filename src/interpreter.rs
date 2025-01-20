pub use crate::interpreter::builder::{ConfigurationBuilder, Configuration};
use crate::nibbles::{
    concatenate_three_nibbles, concatenate_two_nibbles, get_first_nibble, get_second_nibble,
};
use std::time::{Duration, Instant};

pub mod builder;
mod instructions;

/// The chip8 Interpreter that manages the state of a program.
#[derive(Debug)]
pub struct Interpreter {
    configuration: Configuration,
    memory: Box<[u8]>,

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

    /// Using a simple PRNG in the Cxkk instruction.
    /// This state is used to generate random numbers.
    random_state: usize,

    /// `false` represents a black pixel. `true` represents a white pixel
    display: Box<[Box<[bool]>]>,

    last_timer_tick: Instant,
    last_instruction_time: Instant,

    /// A collection of four rows. `true` represents a pressed button. `false` represents a unpressed button
    /// ```text
    /// keypad
    /// ╔═══╦═══╦═══╦═══╗
    /// ║ 1 ║ 2 ║ 3 ║ C ║
    /// ╠═══╬═══╬═══╬═══╣
    /// ║ 4 ║ 5 ║ 6 ║ D ║
    /// ╠═══╬═══╬═══╬═══╣
    /// ║ 7 ║ 8 ║ 9 ║ E ║
    /// ╠═══╬═══╬═══╬═══╣
    /// ║ A ║ 0 ║ B ║ F ║
    /// ╚═══╩═══╩═══╩═══╝
    ///
    /// indexes
    /// ╔═══╦═══╦═══╦═══╗
    /// ║ 0 ║ 1 ║ 2 ║ 3 ║
    /// ╠═══╬═══╬═══╬═══╣
    /// ║ 4 ║ 5 ║ 6 ║ 7 ║
    /// ╠═══╬═══╬═══╬═══╣
    /// ║ 8 ║ 9 ║ A ║ B ║
    /// ╠═══╬═══╬═══╬═══╣
    /// ║ C ║ D ║ E ║ F ║
    /// ╚═══╩═══╩═══╩═══╝
    /// ```
    keypad: [bool; 16],
}

// initialization
impl Default for Interpreter {
    fn default() -> Self {
        Self::builder().build()
    }
}
impl Interpreter {
    pub const fn builder() -> ConfigurationBuilder {
        ConfigurationBuilder::new()
    }
    pub fn load_program_from_path(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), std::io::Error> {
        let program_data = std::fs::read(path)?;
        self.load_program_from_bytes(program_data);
        Ok(())
    }
    pub fn load_program_from_bytes(&mut self, program_data: impl AsRef<[u8]>) {
        let program_data = program_data.as_ref();
        let program_size = program_data.len();
        let program_start = self.configuration.program_start();

        self.memory[program_start..program_start + program_size].copy_from_slice(program_data);
    }
}

// accessors
impl Interpreter {
    pub const fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    pub fn display(&self) -> &[Box<[bool]>] {
        &self.display
    }

    /// Returns an array contain the four nibbles of an opcode.
    /// (a nibble is a four bit number or single hexadecimal digit)
    fn get_current_instruction(&self) -> Option<[u8; 4]> {
        let program_counter = self.program_counter as usize;

        let most_significant_byte = self.memory.get(program_counter)?;
        let least_significant_byte = self.memory.get(program_counter + 1)?;

        let nibbles = [
            get_first_nibble(*most_significant_byte),
            get_second_nibble(*most_significant_byte),
            get_first_nibble(*least_significant_byte),
            get_second_nibble(*least_significant_byte),
        ];

        Some(nibbles)
    }
}

// mutators
impl Interpreter {
    pub const fn keypad_mut(&mut self) -> &mut [bool; 16] {
        &mut self.keypad
    }

    /// The timing and operation of the timers
    /// are completely separate from the fetch-decode-execute cycle.
    fn update_timers(&mut self) {
        // We want to decrement our timers once every ~16.67ms (1/60s).
        const TIMER_INTERVAL: Duration = Duration::from_nanos(16_666_667);
        if self.last_timer_tick.elapsed() >= TIMER_INTERVAL {
            self.last_timer_tick = Instant::now();

            // Decrement delay timer if > 0
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            // Decrement sound timer if > 0, print "BEEP!!!"
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
                println!("BEEP!!!");
            }
        }
    }

    #[rustfmt::skip]
    pub fn execute_current_instruction(&mut self) -> bool {

        let Some(nibbles) = self.get_current_instruction() else {
            return false;
        };
        self.program_counter += 2;

        let address = concatenate_three_nibbles(nibbles[1], nibbles[2], nibbles[3]);
        let value = concatenate_two_nibbles(nibbles[2], nibbles[3]);
        let x_register_index = nibbles[1] as usize;
        let y_register_index = nibbles[2] as usize;
        let sprite_height = nibbles[3];

        match nibbles {
            [0x0, 0x0, 0xE, 0x0] => self.clear_display(),
            [0x0, 0x0, 0xE, 0xE] => self.return_subroutine(),
            [0x1,   _,   _,   _] => self.jump(address),
            [0x2,   _,   _,   _] => self.call_subroutine(address),
            [0x3,   _,   _,   _] => self.skip_if_equal_value(x_register_index, value),
            [0x4,   _,   _,   _] => self.skip_if_not_equal_value(x_register_index, value),
            [0x5,   _,   _, 0x0] => self.skip_if_equal(x_register_index, y_register_index),
            [0x6,   _,   _,   _] => self.assign_value(x_register_index, value),
            [0x7,   _,   _,   _] => self.add_assign_value(x_register_index, value),
            [0x8,   _,   _, 0x0] => self.assign(x_register_index, y_register_index),
            [0x8,   _,   _, 0x1] => self.bitwise_or_assign(x_register_index, y_register_index),
            [0x8,   _,   _, 0x2] => self.bitwise_and_assign(x_register_index, y_register_index),
            [0x8,   _,   _, 0x3] => self.bitwise_xor_assign(x_register_index, y_register_index),
            [0x8,   _,   _, 0x4] => self.add_assign(x_register_index, y_register_index),
            [0x8,   _,   _, 0x5] => self.sub_assign(x_register_index, y_register_index),
            [0x8,   _,   _, 0x6] => self.right_shift_assign(x_register_index, y_register_index),
            [0x8,   _,   _, 0x7] => self.sub_assign_swapped(x_register_index, y_register_index),
            [0x8,   _,   _, 0xE] => self.left_shift_assign(x_register_index, y_register_index),
            [0x9,   _,   _, 0x0] => self.skip_if_not_equal(x_register_index, y_register_index),
            [0xA,   _,   _,   _] => self.address_register_assign(address),
            [0xB,   _,   _,   _] => self.jump_offset(x_register_index, address),
            [0xC,   _,   _,   _] => self.random_number_assign(x_register_index, value),
            [0xD,   _,   _,   _] => self.draw_sprite(x_register_index, y_register_index, sprite_height),
            [0xE,   _, 0x9, 0xE] => self.skip_on_key_pressed(x_register_index),
            [0xE,   _, 0xA, 0x1] => self.skip_on_key_not_pressed(x_register_index),
            [0xF,   _, 0x0, 0x7] => self.store_delay_timer(x_register_index),
            [0xF,   _, 0x0, 0xA] => self.wait_for_key_press(x_register_index),
            [0xF,   _, 0x1, 0x5] => self.delay_timer_assign(x_register_index),
            [0xF,   _, 0x1, 0x8] => self.sound_timer_assign(x_register_index),
            [0xF,   _, 0x1, 0xE] => self.address_register_add_assign(x_register_index),
            [0xF,   _, 0x2, 0x9] => self.address_register_assign_character_address(x_register_index),
            [0xF,   _, 0x3, 0x3] => self.store_binary_coded_decimal_address(x_register_index),
            [0xF,   _, 0x5, 0x5] => self.store_variable_registers(x_register_index),
            [0xF,   _, 0x6, 0x5] => self.load_variable_registers(x_register_index),
            _ => {}
        }

        self.update_timers();

        let instruction_duration= self.last_instruction_time.elapsed();
        self.last_instruction_time = Instant::now();

        let instruction_delay = self.configuration.instruction_delay();
        if instruction_duration < instruction_delay {
            std::thread::sleep(instruction_delay - instruction_duration);
        }

        true
    }
}
