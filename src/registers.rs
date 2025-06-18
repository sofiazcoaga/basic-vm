use std::ops::{Index, IndexMut};

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
}

impl<T> IndexMut<Register> for [T] {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> Index<Register> for [T] {
    type Output = T;
    fn index(&self, index: Register) -> &Self::Output {
        &self[index as usize]
    }
}
/// The representation of the memory registers related to
/// keyboard status.
pub enum MemoryRegister {
    /// Keyboard status.
    Kbsr = 0xFE00,
    /// Keyboard data.
    Kbdr = 0xFE02,
}

impl TryInto<u16> for MemoryRegister {
    type Error = VMError;
    fn try_into(self) -> Result<u16, Self::Error> {
        Ok(self as u16)
    }
}

impl<T> IndexMut<MemoryRegister> for [T] {
    fn index_mut(&mut self, index: MemoryRegister) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> Index<MemoryRegister> for [T] {
    type Output = T;
    fn index(&self, index: MemoryRegister) -> &Self::Output {
        &self[index as usize]
    }
}
