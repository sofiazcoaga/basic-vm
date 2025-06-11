use crate::{VMState, error::VMError, flags::Flag, registers::Register};

pub fn handle_add(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let src_reg_1 = ((instruction >> 6) & 0x7) as usize;
    let imm_mode = ((instruction >> 5) & 0x1) > 0;
    if imm_mode {
        // Immediate mode
        let imm_operand = sign_extend(instruction & 0x1F, 5);
        vm.registers[dest_reg] = vm.registers[src_reg_1].wrapping_add(imm_operand);
    } else {
        // Register mode
        let src_reg_2 = (instruction & 0x7) as usize;
        vm.registers[dest_reg] = vm.registers[src_reg_1].wrapping_add(vm.registers[src_reg_2]);
    }
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}

fn sign_extend(mut number: u16, bit_count: usize) -> u16 {
    if ((number >> (bit_count - 1)) & 1) > 0 {
        number |= 0xFFFF << bit_count;
    }
    number
}

fn update_flags(vm: &mut VMState, register_value: u16) -> Result<(), VMError> {
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
