use crate::consts::*;

mod consts;
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
