use super::Interpreter;
use std::time::{Duration, Instant};

/// Offset is commonly done because of old standards.
/// Most programs written for Chip8 expect programs to start here.
pub const DEFAULT_PROGRAM_START: usize = 0x200;
pub const DEFAULT_DISPLAY_WIDTH: usize = 64;
pub const DEFAULT_DISPLAY_HEIGHT: usize = 32;
pub const DEFAULT_BLACK_DISPLAY: [[bool; DEFAULT_DISPLAY_WIDTH]; DEFAULT_DISPLAY_HEIGHT] =
    [[false; DEFAULT_DISPLAY_WIDTH]; DEFAULT_DISPLAY_HEIGHT];
pub const DEFAULT_FONT_DATA: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];
pub const DEFAULT_FONT_DATA_START: usize = 0x50;
pub const DEFAULT_FONT_DATA_END: usize = 0x9F;
pub const DEFAULT_INSTRUCTION_DELAY: Duration = Duration::from_nanos(((1.0 / 700.0) * 1e9) as u64);
pub const DEFAULT_MEMORY_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, macros::CopyGetters, macros::Builder)]
pub struct Configuration {
    instruction_delay: Duration,
    memory_size: usize,
    key_held_plays_sound: bool,
    use_assembly_routine: bool,
    use_variable_offset: bool,
    increment_on_store: bool,
    program_start: usize,
    display_width: usize,
    display_height: usize,
    font_data: [u8; 80],
    font_data_start: usize,
    font_data_end: usize,
}
impl Configuration {
    pub const fn new() -> Self {
        Self {
            instruction_delay: DEFAULT_INSTRUCTION_DELAY,
            key_held_plays_sound: true,
            use_assembly_routine: false,
            use_variable_offset: true,
            increment_on_store: false,
            program_start: DEFAULT_PROGRAM_START,
            display_width: DEFAULT_DISPLAY_WIDTH,
            display_height: DEFAULT_DISPLAY_HEIGHT,
            font_data: DEFAULT_FONT_DATA,
            font_data_start: DEFAULT_FONT_DATA_START,
            font_data_end: DEFAULT_FONT_DATA_END,
            memory_size: DEFAULT_MEMORY_SIZE,
        }
    }
}
impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}
impl ConfigurationBuilder {
    pub const fn new() -> Self {
        Self(Configuration::new())
    }
    pub fn build(self) -> Interpreter {
        let mut memory = vec![0; self.0.memory_size].into_boxed_slice();
        memory[self.0.font_data_start..=self.0.font_data_end].copy_from_slice(&self.0.font_data);

        Interpreter {
            memory,
            program_counter: self.0.program_start as u16,
            address_register: 0,
            variable_register: [0; 16],
            call_stack: [0; 16],
            call_stack_index: 0,
            delay_timer: 0,
            sound_timer: 0,
            last_timer_tick: Instant::now(),
            last_instruction_time: Instant::now(),
            random_state: 0x13275389,
            display: vec![
                vec![false; self.0.display_width].into_boxed_slice();
                self.0.display_height
            ]
            .into_boxed_slice(),
            keypad: [false; 16],
            configuration: self.0,
        }
    }
}
