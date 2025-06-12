use crate::{
    error::VMError, operations::utils::{sign_extend, update_flags}, registers::Register, VMState
};
pub fn handle_lea(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    vm.registers[dest_reg] = vm.registers[Register::PC.usize()].wrapping_add(pc_offset);
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}
