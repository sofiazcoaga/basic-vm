use std::io::Write;

use crate::opcodes::Opcode::{self, *};
use crate::operations::add::handle_add;
use crate::operations::and::handle_and;
use crate::operations::br::handle_br;
use crate::operations::jmp::handle_jmp;
use crate::operations::jsr::handle_jsr;
use crate::operations::ld::handle_ld;
use crate::operations::ldi::handle_ldi;
use crate::operations::ldr::handle_ldr;
use crate::operations::lea::handle_lea;
use crate::operations::not::handle_not;
use crate::operations::st::handle_st;
use crate::operations::sti::handle_sti;
use crate::operations::str::handle_str;
use crate::operations::trap::handle_trap;
use crate::registers::Register::*;
use crate::{
    error::VMError,
    flags::Flag,
    registers::{MemoryRegister, Register},
    utils::{disable_input_buffering, get_char, restore_terminal},
};

/// Memory size for LC-3 architecture, where each memory position stores a 16 bit value. [See more here.](https://www.jmeiners.com/lc3-vm/#lc-3-architecture)
pub const MEMORY_MAX: usize = 1 << 16;

/// This structure represents the state of the VM, so basically the registers and the memory. It is instantiated only once
/// when the program runs and its updated through instructions execution.
pub struct VMState {
    /// A fixed size array representing the memory of the VM.
    pub memory: [u16; MEMORY_MAX],
    /// A fixed size array representing the registers of the VM.
    pub registers: [u16; Register::COUNT],
}
impl VMState {
    /// Acts as the constructor of the VMState, initiating it with default values: the memory starts empty (filled with zeros in each position)
    /// and all of the registers that are not the PC or the COND start with zero as well. PC starts by default in value 0x3000 (the address where
    /// most programs store their first instruction) and COND register starts with flag ZERO.
    pub fn init() -> Result<Self, VMError> {
        let mut vm = Self {
            memory: [0; MEMORY_MAX],
            registers: [0; Register::COUNT],
        };
        vm.registers[Register::Cond] = Flag::Zro.try_into()?;
        vm.registers[Register::PC] = 0x3000; // Set PC to starting position. 0x3000 is the default.
        Ok(vm)
    }

    /// Having parsed the binary file into a Vec of u8, this function allows to store every instruction in the VM's memory.
    /// Consider an instruction is made of two bytes and that LC3 binaries come in big endian.
    pub fn write_ixs_to_mem(&mut self, parsed_file: Vec<u8>) {
        // The offset when reading the file.
        let mut file_index = 0;

        // The first two bytes of the binary indicate where the program will store its first instruction, therefore which
        // will be the addres the PC must be set up with at the start. Usually is 0x3000.
        let origin = u16::from_be_bytes([parsed_file[file_index], parsed_file[file_index + 1]]);
        file_index += 2;

        // We must write instructions in memory from the origin address.
        let mut offset = origin;
        while file_index + 1 < parsed_file.len() {
            // We take two bytes at a time.
            let content =
                u16::from_be_bytes([parsed_file[file_index], parsed_file[file_index + 1]]);
            self.mem_write(offset, content);
            file_index += 2;
            offset += 1;
        }
    }

    /// Writes the content passed as `val` inside the memory position given by `address`.
    pub fn mem_write(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }

    /// Reads the content of the memory in a specific position. If the address to be read is the corresponding to the
    /// memory register `Keyboard Status (Kbsr)`, the VM tries to read a character from stdin. In case it reads something
    /// it stores the new value in the other memory register `Keyboard Data (Kbdr)`, otherwise it stores 0.
    pub fn mem_read(&mut self, address: u16) -> Result<u16, VMError> {
        if address == MemoryRegister::Kbsr.try_into()? {
            let char = get_char()?;
            if char != 0 {
                self.memory[MemoryRegister::Kbsr] = 1 << 15;
                self.memory[MemoryRegister::Kbdr] = char;
            } else {
                self.memory[MemoryRegister::Kbsr] = 0;
            }
        }
        Ok(self.memory[address as usize])
    }

    /// Runs the virtual machine and executes instruction loop. 
    pub fn run(&mut self, file_vec: Vec<u8>) -> Result<(), VMError> {
        // We disable input buffering (keys will be detected as soon as they are pressed and they will not be echoed).
        // We store the original terminal configuration to restore it when the program finishes.
        let original_terminal_setup = disable_input_buffering()?;

        // Write the obtained instructions from the file into VM's memory
        self.write_ixs_to_mem(file_vec);

        // Set the running flag to true - only HALT instruction will set it to false and stop the execution loop.
        let mut running = true;

        // Execution loop.
        while running {
            // Get the next instruction from memory - its address is stored in the PC register.
            let ix: u16 = self.mem_read(self.registers[PC])?;
            // Update the Program Counter to store the next ix address.
            self.registers[PC] += 1;
            // Decode instruction opcode.
            let opcode = Opcode::try_from(ix >> 12)?;

            // Handle opcode.
            match opcode {
                OpADD => handle_add(ix, self)?,
                OpAND => handle_and(ix, self)?,
                OpNOT => handle_not(ix, self)?,
                OpBR => handle_br(ix, self)?,
                OpJMP => handle_jmp(ix, self)?,
                OpJSR => handle_jsr(ix, self)?,
                OpLD => handle_ld(ix, self)?,
                OpLDI => handle_ldi(ix, self)?,
                OpLDR => handle_ldr(ix, self)?,
                OpLEA => handle_lea(ix, self)?,
                OpST => handle_st(ix, self)?,
                OpSTI => handle_sti(ix, self)?,
                OpSTR => handle_str(ix, self)?,
                OpTRAP => handle_trap(ix, self, &mut running)?,
                OpRES => println!("Opcode is RES"), // Unused
                OpRTI => println!("Opcode is RTI"), // Unused
            }

            // If operation was I/O force output to be delivered right away.
            std::io::stdout()
                .flush()
                .map_err(|e| VMError::ErrorFlushinStdout(e.to_string()))?;
        }

        // When the program is finished, restore terminal to its original configuration.
        restore_terminal(original_terminal_setup)?;
        Ok(())
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
