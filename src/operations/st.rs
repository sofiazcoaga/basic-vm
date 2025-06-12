use crate::{
    VMState, error::VMError, mem_write, operations::utils::sign_extend, registers::Register,
};
pub fn handle_st(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let src_reg = ((instruction >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    mem_write(
        vm.registers[Register::PC.usize()].wrapping_add(pc_offset),
        vm.registers[src_reg],
        vm,
    );
    Ok(())
}
