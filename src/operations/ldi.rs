use crate::{
    VMState,
    error::VMError,
    operations::utils::{sign_extend, update_flags},
    registers::Register,
};

/// Handler for instruction LOAD INDIRECT. This instruction calculates a memory address
/// by adding the PC offset to the current PC content. The content of this calculated address
/// will be the memory address of the actual content to be loaded to the destination register.
//         | LDI opcode (1010) | destination reg | PC offset |
//         |   4 bits          |     3 bits      |   9 bits  |
pub fn handle_ldi(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    let first_addr = vm.registers[Register::PC.usize()].wrapping_add(pc_offset);
    let final_addr = vm.mem_read(first_addr)?;
    vm.registers[dest_reg] = vm.mem_read(final_addr)?;
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::VMState;

    #[test]
    fn loads_register() {
        let mut vm = VMState::init().unwrap();
        let pc_content = 0x3000;
        let pc_offset: u16 = 0x00FF;
        let first_addr = pc_content + pc_offset;
        let content_addr = 0x4000;
        let random_content = 0x1234;
        vm.memory[first_addr as usize] = content_addr;
        vm.memory[content_addr as usize] = random_content;

        assert_eq!(vm.registers[Register::PC.usize()], pc_content);
        assert_eq!(vm.registers[Register::R1.usize()], 0); // Still has nothing
        // LDI  DestReg PCOffset
        // 1010 001     011111111
        let ldi_ix = 0xA2FF;
        let res = handle_ldi(ldi_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(vm.registers[Register::R1.usize()], random_content);
    }
}
