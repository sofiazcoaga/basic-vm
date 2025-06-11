use crate::error::VMError;

pub enum Opcode {
    OpBR = 0,   /* branch */
    OpADD = 1,  /* add  */
    OpLD = 2,   /* load */
    OpST = 3,   /* store */
    OpJSR = 4,  /* jump register */
    OpAND = 5,  /* bitwise and */
    OpLDR = 6,  /* load register */
    OpSTR = 7,  /* store register */
    OpRTI = 8,  /* unused */
    OpNOT = 9,  /* bitwise not */
    OpLDI = 10, /* load indirect */
    OpSTI = 11, /* store indirect */
    OpJMP = 12, /* jump */
    OpRES = 13, /* reserved (unused) */
    OpLEA = 14, /* load effective address */
    OpTRAP = 15,
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
