use crate::{
    VMState,
    error::VMError,
    mem_read,
    operations::utils::{sign_extend, update_flags},
};
pub fn handle_ldr(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let base_reg = ((instruction >> 6) & 0x7) as usize;
    let offset = sign_extend(instruction & 0x3F, 6);
    vm.registers[dest_reg] = mem_read(vm.registers[base_reg].wrapping_add(offset), vm);
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{VMState, mem_write, operations::ldr::handle_ldr, registers::Register};

    #[test]
    fn loads_register() {
        let mut vm = VMState::init().unwrap();
        let content_base_reg: u16 = 0x3020; // Base Reg contains random memory address.
        let offset: u16 = 0x0004; //offset that will be used in ix.
        let random_memory_content: u16 = 400;
        vm.registers[Register::R2.usize()] = content_base_reg;
        mem_write(
            content_base_reg.wrapping_add(offset),
            random_memory_content,
            &mut vm,
        );
        // LDR  DestReg BaseReg Offset
        // 0110 001     010     000100
        let ldr_ix = 0x6284;
        assert_eq!(vm.registers[Register::R1.usize()], 0); // R1 is currently empty
        assert_eq!(vm.registers[Register::R2.usize()], content_base_reg); // R2 contains a memory address

        let res = handle_ldr(ldr_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(vm.registers[Register::R1.usize()], random_memory_content);
    }
}
