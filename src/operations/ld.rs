use crate::{
    VMState,
    error::VMError,
    mem_read,
    operations::utils::{sign_extend, update_flags},
    registers::Register,
};
pub fn handle_ld(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    vm.registers[dest_reg] = mem_read(vm.registers[Register::PC.usize()] + pc_offset, vm);
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}
