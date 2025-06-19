use crate::{
    VMState, error::VMError, operations::utils::update_flags, registers::Register, utils::get_char,
};

/// Handler for instruction TRAP, that is related with I/O interactions. There are
/// different types of traps that are executed differently.
//         | TRAP opcode (1111)| unused | Trap Type |
//         |   4 bits          | 4 bits | 8 bits    |
pub fn handle_trap(instruction: u16, vm: &mut VMState, running: &mut bool) -> Result<(), VMError> {
    match TrapCode::try_from(instruction & 0xFF)? {
        TrapCode::Getc => handle_getc(vm)?,
        TrapCode::Out => handle_out(vm)?,
        TrapCode::PutSp => handle_putsp(vm)?,
        TrapCode::In => handle_in(vm)?,
        TrapCode::Puts => handle_puts(vm)?,
        TrapCode::Halt => {
            println!("Halt execution");
            *running = false;
        }
    }
    Ok(())
}
enum TrapCode {
    Getc = 0x20,  // Get character from keyboard, not echoed onto the terminal.
    Out = 0x21,   // Output a character.
    Puts = 0x22,  // Output a word string.
    In = 0x23,    // Get character from keyboard, echoed onto the terminal.
    PutSp = 0x24, // Output a byte string.
    Halt = 0x25,  // Halt the program.
}

impl TryInto<u16> for TrapCode {
    type Error = VMError;
    fn try_into(self) -> Result<u16, Self::Error> {
        Ok(self as u16)
    }
}

impl TryFrom<u16> for TrapCode {
    type Error = VMError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let result = match value {
            0x20 => TrapCode::Getc,
            0x21 => TrapCode::Out,
            0x22 => TrapCode::Puts,
            0x23 => TrapCode::In,
            0x24 => TrapCode::PutSp,
            0x25 => TrapCode::Halt,

            _ => return Err(VMError::UnrecognizedTrapCode(value)),
        };
        Ok(result)
    }
}

/// Gets a character from standard input and stores it in R0.
fn handle_getc(vm: &mut VMState) -> Result<(), VMError> {
    let char = get_char()?;
    vm.registers[Register::R0] = char;
    update_flags(vm, char)?;
    Ok(())
}

/// Prints the character stored in the first byte of R0.
fn handle_out(vm: &mut VMState) -> Result<(), VMError> {
    let char = vm.registers[Register::R0].to_le_bytes()[0];
    print_char(char);
    Ok(())
}

/// Prints two characters per memory address, one per each byte.
fn handle_putsp(vm: &mut VMState) -> Result<(), VMError> {
    let mut memory_address = vm.registers[Register::R0];
    let mut content = vm.mem_read(memory_address)?;
    while content != 0 {
        let bytes: [u8; 2] = content.to_le_bytes();
        print_char(bytes[0]);
        if bytes[1] != b'\0' {
            print_char(bytes[1]);
        }
        memory_address = memory_address.wrapping_add(1);
        content = vm.mem_read(memory_address)?;
    }
    Ok(())
}

/// Gets a character from standard input echoing it to terminal.
fn handle_in(vm: &mut VMState) -> Result<(), VMError> {
    print!("\n\rEnter a character: \n\r");
    let char = get_char()?;
    vm.registers[Register::R0] = char;
    print!("{}", char as u8 as char);
    update_flags(vm, char)?;
    Ok(())
}

/// Prints one char per memory address.
fn handle_puts(vm: &mut VMState) -> Result<(), VMError> {
    let mut memory_address = vm.registers[Register::R0];
    let mut content = vm.mem_read(memory_address)?;
    while content != 0 {
        print_char(content.to_le_bytes()[0]);
        memory_address = memory_address.wrapping_add(1);
        content = vm.mem_read(memory_address)?;
    }
    Ok(())
}

fn print_char(char: u8) {
    if char == 0x0A {
        print!("\n\r");
    } else {
        print!("{}", char as char);
    }
}
