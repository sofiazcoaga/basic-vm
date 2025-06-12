use std::io::{Read, Write};

use crate::{
    VMState, error::VMError, mem_read, operations::utils::update_flags, registers::Register,
};
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
    Getc = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    Out = 0x21,   /* output a character */
    Puts = 0x22,  /* output a word string */
    In = 0x23,    /* get character from keyboard, echoed onto the terminal */
    PutSp = 0x24, /* output a byte string */
    Halt = 0x25,  /* halt the program */
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

            _ => return Err(VMError::UnrecognizedTrapCode),
        };
        Ok(result)
    }
}

fn handle_getc(vm: &mut VMState) -> Result<(), VMError> {
    let mut char = [0];
    std::io::stdin()
        .read_exact(&mut char)
        .map_err(|e| VMError::CouldNotReadChar(e.to_string()))?;
    vm.registers[Register::R0.usize()] = char[0] as u16;
    Ok(())
}

fn handle_out(vm: &mut VMState) -> Result<(), VMError> {
    print!(
        "{:?}",
        vm.registers[Register::R0.usize()].to_le_bytes()[0] as char
    );
    Ok(())
}

fn handle_putsp(vm: &mut VMState) -> Result<(), VMError> {
    let mut memory_address = vm.registers[Register::R0.usize()];
    let mut content = mem_read(memory_address, vm);
    while content != 0 {
        let bytes: [u8; 2] = content.to_le_bytes();
        print!("{:?}", bytes[0] as char);
        print!("{:?}", bytes[1] as char);
        memory_address = memory_address.wrapping_add(1);
        content = mem_read(memory_address, vm);
    }
    Ok(())
}

fn handle_in(vm: &mut VMState) -> Result<(), VMError> {
    println!("Enter a character: ");
    let mut char = [0];
    std::io::stdin()
        .read_exact(&mut char)
        .map_err(|e| VMError::CouldNotReadChar(e.to_string()))?;
    print!("{:?}", char[0] as char);
    vm.registers[Register::R0.usize()] = char[0] as u16;
    update_flags(vm, vm.registers[Register::R0.usize()])?;
    Ok(())
}

fn handle_puts(vm: &mut VMState) -> Result<(), VMError> {
    let mut memory_address = vm.registers[Register::R0.usize()];
    let mut content = mem_read(memory_address, vm);
    while content != 0 {
        print!("{:?}", content.to_string());
        memory_address = memory_address.wrapping_add(1);
        content = mem_read(memory_address, vm);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Read;

    use crate::{
        VMState, mem_write,
        operations::trap::{handle_out, handle_putsp},
        registers::Register,
    };

    #[test]
    fn probando() {
        let mut char = [0];

        std::io::stdin().read_exact(&mut char);
        let value = char[0] as u16;
        println!("big endian {:?}", value.to_be_bytes());
        println!("little endian: {:?}", value.to_le_bytes());
        // let t: u16 = 116;
        // let mut vm = VMState::init().unwrap();
        // let mut mem_addr = 0x3000;
        // vm.registers[Register::R0.usize()] = mem_addr;
        // mem_write(mem_addr, u16::from_le_bytes([72, 79]), &mut vm);
        // mem_addr +=1;
        // mem_write(mem_addr, u16::from_le_bytes([76, 65]), &mut vm);

        // handle_putsp(&mut vm);
    }
}
