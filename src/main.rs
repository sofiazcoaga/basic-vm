use std::fs::{self};

use crate::error::VMError;

mod error;

use crate::flags::Flag;
use crate::opcodes::Opcode::{self, *};
use crate::operations::add::handle_add;
use crate::operations::and::handle_and;
use crate::operations::jmp::handle_jmp;
use crate::operations::jsr::handle_jsr;
use crate::operations::ld::handle_ld;
use crate::operations::ldi::handle_ldi;
use crate::operations::ldr::handle_ldr;
use crate::operations::not::handle_not;
use crate::registers::Register::{self, *};

mod flags;
mod opcodes;
mod operations;
mod registers;
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
        vm.registers[Cond.usize()] = Flag::Zro.try_into()?;
        vm.registers[PC.usize()] = 0x3000; // Set PC to starting position. 0x3000 is the default.
        Ok(vm)
    }
}

fn main() -> Result<(), VMError> {
    // Fill memory with instructions here
    let example_file = read_file("./binary-examples/2048.obj")?;

    // Initialize VM state
    let mut vm = VMState::init()?;

    write_ixs_to_mem(example_file, &mut vm);

    let mut running = true;

    while running {
        // Get the next instruction
        let ix: u16 = mem_read(vm.registers[PC.usize()], &vm);
        vm.registers[PC.usize()] += 1;
        let opcode = Opcode::try_from(ix >> 12)?;

        match opcode {
            OpADD => {
                println!("Opcode is ADD");
                handle_add(ix, &mut vm)?;
            }
            OpAND => {
                println!("Opcode is AND");
                handle_and(ix, &mut vm)?;
            }
            OpNOT => {
                println!("Opcode is NOT");
                handle_not(ix, &mut vm)?;
            }
            OpBR => println!("Opcode is BR"),
            OpJMP => {
                println!("Opcode is JMP");
                handle_jmp(ix, &mut vm)?;
            }
            OpJSR => {
                println!("Opcode is JSR");
                handle_jsr(ix, &mut vm)?;
            }
            OpLD => {
                println!("Opcode is LD");
                handle_ld(ix, &mut vm)?;
            }
            OpLDI => {
                println!("Opcode is LDI");
                handle_ldi(ix, &mut vm)?;
            }
            OpLDR => {
                println!("Opcode is LDR");
                handle_ldr(ix, &mut vm)?;
            }
            OpLEA => println!("Opcode is LEA"),
            OpST => println!("Opcode is ST"),
            OpSTI => println!("Opcode is STI"),
            OpSTR => println!("Opcode is STR"),
            OpTRAP => println!("Opcode is TRAP"),
            OpRES => println!("Opcode is RES"),
            OpRTI => println!("Opcode is RTI"),
        }

        running = false; // Temporarily until opcodes are filled.
    }

    Ok(())
}

fn mem_read(address: u16, vm: &VMState) -> u16 {
    vm.memory[address as usize]
}

fn mem_write(address: u16, val: u16, vm: &mut VMState) {
    vm.memory[address as usize] = val;
}

fn read_file(path: &str) -> Result<Vec<u8>, VMError> {
    let read_result = fs::read(path).map_err(|e| VMError::CouldNotReadFile(e.to_string()))?;
    Ok(read_result)
}

fn write_ixs_to_mem(parsed_file: Vec<u8>, vm: &mut VMState) {
    let mut file_index = 0;
    let origin = u16::from_be_bytes([parsed_file[file_index], parsed_file[file_index + 1]]);
    file_index += 2;

    let mut offset = origin;
    while file_index + 1 < parsed_file.len() {
        // LC3 binaries come in big endian but we need to store it swapped
        let content = u16::from_le_bytes([parsed_file[file_index], parsed_file[file_index + 1]]);
        mem_write(offset, content, vm);
        file_index += 2;
        offset += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_file() {
        let path = "./binary-examples/2048.obj";
        let read_file = read_file(path).unwrap();
        assert_eq!(read_file.len(), 2276);
    }

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
        write_ixs_to_mem(binary, &mut vm);
        assert_eq!(
            vm.memory[origin as usize],
            u16::from_le_bytes(first_ix.to_be_bytes())
        );
        assert_eq!(
            vm.memory[(origin + 1) as usize],
            u16::from_le_bytes(second_ix.to_be_bytes())
        );
    }
}
