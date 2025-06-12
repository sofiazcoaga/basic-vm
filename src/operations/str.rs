use crate::{VMState, error::VMError, mem_write, operations::utils::sign_extend};
pub fn handle_str(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let src_reg = ((instruction >> 9) & 0x7) as usize;
    let base_reg = ((instruction >> 6) & 0x7) as usize;
    let offset = sign_extend(instruction & 0x3F, 6);
    mem_write(
        vm.registers[base_reg].wrapping_add(offset),
        vm.registers[src_reg],
        vm,
    );
    Ok(())
}
