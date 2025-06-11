use crate::error::VMError;
use crate::flags::Flag;
use crate::opcodes::Opcode::{self, *};
use crate::registers::Register::{self, *};
mod error;
mod flags;
mod opcodes;
mod registers;

pub const MEMORY_MAX: usize = 1 << 16;
fn main() -> Result<(), VMError> {
    // Initialize memory
    let memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];

    // Fill memory with instructions here

    // Initialize registers
    let mut registers: [u16; Register::COUNT] = [0; Register::COUNT];
    registers[Cond.usize()] = Flag::Zro.try_into()?;
    registers[PC.usize()] = 0x3000; // Set PC to starting position. 0x3000 is the default.

    let mut running = true;

    while running {
        // Get the next instruction
        let ix: u16 = mem_read(registers[PC.usize()], &memory);
        registers[PC.usize()] += 1;
        let opcode = Opcode::try_from(ix >> 12)?;

        match opcode {
            OpADD => println!("Opcode is ADD"),
            OpAND => println!("Opcode is AND"),
            OpNOT => println!("Opcode is NOT"),
            OpBR => println!("Opcode is BR"),
            OpJMP => println!("Opcode is JMP"),
            OpJSR => println!("Opcode is JSR"),
            OpLD => println!("Opcode is LD"),
            OpLDI => println!("Opcode is LDI"),
            OpLDR => println!("Opcode is LDR"),
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

fn mem_read(address: u16, memory: &[u16; MEMORY_MAX]) -> u16 {
    memory[address as usize]
}
