use std::env;
use std::io::Write;

mod error;
mod flags;
mod opcodes;
mod operations;
mod registers;
mod utils;
mod vm;

use crate::error::VMError;
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
use crate::utils::{disable_input_buffering, read_file, restore_terminal};
use crate::vm::VMState;

fn main() -> Result<(), VMError> {
    // Get terminal arguments to obtain the path to the binary file to be executed.
    let console_args: Vec<_> = env::args().collect();
    // Arguments length must be two - the first argument is for cargo and the second should be the path.
    if console_args.len() != 2 {
        return Err(VMError::WrongArgumentsLen);
    }
    let path = console_args[1].clone();

    // Read the file.
    let file = read_file(&path)?;

    // After obtaining the needed path, we disable input buffering (keys will be detected as soon as they are pressed and they will not be echoed).
    // We store the original terminal configuration to restore it when the program finishes.
    let original_terminal_setup = disable_input_buffering()?;

    // Initialize VM state with default values
    let mut vm = VMState::init()?;

    // Write the obtained instructions from the file into VM's memory
    vm.write_ixs_to_mem(file);

    // Set the running flag to true - only HALT instruction will set it to false and stop the execution loop.
    let mut running = true;

    // Execution loop.
    while running {
        // Get the next instruction from memory - its address is stored in the PC register.
        let ix: u16 = vm.mem_read(vm.registers[PC.usize()])?;
        // Update the Program Counter to store the next ix address.
        vm.registers[PC.usize()] += 1;
        // Decode instruction opcode.
        let opcode = Opcode::try_from(ix >> 12)?;

        // Handle opcode.
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
