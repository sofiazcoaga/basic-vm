use crate::{VMState, error::VMError, operations::utils::sign_extend, registers::Register};
pub fn handle_st(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let src_reg = ((instruction >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    vm.mem_write(
        vm.registers[Register::PC.usize()].wrapping_add(pc_offset),
        vm.registers[src_reg],
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{VMState, operations::st::handle_st, registers::Register};

    #[test]
    fn stores_reg_value_in_memory() {
        let mut vm = VMState::init().unwrap();
        let src_reg: usize = 3; // R3
        let random_content = 1234;
        vm.registers[src_reg] = random_content; // Add some random content to register R3
        let pc_offset = 10;
        let pc_content = vm.registers[Register::PC.usize()];

        // ST   src_reg pc_offset
        // 0011 011     000001010
        let st_ix = 0x360A;
        assert_eq!(vm.mem_read(pc_content.wrapping_add(pc_offset)).unwrap(), 0); // Memory Address is empty
        let res = handle_st(st_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(
            vm.mem_read(pc_content.wrapping_add(pc_offset)).unwrap(),
            random_content
        );
    }
}
