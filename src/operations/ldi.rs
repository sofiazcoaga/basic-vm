use crate::{
    VMState,
    error::VMError,
    mem_read,
    operations::utils::{sign_extend, update_flags},
    registers::Register,
};
pub fn handle_ldi(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    vm.registers[dest_reg] = mem_read(
        vm.registers[Register::PC.usize()].wrapping_add(pc_offset),
        vm,
    )?;
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{VMState, mem_write};

    #[test]
    fn loads_register() {
        let mut vm = VMState::init().unwrap();
        let pc_offset = 0x0007;
        let pc_current_content = vm.registers[Register::PC.usize()];
        mem_write(pc_current_content.wrapping_add(pc_offset), 1000, &mut vm);
        // LDI  DestReg PCOffset
        // 1010 001     000000111
        let ldi_ix = 0xA207;
        let res = handle_ldi(ldi_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(vm.registers[Register::R1.usize()], 1000);
    }
}
