use crate::nibbles::{
    concatenate_three_nibbles, concatenate_two_nibbles, get_first_nibble, get_second_nibble,
};
use std::time::{Duration, Instant};

#[cfg(test)]
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
#[cfg(test)]
use std::io::Write;

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

#[derive(Default)]
pub struct InterpreterConfig {
    pub instructions_per_second: f64,
    pub display_update_rate_hz: u64,
    pub key_held_plays_sound: bool,
    pub use_assembly_subroutines: bool,
    pub use_variable_offset: bool,
    pub increment_on_store: bool,
}

impl InterpreterConfig {
    pub fn builder() -> InterpreterConfigBuilder {
        InterpreterConfigBuilder::default()
    }
}

pub struct InterpreterConfigBuilder {
    instructions_per_second: f64,
    display_update_rate_hz: u64,
    key_held_plays_sound: bool,
    use_assembly_subroutines: bool,
    use_variable_offset: bool,
    increment_on_store: bool,
}

impl Default for InterpreterConfigBuilder {
    fn default() -> Self {
        InterpreterConfigBuilder {
            instructions_per_second: 700.0, // default
            display_update_rate_hz: 60,     // default
            // other flags default to false
            ..Self {
                instructions_per_second: 700.0,
                display_update_rate_hz: 60,
                key_held_plays_sound: false,
                use_assembly_subroutines: false,
                use_variable_offset: false,
                increment_on_store: false,
            }
        }
    }
}

impl InterpreterConfigBuilder {
    pub fn instructions_per_second(mut self, value: f64) -> Self {
        self.instructions_per_second = value;
        self
    }

    pub fn display_update_rate_hz(mut self, value: u64) -> Self {
        self.display_update_rate_hz = value;
        self
    }

    pub fn key_held_plays_sound(mut self, value: bool) -> Self {
        self.key_held_plays_sound = value;
        self
    }

    pub fn use_assembly_subroutines(mut self, value: bool) -> Self {
        self.use_assembly_subroutines = value;
        self
    }

    pub fn use_variable_offset(mut self, value: bool) -> Self {
        self.use_variable_offset = value;
        self
    }

    pub fn increment_on_store(mut self, value: bool) -> Self {
        self.increment_on_store = value;
        self
    }

    pub fn build(self) -> InterpreterConfig {
        InterpreterConfig {
            instructions_per_second: self.instructions_per_second,
            display_update_rate_hz: self.display_update_rate_hz,
            key_held_plays_sound: self.key_held_plays_sound,
            use_assembly_subroutines: self.use_assembly_subroutines,
            use_variable_offset: self.use_variable_offset,
            increment_on_store: self.increment_on_store,
        }
    }
}

/// The chip8 Interpreter that manages the state of a program.
///
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

    last_timer_tick: Instant,
    last_instruction_time: Instant,
    play_sound: bool,

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
    config: InterpreterConfig,
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
            last_timer_tick: Instant::now(),
            last_instruction_time: Instant::now(),
            play_sound: false,
            keypad: [false; 16],
            config: InterpreterConfig::default(),
        }
    }

    pub fn new_with_config(config: InterpreterConfig) -> Self {
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
            last_timer_tick: Instant::now(),
            last_instruction_time: Instant::now(),
            play_sound: false,
            keypad: [false; 16],
            config,
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
    pub fn display(&self) -> &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
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
    pub fn keypad_mut(&mut self) -> &mut [bool; 16] {
        &mut self.keypad
    }

    /// The timing and operation of the timers
    /// are completely separate from the fetch-decode-execute cycle.
    fn update_timers(&mut self) {
        let timer_interval = Duration::from_millis(16);
        if self.last_timer_tick.elapsed() >= timer_interval {
            self.last_timer_tick = Instant::now();

            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.play_sound = true;
                self.sound_timer -= 1;
            } else {
                self.play_sound = false;
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

        /// These are generated from the config options.
        let instruction_delay = Duration::from_nanos(((1.0 / InterpreterConfig::builder().instructions_per_second) * 1e9) as u64);
        let display_update_duration = Duration::from_nanos(1_000_000_000 / InterpreterConfig::builder().display_update_rate_hz);

        if instruction_duration < instruction_delay {
            std::thread::sleep(instruction_delay - instruction_duration);
        }

        true
    }
}

#[cfg(test)]
impl Interpreter {
    pub fn execute_program_terminal(&mut self) -> Result<(), std::io::Error> {
        // prepare the terminal
        let mut stdout = std::io::stdout();
        terminal::enable_raw_mode()?;
        stdout
            .execute(Hide)?
            .execute(Clear(ClearType::All))?
            .execute(MoveTo(0, 0))?;

        loop {
            // handle input
            self.keypad = [false; 16];
            if event::poll(Duration::from_nanos(1))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            break
                        }
                        KeyCode::Char('1') => self.keypad[0x0] = true,
                        KeyCode::Char('2') => self.keypad[0x1] = true,
                        KeyCode::Char('3') => self.keypad[0x2] = true,
                        KeyCode::Char('4') => self.keypad[0x3] = true,
                        KeyCode::Char('q') => self.keypad[0x4] = true,
                        KeyCode::Char('w') => self.keypad[0x5] = true,
                        KeyCode::Char('e') => self.keypad[0x6] = true,
                        KeyCode::Char('r') => self.keypad[0x7] = true,
                        KeyCode::Char('a') => self.keypad[0x8] = true,
                        KeyCode::Char('s') => self.keypad[0x9] = true,
                        KeyCode::Char('d') => self.keypad[0xA] = true,
                        KeyCode::Char('f') => self.keypad[0xB] = true,
                        KeyCode::Char('z') => self.keypad[0xC] = true,
                        KeyCode::Char('x') => self.keypad[0xD] = true,
                        KeyCode::Char('c') => self.keypad[0xE] = true,
                        KeyCode::Char('v') => self.keypad[0xF] = true,
                        _ => {}
                    }
                }
            }

            // print display
            for (y, row) in self.display.iter().enumerate() {
                stdout.execute(MoveTo(0, y as u16))?;
                for pixel in row.iter().map(|&pixel| if pixel { "█" } else { " " }) {
                    stdout.write_all(pixel.as_bytes())?;
                }
            }
            stdout.execute(MoveTo(0, DISPLAY_HEIGHT as u16));

            // execute instruction
            if !self.execute_current_instruction() {
                break;
            }
        }

        // reset the terminal
        stdout.execute(Show)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }
}
