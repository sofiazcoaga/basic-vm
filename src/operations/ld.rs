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
    vm.registers[dest_reg] = mem_read(
        vm.registers[Register::PC.usize()].wrapping_add(pc_offset),
        vm,
    )?;
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{VMState, mem_write, operations::ld::handle_ld, registers::Register};

    #[test]
    fn loads_a_register() {
        let offset_u16: u16 = 0x0004;
        let pc_value: u16 = 0x3000;
        let memory_address = pc_value.wrapping_add(offset_u16);
        let mut vm = VMState::init().unwrap();
        mem_write(memory_address, 50, &mut vm);
        // LD   DestReg PCOffset
        // 0010 001     000000100
        let ld_ix: u16 = 0x2204;
        assert_eq!(vm.registers[Register::PC.usize()], 0x3000); // Default init value for PC
        let res = handle_ld(ld_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(vm.registers[Register::R1.usize()], 50);
    }
}
