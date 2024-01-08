/*
0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
0x200-0xFFF - Program ROM and work RAM
*/
pub struct Chip8 {
    current_opcode: u16,
    program_counter: u16,
    index_register: u16,
    v_register: [u8; 16],
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: u16,
    pixels: [u8; 2048],
    delay_timer: u8,
    sound_timer: u8,
    key_pad: [bool; 16],
}
impl Chip8 {
    fn new(program: &[u8]) -> Result<Self, String> {
        let mut chip8 = Chip8 {
            program_counter: 0,
            current_opcode: 0,
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
    fn load_program(&mut self, program: &[u8]) -> Result<(), String> {
        const PROGRAM_MEMORY_OFFSET: usize = 512;
        if program.len() > self.memory.len() - PROGRAM_MEMORY_OFFSET {
            return Err("The program exceeds chip8 program memory of 3584 bytes".to_string())
        }
        
        for (i, program_byte) in program.iter().enumerate() {
            self.memory[PROGRAM_MEMORY_OFFSET + i] = *program_byte;
        }

        Ok(())
    }
}
impl Chip8 {
    pub fn emulate_cycle(&mut self) {
        // fetch opcode
        self.current_opcode = combine_bytes(
            self.memory[self.program_counter as usize],
            self.memory[self.program_counter as usize + 1],
        );

        match self.current_opcode & 0xF000 {
            0xA000 => {
                self.index_register = self.current_opcode & 0x0FFF;
                self.program_counter += 2;
            }
            _ => {
                eprintln!("Unknown opcode: {:x}", self.current_opcode);
            }
        }
    }
}
fn combine_bytes(top_half: u8, bottom_half: u8) -> u16 {
    (top_half as u16) << 8 | bottom_half as u16
}
