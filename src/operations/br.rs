use crate::{VMState, error::VMError, operations::utils::sign_extend, registers::Register};
pub fn handle_br(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    let cond_flag = (instruction >> 9) & 0x7;
    if (cond_flag & vm.registers[Register::Cond.usize()]) > 0 {
        vm.registers[Register::PC.usize()] =
            vm.registers[Register::PC.usize()].wrapping_add(pc_offset);
    }
    Ok(())
}
