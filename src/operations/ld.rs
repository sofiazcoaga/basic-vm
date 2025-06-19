use crate::{
    VMState,
    error::VMError,
    operations::utils::{sign_extend, update_flags},
    registers::Register,
};

/// Handler for instruction LOAD, that loads the content of a calculated memory address
/// into the destination register. The calculated address is obtained by adding the PC offset
/// to the current PC content.
//         | LD opcode (0010) | destination reg | PC offset |
//         |   4 bits         |     3 bits      |   9 bits  |
pub fn handle_ld(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    vm.registers[dest_reg] = vm.mem_read(vm.registers[Register::PC].wrapping_add(pc_offset))?;
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{VMState, operations::ld::handle_ld, registers::Register};

    #[test]
    fn loads_a_register() {
        let offset_u16: u16 = 0x0004;
        let pc_value: u16 = 0x3000;
        let memory_address = pc_value.wrapping_add(offset_u16);
        let mut vm = VMState::init().unwrap();
        vm.mem_write(memory_address, 50);
        // LD   DestReg PCOffset
        // 0010 001     000000100
        let ld_ix: u16 = 0x2204;
        assert_eq!(vm.registers[Register::PC], 0x3000); // Default init value for PC
        let res = handle_ld(ld_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(vm.registers[Register::R1], 50);
    }
}
