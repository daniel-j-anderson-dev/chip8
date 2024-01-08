pub enum Opcode {
    NoOp,
    ClearScreen,
    Return,
    Jump {
        address: u16
    },
    JumpWithOffset {
        address: u16
    },
    Call{
        address: u16
    },
    SkipIfRegisterEqualTo {
        register: usize,
        value: u16,
    },
    SkipIfRegisterNotEqualTo {
        register: usize,
        value: u16,    
    },
    SkipIfRegistersEqual {
        lhs_register: usize,
        rhs_register: usize,
    },
    SkipIfRegistersNotEqual {
        lhs_register: usize,
        rhs_register: usize,
    },
    AssignValue {
        register: usize,
        value: u16,
    },
    Assign {
        lhs_register: usize,
        rhs_register: usize,
    },
    OrAssign {
        lhs_register: usize,
        rhs_register: usize,
    },
    AndAssign {
        lhs_register: usize,
        rhs_register: usize,
    },
    XorAssign {
        lhs_register: usize,
        rhs_register: usize,
    },
    AddAssign {
        lhs_register: usize,
        rhs_register: usize,
    },
    AddAssignValue {
        register: usize,
        value: u16,
    },
    SubtractAssign {
        lhs_register: usize,
        rhs_register: usize,
    },
    SubtractAssignReversed {
        lhs_register: usize,
        rhs_register: usize,
    },
    RightShiftAssign {
        lhs_register: usize,
        rhs_register: usize,
    },
    LeftShiftAssign {
        lhs_register: usize,
        rhs_register: usize, 
    },
    SetIRegister {
        value: u16,    
    },
    Random {
        register: usize
    },
    Draw {
        x_register: usize,
        y_register: usize,
        height: u8,
    }
}
impl From<u16> for Opcode {
    fn from(raw_opcode: u16) -> Self {
        let opcode_hex_digits: [u8; 4] = [
            ((raw_opcode & 0xF000) >> 12) as u8,
            ((raw_opcode & 0x0F00) >> 8) as u8,
            ((raw_opcode & 0x00F0) >> 4) as u8,
            (raw_opcode & 0x000F) as u8,
        ];

        let x = (raw_opcode & 0x0F00) as usize; // second nybls
        let y = (raw_opcode & 0x00F0) as usize; // third nybls
        let nn = raw_opcode & 0x00FF; // last byte
        let nnn = raw_opcode & 0x0FFF; // last three nybls

        match opcode_hex_digits {
            [0x0, 0x0, 0xE, 0x0] => Opcode::ClearScreen,
            [0x0, 0x0, 0xE, 0xE] => Opcode::Return, // return from subroutine
            [0x1,   _,   _,   _] => Opcode::Jump{ address: nnn }, // Jumps to address NNN
            [0x2,   _,   _,   _] => Opcode::Call{ address: nnn }, // Calls subroutine at NNN
            [0x3,   _,   _,   _] => Opcode::SkipIfRegisterEqualTo { register: x, value: nn }, // Skips the next instruction if VX is equal to last two nybls
            [0x4,   _,   _,   _] => Opcode::SkipIfRegisterNotEqualTo { register: x, value: nn }, // Skips the next instruction if VX is NOT equal to last two nybls
            [0x5,   _,   _, 0x0] => Opcode::SkipIfRegistersEqual { lhs_register: x, rhs_register: y }, // Skips the next instruction if VX equals VY
            [0x6,   _,   _,   _] => Opcode::AssignValue { register: x, value: nn }, // Sets VX to NN
            [0x7,   _,   _,   _] => Opcode::AddAssignValue { register: x, value: nn }, // Adds NN to VX (carry flag is not changed)
            [0x8,   _,   _, 0x0] => Opcode::Assign { lhs_register: x, rhs_register: y }, // Sets VX to the value of VY
            [0x8,   _,   _, 0x1] => Opcode::OrAssign { lhs_register: x, rhs_register: y }, // Sets VX to VX or VY (bitwise)
            [0x8,   _,   _, 0x2] => Opcode::AndAssign { lhs_register: x, rhs_register: y }, // Sets VX to VX and VY (bitwise)
            [0x8,   _,   _, 0x3] => Opcode::XorAssign { lhs_register: x, rhs_register: y }, // Sets VX to VX xor VY (bitwise)
            [0x8,   _,   _, 0x4] => Opcode::AddAssign { lhs_register: x, rhs_register: y }, // Adds VY to VX. VF is set to 1 when there's an overflow, and to 0 when there is not
            [0x8,   _,   _, 0x5] => Opcode::SubtractAssign { lhs_register: x, rhs_register: y }, // VY is subtracted from VX. VF is set to 0 when there's an underflow, and 1 when there is not
            [0x8,   _,   _, 0x6] => Opcode::RightShiftAssign { lhs_register: x, rhs_register: y }, // Stores the least significant bit of VX in VF and then shifts VX to the right by 1
            [0x8,   _,   _, 0x7] => Opcode::SubtractAssignReversed { lhs_register: x, rhs_register: y }, // Sets VX to VY minus VX. VF is set to 0 when there's an underflow, and 1 when there is not
            [0x8,   _,   _, 0xE] => Opcode::NoOp, // Stores the most significant bit of VX in VF and then shifts VX to the left by 1
            [0x9,   _,   _, 0x0] => Opcode::NoOp, // Skips the next instruction if VX does not equal VY
            [0xA,   _,   _,   _] => Opcode::NoOp, // Sets I to the address NNN
            [0xB,   _,   _,   _] => Opcode::NoOp, // Jumps to the address NNN plus V0
            [0xC,   _,   _,   _] => Opcode::NoOp, // Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN
            [0xD,   _,   _,   _] => Opcode::NoOp, // draw a sprite
            [0xE,   _, 0x9, 0xE] => Opcode::NoOp, // Skips the next instruction if the key stored in VX is pressed 
            [0xE,   _, 0xA, 0x1] => Opcode::NoOp, // Skips the next instruction if the key stored in VX is NOT pressed
            [0xF,   _, 0x0, 0x7] => Opcode::NoOp, // Sets VX to the value of the delay timer
            [0xF,   _, 0x0, 0xA] => Opcode::NoOp, // A key press is awaited, and then stored in VX
            [0xF,   _, 0x1, 0x5] => Opcode::NoOp, // Sets the delay timer to VX
            [0xF,   _, 0x1, 0x8] => Opcode::NoOp, // Sets the sound timer to VX
            [0xF,   _, 0x1, 0xE] => Opcode::NoOp, // Adds VX to I. VF is not affected.
            [0xF,   _, 0x2, 0x9] => Opcode::NoOp, // Sets I to the location of the sprite for the character in VX
            [0xF,   _, 0x3, 0x3] => Opcode::NoOp, // Stores the binary-coded decimal representation of V_, with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2
            [0xF,   _, 0x5, 0x5] => Opcode::NoOp, // Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified
            [0xF,   _, 0x6, 0x5] => Opcode::NoOp, // Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified 
            _ => Opcode::NoOp,
        }
    }
}
impl From<&[u8]> for Opcode {
    fn from(value: &[u8]) -> Self {
        if value.len() == 2 {
            combine_bytes(value[0], value[1]).into()
        } else {
            0x0000.into()
        }
    }
}
fn combine_bytes(top_half: u8, bottom_half: u8) -> u16 {
    (top_half as u16) << 8 | bottom_half as u16
}