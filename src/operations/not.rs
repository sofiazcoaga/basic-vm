use crate::{VMState, error::VMError};

pub fn handle_not(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let src_reg = ((instruction >> 6) & 0x7) as usize;
    vm.registers[dest_reg] = !vm.registers[src_reg];
    Ok(())
}
