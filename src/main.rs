use std::fs::{self};

use crate::{consts::*, error::VMError};

mod consts;
mod error;
fn main() -> Result<(), VMError> {
    // Initialize memory
    let mut memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];

    // Fill memory with instructions here
    let example_file = read_file("./binary-examples/2048.obj")?;
    write_ixs_to_mem(example_file, &mut memory);

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

    Ok(())
}

fn mem_read(address: u16, memory: &[u16; MEMORY_MAX]) -> u16 {
    memory[address as usize]
}

fn mem_write(address: u16, val: u16, memory: &mut [u16; MEMORY_MAX]) {
    memory[address as usize] = val;
}

fn read_file(path: &str) -> Result<Vec<u8>, VMError> {
    let read_result = fs::read(path).map_err(|e| VMError::CouldNotReadFile(e.to_string()))?;
    Ok(read_result)
}

fn write_ixs_to_mem(parsed_file: Vec<u8>, memory: &mut [u16; MEMORY_MAX]) {
    let mut file_index = 0;
    let origin = u16::from_be_bytes([parsed_file[file_index], parsed_file[file_index + 1]]);
    file_index += 2;

    let mut offset = origin;
    while file_index + 1 < parsed_file.len() {
        // LC3 binaries come in big endian but we need to store it swapped
        let content = u16::from_le_bytes([parsed_file[file_index], parsed_file[file_index + 1]]);
        mem_write(offset, content, memory);
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
        let mut memory = [0; MEMORY_MAX];
        let origin: u16 = 0x3000;
        let first_ix: u16 = 0x4314;
        let second_ix: u16 = 0x975A;
        let binary = vec![
            origin.to_be_bytes(),
            first_ix.to_be_bytes(),
            second_ix.to_be_bytes(),
        ]
        .concat();
        write_ixs_to_mem(binary, &mut memory);
        assert_eq!(
            memory[origin as usize],
            u16::from_le_bytes(first_ix.to_be_bytes())
        );
        assert_eq!(
            memory[(origin + 1) as usize],
            u16::from_le_bytes(second_ix.to_be_bytes())
        );
    }
}
