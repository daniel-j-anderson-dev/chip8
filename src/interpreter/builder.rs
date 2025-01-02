use std::time::Instant;

use super::Interpreter;

pub struct Configuration {
    pub instructions_per_second: usize,
    pub key_held_plays_sound: bool,
    pub use_assembly_routine: bool,
    pub use_variable_offset: bool,
    pub increment_on_store: bool,
    pub program_start: usize,
    pub display_width: usize,
    pub display_height: usize,
    pub font_data: [u8; 80],
    pub font_data_start: usize,
    pub font_data_end: usize,
}
impl Default for Configuration {
    fn default() -> Self {
        Self {
            instructions_per_second: 700,
            key_held_plays_sound: true,
            use_assembly_routine: false,
            use_variable_offset: true,
            increment_on_store: false,
            program_start: 0x200,
            display_width: 64,
            display_height: 32,
            font_data: [
                0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
                0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
                0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
                0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
                0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
                0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
            ],
            font_data_start: 0x50,
            font_data_end: 0x9F,
        }
    }
}

impl Configuration {
    pub fn build(self) -> Interpreter {
        let mut memory = [0; 4096];
        memory[self.font_data_start..=self.font_data_end].copy_from_slice(&self.font_data);

        Interpreter {
            memory,
            program_counter: self.program_start as u16,
            address_register: 0,
            variable_register: [0; 16],
            call_stack: [0; 16],
            call_stack_index: 0,
            delay_timer: 0,
            sound_timer: 0,
            random_state: 0x13275389,
            display: [[false; 64]; 32],
            last_timer_tick: Instant::now(),
            last_instruction_time: Instant::now(),
            keypad: [false; 16],
            configuration: self,
        }
    }
}

impl Configuration {
    pub fn instructions_per_second(self, value: usize) -> Self {
        Self {
            instructions_per_second: value,
            ..self
        }
    }

    pub fn key_held_plays_sound(self, value: bool) -> Self {
        Self {
            key_held_plays_sound: value,
            ..self
        }
    }

    pub fn use_assembly_routine(self, value: bool) -> Self {
        Self {
            use_assembly_routine: value,
            ..self
        }
    }

    pub fn use_variable_offset(self, value: bool) -> Self {
        Self {
            use_variable_offset: value,
            ..self
        }
    }

    pub fn increment_on_store(self, value: bool) -> Self {
        Self {
            increment_on_store: value,
            ..self
        }
    }

    pub fn program_start(self, value: usize) -> Self {
        Self {
            program_start: value,
            ..self
        }
    }

    pub fn display_width(self, value: usize) -> Self {
        Self {
            display_width: value,
            ..self
        }
    }

    pub fn display_height(self, value: usize) -> Self {
        Self {
            display_height: value,
            ..self
        }
    }

    pub fn font_data(self, value: [u8; 80]) -> Self {
        Self {
            font_data: value,
            ..self
        }
    }

    pub fn font_data_start(self, value: usize) -> Self {
        Self {
            font_data_start: value,
            ..self
        }
    }

    pub fn font_data_end(self, value: usize) -> Self {
        Self {
            font_data_end: value,
            ..self
        }
    }
}

#[test]
fn test_builder() {
    let chip8 = Interpreter::builder()
        .display_height(100)
        .display_height(200)
        .program_start(0x200)
        .build();
}

#[test]
fn generate_builder_methods() {
    let fields = include_str!("./builder.rs")
        .lines()
        .skip(1)
        .take(11)
        .filter_map(|line| {
            let line = line.trim();

            const FIELD_NAME_START: usize = 3;
            let field_name_end = line.find(':')?;
            let field_name = &line[FIELD_NAME_START..field_name_end];

            let field_type_start = field_name_end + 1;
            let field_type = &line[field_type_start..];

            Some((field_name, field_type))
        });

    for (field_name, field_type) in fields {
        println!(
            "
pub fn {field_name}(self, value: {field_type}) -> Self {{
    Self {{
        {field_name}: value,
        ..self
    }}
}}
"
        )
    }
}
