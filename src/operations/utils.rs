use crate::{VMState, error::VMError, flags::Flag, registers::Register};

pub fn sign_extend(mut number: u16, bit_count: usize) -> u16 {
    if ((number >> (bit_count - 1)) & 1) > 0 {
        number |= 0xFFFF << bit_count;
    }
    number
}

pub fn update_flags(vm: &mut VMState, register_value: u16) -> Result<(), VMError> {
    let cond_register = &mut vm.registers[Register::Cond.usize()];
    if register_value == 0 {
        *cond_register = Flag::Zro.try_into()?;
    } else if (register_value >> 15) > 0 {
        *cond_register = Flag::Neg.try_into()?;
    } else {
        *cond_register = Flag::Pos.try_into()?;
    }
    Ok(())
}
