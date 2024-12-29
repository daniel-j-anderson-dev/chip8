use crate::nibbles::{
    concatenate_three_nibbles, concatenate_two_nibbles, get_first_nibble, get_second_nibble,
};

mod instructions;

/// Offset is commonly done because of old standards.
/// Most programs written for Chip8 expect programs to start here.
pub const PROGRAM_START: usize = 0x200;
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const BLACK_DISPLAY: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] =
    [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
pub const FONT_DATA: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];
pub const FONT_DATA_START: usize = 0x50;
pub const FONT_DATA_END: usize = 0x9F;

/// The chip8 Interpreter that manages the state of a program.
///
/// TODO: Configuration for the Chip8 interpreter.
/// These options do not exist yet, but will be useful
/// once we start implementing the options.
///
///     DISPLAY_UPDATE_RATE     = 60 Hz
///     KEY_HELD_PLAYS_SOUND    = true
///     RUN_SPEED               = 700 instructions per second
///     USE_ASSEMBLY_SUBROUTINE = false
///     USE_VARIABLE_OFFSET     = true
///     INCREMENT_ON_STORE      = false
pub struct Interpreter {
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

    /// A collection of four rows. `true` represents a pressed button. `false` represents a unpressed button
    /// ```text
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

// initialization
impl Interpreter {
    pub fn new() -> Interpreter {
        let mut memory = [0; 4096];
        memory[FONT_DATA_START..=FONT_DATA_END].copy_from_slice(&FONT_DATA);

        Self {
            memory,
            program_counter: PROGRAM_START as u16,
            address_register: 0,
            variable_register: [0; 16],
            call_stack: [0; 16],
            call_stack_index: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: BLACK_DISPLAY,
            keypad: [[false; 4]; 4],
        }
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

        self.memory[PROGRAM_START..PROGRAM_START + program_size].copy_from_slice(program_data);
    }
}

// accessors
impl Interpreter {
    pub fn display(&mut self) -> &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
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
    /// The timing and operation of the timers
    /// are completely separate from the fetch-decode-execute cycle.
    /// The logic will look a little something like this:
    ///
    /// ```text
    /// if > 0
    ///     decrement @ 60 Hz
    /// else
    ///     if sound_timer
    ///         play_sound()
    /// ```
    fn update_timers(&mut self) {
        // TODO
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
            [0xB,   _,   _,   _] => self.jump_offset(address),
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

        true
    }

    pub fn execute_program_stdout(&mut self) {
        unimplemented!()
    }
}
