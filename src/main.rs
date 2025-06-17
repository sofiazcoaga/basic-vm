use std::fs::{self};
use std::io::Read;

use termion::raw::IntoRawMode;

use crate::error::VMError;

mod error;

use crate::flags::Flag;
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
use crate::registers::MemoryRegister;
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
    let example_file = read_file("./binary-examples/rogue.obj")?;
    let mut _stdout = std::io::stdout().into_raw_mode().unwrap();
    // Initialize VM state
    let mut vm = VMState::init()?;

    write_ixs_to_mem(example_file, &mut vm);

    let mut running = true;

    while running {
        // Get the next instruction
        let ix: u16 = mem_read(vm.registers[PC.usize()], &mut vm)?;
        vm.registers[PC.usize()] += 1;
        let opcode = Opcode::try_from(ix >> 12)?;

        match opcode {
            OpADD => handle_add(ix, &mut vm)?,
            OpAND => handle_and(ix, &mut vm)?,
            OpNOT => handle_not(ix, &mut vm)?,
            OpBR => handle_br(ix, &mut vm)?,
            OpJMP => handle_jmp(ix, &mut vm)?,
            OpJSR => handle_jsr(ix, &mut vm)?,
            OpLD => handle_ld(ix, &mut vm)?,
            OpLDI => handle_ldi(ix, &mut vm)?,
            OpLDR => handle_ldr(ix, &mut vm)?,
            OpLEA => handle_lea(ix, &mut vm)?,
            OpST => handle_st(ix, &mut vm)?,
            OpSTI => handle_sti(ix, &mut vm)?,
            OpSTR => handle_str(ix, &mut vm)?,
            OpTRAP => handle_trap(ix, &mut vm, &mut running)?,
            OpRES => println!("Opcode is RES"),
            OpRTI => println!("Opcode is RTI"),
        }
    }

    Ok(())
}

fn mem_read(address: u16, vm: &mut VMState) -> Result<u16, VMError> {
    if address == MemoryRegister::KBSR.try_into()? {
        let char = get_char()?;
        if char != 0 {
            vm.memory[MemoryRegister::KBSR.usize()] = 1 << 15;
            vm.memory[MemoryRegister::KBDR.usize()] = char;
        } else {
            vm.memory[MemoryRegister::KBSR.usize()] = 0;
        }
    }
    Ok(vm.memory[address as usize])
}

fn get_char() -> Result<u16, VMError> {
    let mut buffer: [u8;1] = [0];
    std::io::stdin()
        .read_exact(&mut buffer)
        .map_err(|e| VMError::CouldNotReadChar(e.to_string()))?;
    let char = buffer[0] as u16;
    Ok(char)
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
        // LC3 binaries come in big endian
        let content = u16::from_be_bytes([parsed_file[file_index], parsed_file[file_index + 1]]);
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
        assert_eq!(vm.memory[origin as usize], first_ix);
        assert_eq!(vm.memory[(origin + 1) as usize], second_ix);
    }
}
