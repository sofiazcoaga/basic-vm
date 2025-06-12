use crate::{
    VMState,
    error::VMError,
    operations::utils::{sign_extend, update_flags},
    registers::Register,
};
pub fn handle_lea(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    vm.registers[dest_reg] = vm.registers[Register::PC.usize()].wrapping_add(pc_offset);
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{VMState, operations::lea::handle_lea, registers::Register};

    #[test]
    fn loads_address_to_register() {
        let mut vm = VMState::init().unwrap();
        // LEA  DestReg PCOffset
        // 1110 001     000000111
        let lea_ix = 0xE207;
        assert_eq!(vm.registers[Register::R1.usize()], 0); // R1 is empty at first
        let res = handle_lea(lea_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(
            vm.registers[Register::R1.usize()],
            vm.registers[Register::PC.usize()].wrapping_add(0x0007)
        );
    }
}
