use crate::error::VMError;

/// The enumerated representation for the instructions opcodes.
pub enum Opcode {
    OpBR = 0,    // Branch
    OpADD = 1,   // Add
    OpLD = 2,    // Load
    OpST = 3,    // Store
    OpJSR = 4,   // Jump register
    OpAND = 5,   // Bitwise and
    OpLDR = 6,   // Load register
    OpSTR = 7,   // Store register
    OpRTI = 8,   // Unused
    OpNOT = 9,   // Bitwise not
    OpLDI = 10,  // Load indirect
    OpSTI = 11,  // Store indirect
    OpJMP = 12,  // Jump
    OpRES = 13,  // Reserved (unused)
    OpLEA = 14,  // Load effective address
    OpTRAP = 15, // I/O
}

impl TryInto<u16> for Opcode {
    type Error = VMError;
    fn try_into(self) -> Result<u16, Self::Error> {
        Ok(self as u16)
    }
}

impl TryFrom<u16> for Opcode {
    type Error = VMError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let result = match value {
            0 => Opcode::OpBR,
            1 => Opcode::OpADD,
            2 => Opcode::OpLD,
            3 => Opcode::OpST,
            4 => Opcode::OpJSR,
            5 => Opcode::OpAND,
            6 => Opcode::OpLDR,
            7 => Opcode::OpSTR,
            8 => Opcode::OpRTI,
            9 => Opcode::OpNOT,
            10 => Opcode::OpLDI,
            11 => Opcode::OpSTI,
            12 => Opcode::OpJMP,
            13 => Opcode::OpRES,
            14 => Opcode::OpLEA,
            15 => Opcode::OpTRAP,
            _ => return Err(VMError::UnrecognizedOpcode),
        };
        Ok(result)
    }
}
