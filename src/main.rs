use std::fs;

use crate::{consts::*, error::VMError};

mod consts;
mod error;
fn main() {
    // Initialize memory
    let memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];

    // Fill memory with instructions here

    // Initialize registers
    let mut registers: [u16; REGISTER_COUNT] = [0; REGISTER_COUNT];
    registers[COND] = FL_ZRO;
    registers[PC] = 0x3000; // Set PC to starting position. 0x3000 is the default.

    let mut running = true;

    while running {
        // Get the next instruction
        let ix: u16 = mem_read(registers[PC], &memory);
        registers[PC] += 1;
        let opcode = ix >> 12;

        match opcode {
            OP_ADD => println!("Opcode is ADD"),
            OP_AND => println!("Opcode is AND"),
            OP_NOT => println!("Opcode is NOT"),
            OP_BR => println!("Opcode is BR"),
            OP_JMP => println!("Opcode is JMP"),
            OP_JSR => println!("Opcode is JSR"),
            OP_LD => println!("Opcode is LD"),
            OP_LDI => println!("Opcode is LDI"),
            OP_LDR => println!("Opcode is LDR"),
            OP_LEA => println!("Opcode is LEA"),
            OP_ST => println!("Opcode is ST"),
            OP_STI => println!("Opcode is STI"),
            OP_STR => println!("Opcode is STR"),
            OP_TRAP => println!("Opcode is TRAP"),
            OP_RES => println!("Opcode is RES"),
            OP_RTI => println!("Opcode is RTI"),
            _ => println!("Bad opcode"),
        }
        running = false; // Temporarily until opcodes are filled.
    }
}

fn mem_read(address: u16, memory: &[u16; MEMORY_MAX]) -> u16 {
    memory[address as usize]
}

fn mem_write(address: u16, val: u16, memory: &mut [u16; MEMORY_MAX]) {
    memory[address as usize] = val;
}

fn parse_file_to_memory(path: &str, memory: &mut [u16; MEMORY_MAX]) -> Result<(), VMError>{
    // The origin tells us where in memory to place the content.
    let whole_file = fs::read(path).unwrap();
    //let whole_file = fs::read(path).map_err(|x| Err(VMError::CouldNotReadFile(x.to_string()))); // TODO

    let mut file_index = 0;
    let origin = u16::from_be_bytes([whole_file[file_index], whole_file[file_index + 1]]);
    file_index += 2;

    let mut offset = origin;
    while file_index + 1 < whole_file.len() {
        let content = u16::from_be_bytes([whole_file[file_index], whole_file[file_index + 1]]);
        mem_write(offset, content, memory);
        file_index += 2;
        offset += 1;
    }

    Ok(())
}
