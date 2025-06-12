use crate::{
    VMState,
    error::VMError,
    mem_read,
    operations::utils::{sign_extend, update_flags},
};
pub fn handle_ldr(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let base_reg = ((instruction >> 6) & 0x7) as usize;
    let offset = sign_extend(instruction & 0x3F, 6);
    vm.registers[dest_reg] = mem_read(vm.registers[base_reg].wrapping_add(offset), vm);
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}
