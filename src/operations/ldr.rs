use crate::{
    VMState,
    error::VMError,
    operations::utils::{sign_extend, update_flags},
};

/// Handler for instruction LOAD FROM REGISTER. A memory address gets calculated from the
/// content of the base register plus the offset. The content of this calculated memory address
/// gets stored in the destination register.
//         | LDR opcode (0110) | destination reg | base reg | offset |
//         |   4 bits          |     3 bits      |   3 bits | 6 bits |
pub fn handle_ldr(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let base_reg = ((instruction >> 6) & 0x7) as usize;
    let offset = sign_extend(instruction & 0x3F, 6);
    vm.registers[dest_reg] = vm.mem_read(vm.registers[base_reg].wrapping_add(offset))?;
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{VMState, operations::ldr::handle_ldr, registers::Register};

    #[test]
    fn loads_register() {
        let mut vm = VMState::init().unwrap();
        let content_base_reg: u16 = 0x3020; // Base Reg contains random memory address.
        let offset: u16 = 0x0004; //offset that will be used in ix.
        let random_memory_content: u16 = 400;
        vm.registers[Register::R2] = content_base_reg;
        vm.mem_write(content_base_reg.wrapping_add(offset), random_memory_content);
        // LDR  DestReg BaseReg Offset
        // 0110 001     010     000100
        let ldr_ix = 0x6284;
        assert_eq!(vm.registers[Register::R1], 0); // R1 is currently empty
        assert_eq!(vm.registers[Register::R2], content_base_reg); // R2 contains a memory address

        let res = handle_ldr(ldr_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(vm.registers[Register::R1], random_memory_content);
    }
}
