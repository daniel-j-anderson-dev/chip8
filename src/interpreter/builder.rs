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

#[derive(Debug)]
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
impl Configuration {
    pub const fn instruction_delay(&self) -> Duration {
        self.instruction_delay
    }
    pub const fn memory_size(&self) -> usize {
        self.memory_size
    }
    pub const fn key_held_plays_sound(&self) -> bool {
        self.key_held_plays_sound
    }
    pub const fn use_assembly_routine(&self) -> bool {
        self.use_assembly_routine
    }
    pub const fn use_variable_offset(&self) -> bool {
        self.use_variable_offset
    }
    pub const fn increment_on_store(&self) -> bool {
        self.increment_on_store
    }
    pub const fn program_start(&self) -> usize {
        self.program_start
    }
    pub const fn display_width(&self) -> usize {
        self.display_width
    }
    pub const fn display_height(&self) -> usize {
        self.display_height
    }
    pub const fn font_data(&self) -> &[u8; 80] {
        &self.font_data
    }
    pub const fn font_data_start(&self) -> usize {
        self.font_data_start
    }
    pub const fn font_data_end(&self) -> usize {
        self.font_data_end
    }
}

#[derive(Debug, Default)]
pub struct Builder(Configuration);
impl Builder {
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
impl Builder {
    pub const fn instruction_delay(self, value: Duration) -> Self {
        Self(Configuration {
            instruction_delay: value,
            ..self.0
        })
    }
    pub const fn memory_size(self, value: usize) -> Self {
        Self(Configuration {
            memory_size: value,
            ..self.0
        })
    }
    pub const fn key_held_plays_sound(self, value: bool) -> Self {
        Self(Configuration {
            key_held_plays_sound: value,
            ..self.0
        })
    }
    pub const fn use_assembly_routine(self, value: bool) -> Self {
        Self(Configuration {
            use_assembly_routine: value,
            ..self.0
        })
    }
    pub const fn use_variable_offset(self, value: bool) -> Self {
        Self(Configuration {
            use_variable_offset: value,
            ..self.0
        })
    }
    pub const fn increment_on_store(self, value: bool) -> Self {
        Self(Configuration {
            increment_on_store: value,
            ..self.0
        })
    }
    pub const fn program_start(self, value: usize) -> Self {
        Self(Configuration {
            program_start: value,
            ..self.0
        })
    }
    pub const fn display_width(self, value: usize) -> Self {
        Self(Configuration {
            display_width: value,
            ..self.0
        })
    }
    pub const fn display_height(self, value: usize) -> Self {
        Self(Configuration {
            display_height: value,
            ..self.0
        })
    }
    pub const fn font_data(self, value: [u8; 80]) -> Self {
        Self(Configuration {
            font_data: value,
            ..self.0
        })
    }
    pub const fn font_data_start(self, value: usize) -> Self {
        Self(Configuration {
            font_data_start: value,
            ..self.0
        })
    }
    pub const fn font_data_end(self, value: usize) -> Self {
        Self(Configuration {
            font_data_end: value,
            ..self.0
        })
    }
}

#[test]
fn generate_builder_methods() {
    const SELF_SOURCE: &str = include_str!("./builder.rs");
    let fields_start = SELF_SOURCE.find("pub struct Configuration {\n").unwrap()
        + "pub struct Configuration {\n".len();
    let fields_end = SELF_SOURCE[fields_start..].find('}').unwrap() + fields_start;
    let fields = SELF_SOURCE[fields_start..fields_end].lines().map(|line| {
        let line = line.trim();
        let field_name_end = line.find(':').unwrap();
        let field_name = &line[..field_name_end];

        let field_type_start = field_name_end + 1;
        let field_type = &line[field_type_start..];

        (field_name, field_type)
    });

    println!("impl Builder {{");
    for (field_name, field_type) in fields {
        print!(
            "
pub const fn {field_name}(self, value: {field_type}) -> Self {{
    Self(Configuration{{
        {field_name}: value,
        ..self.0
    }})
}}"
        )
    }
    println!("}}");
}
