mod opcode;
mod error;
use self::{error::Chip8Error, opcode::Opcode};

/*
memory map
0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
0x200-0xFFF - Program ROM and work RAM
*/

pub struct Chip8 {
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: u16,
    current_opcode: Opcode,
    program_counter: u16,
    index_register: u16,
    v_register: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    pixels: [u8; 64 * 32], // 2048
    key_pad: [bool; 16],
}
impl Chip8 {
    pub fn new(program: &[u8]) -> Result<Self, Chip8Error> {
        let mut chip8 = Chip8 {
            program_counter: 0,
            current_opcode: Opcode::NoOp,
            index_register: 0,
            stack_pointer: 0,
            v_register: [0; 16],
            memory: [0; 4096],
            stack: [0; 16],
            pixels: [0; 64 * 32],
            key_pad: [false; 16],
            delay_timer: 0,
            sound_timer: 0,
        };

        chip8.load_font();
        chip8.load_program(program)?;

        Ok(chip8)
    }
    fn load_font(&mut self) {
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
        for (i, font_byte) in FONT.iter().enumerate() {
            self.memory[i] = *font_byte;
        }
    }
    fn load_program(&mut self, program: &[u8]) -> Result<(), Chip8Error> {
        const PROGRAM_MEMORY_OFFSET: usize = 512;
        if program.len() > self.memory.len() - PROGRAM_MEMORY_OFFSET {
            return Err(Chip8Error::ProgramTooLarge)
        }
        
        for (i, program_byte) in program.iter().enumerate() {
            self.memory[PROGRAM_MEMORY_OFFSET + i] = *program_byte;
        }

        Ok(())
    }
}
impl Chip8 {
    pub fn emulate_cycle(&mut self) -> Result<(), Chip8Error> {
        
        self.update_opcode();

        self.execute_opcode()?;

        self.update_delay_timer();

        self.update_sound_timer();

        Ok(())
    }
    fn update_opcode(&mut self) {
        if let Some(raw_opcode) = self.memory.get(self.program_counter as usize .. self.program_counter as usize + 1) {
            self.current_opcode = raw_opcode.into();
        } else {
            self.current_opcode = Opcode::NoOp;
        }
    }
    fn execute_opcode(&mut self) -> Result<(), Chip8Error> {
        match self.current_opcode {
            _ => {}
        }
        Ok(())
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
