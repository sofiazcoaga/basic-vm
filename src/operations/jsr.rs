use crate::{error::VMError, operations::utils::sign_extend, registers::Register, VMState};
pub fn handle_jsr(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let long_flag = ((instruction >> 11) & 1) > 0;
    vm.registers[Register::R7.usize()] = vm.registers[Register::PC.usize()];
    if long_flag {
        let pc_offset = sign_extend(instruction & 0x7FF, 11);
        vm.registers[Register::PC.usize()] += pc_offset;
    } else {
        let base_reg = ((instruction >>6) & 0x7) as usize;
        vm.registers[Register::PC.usize()] = vm.registers[base_reg];
    }
    Ok(())
}
