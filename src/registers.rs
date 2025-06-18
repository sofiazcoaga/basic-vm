use crate::error::VMError;

#[allow(dead_code)]
/// The enumerated representation for the registers in the VM.
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    PC = 8,
    Cond = 9,
}
impl Register {
    pub const COUNT: usize = 10;

    pub fn usize(self) -> usize {
        self as usize
    }
}

/// The representation of the memory registers related to
/// keyboard status.
pub enum MemoryRegister {
    Kbsr = 0xFE00, // Keyboard status.
    Kbdr = 0xFE02, // Keyboard data.
}

impl TryInto<u16> for MemoryRegister {
    type Error = VMError;
    fn try_into(self) -> Result<u16, Self::Error> {
        Ok(self as u16)
    }
}

impl MemoryRegister {
    pub fn usize(self) -> usize {
        self as usize
    }
}
