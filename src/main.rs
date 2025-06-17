use std::env;
use std::io::Write;

mod error;
mod utils;
mod flags;
mod opcodes;
mod operations;
mod registers;
mod vm;

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
use crate::utils::{disable_input_buffering, read_file};
use crate::vm::VMState;
use crate::error::VMError;

fn main() -> Result<(), VMError> {
    let console_args: Vec<_> = env::args().collect();
    if console_args.len() != 2 {
        return Err(VMError::WrongArgumentsLen);
    }
    let path = console_args[1].clone();
    let file = read_file(&path)?;

    disable_input_buffering()?;
    // Initialize VM state
    let mut vm = VMState::init()?;

    vm.write_ixs_to_mem(file);

    let mut running = true;

    while running {
        // Get the next instruction
        let ix: u16 = vm.mem_read(vm.registers[PC.usize()])?;
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
        std::io::stdout()
            .flush()
            .map_err(|e| VMError::ErrorFlushinStdout(e.to_string()))?;
    }

    Ok(())
}
