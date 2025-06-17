use crate::{
    error::VMError,
    flags::Flag,
    registers::{MemoryRegister, Register},
    utils::get_char,
};

pub const MEMORY_MAX: usize = 1 << 16;
pub struct VMState {
    pub memory: [u16; MEMORY_MAX],
    pub registers: [u16; Register::COUNT],
}
impl VMState {
    pub fn init() -> Result<Self, VMError> {
        let mut vm = Self {
            memory: [0; MEMORY_MAX],
            registers: [0; Register::COUNT],
        };
        vm.registers[Register::Cond.usize()] = Flag::Zro.try_into()?;
        vm.registers[Register::PC.usize()] = 0x3000; // Set PC to starting position. 0x3000 is the default.
        Ok(vm)
    }

    pub fn write_ixs_to_mem(&mut self, parsed_file: Vec<u8>) {
        let mut file_index = 0;
        let origin = u16::from_be_bytes([parsed_file[file_index], parsed_file[file_index + 1]]);
        file_index += 2;

        let mut offset = origin;
        while file_index + 1 < parsed_file.len() {
            // LC3 binaries come in big endian but we need to store it swapped
            let content =
                u16::from_be_bytes([parsed_file[file_index], parsed_file[file_index + 1]]);
            self.mem_write(offset, content);
            file_index += 2;
            offset += 1;
        }
    }

    pub fn mem_write(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }

    pub fn mem_read(&mut self, address: u16) -> Result<u16, VMError> {
        if address == MemoryRegister::Kbsr.try_into()? {
            let char = get_char()?;
            if char != 0 {
                self.memory[MemoryRegister::Kbsr.usize()] = 1 << 15;
                self.memory[MemoryRegister::Kbdr.usize()] = char;
            } else {
                self.memory[MemoryRegister::Kbsr.usize()] = 0;
            }
        }
        Ok(self.memory[address as usize])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn writes_ix_to_memory() {
        let mut vm = VMState::init().unwrap();
        let origin: u16 = 0x3000;
        let first_ix: u16 = 0x4314;
        let second_ix: u16 = 0x975A;
        let binary = vec![
            origin.to_be_bytes(),
            first_ix.to_be_bytes(),
            second_ix.to_be_bytes(),
        ]
        .concat();
        vm.write_ixs_to_mem(binary);
        assert_eq!(vm.memory[origin as usize], first_ix);
        assert_eq!(vm.memory[(origin + 1) as usize], second_ix);
    }
}
