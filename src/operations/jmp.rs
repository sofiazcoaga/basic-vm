use crate::{VMState, error::VMError, registers::Register};
pub fn handle_jmp(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let base_reg = ((instruction >> 6) & 0x7) as usize;
    vm.registers[Register::PC.usize()] = vm.registers[base_reg];
    Ok(())
}
