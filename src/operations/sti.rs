use crate::{
    VMState,
    error::VMError,
    mem_read, mem_write,
    operations::{add, utils::sign_extend},
    registers::Register,
};
pub fn handle_sti(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let src_reg = ((instruction >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    let address = mem_read(
        vm.registers[Register::PC.usize()].wrapping_add(pc_offset),
        vm,
    );
    mem_write(address, vm.registers[src_reg], vm);
    Ok(())
}
