use crate::{VMState, error::VMError, operations::utils::sign_extend, registers::Register};

/// Handler for instruction STORE INDIRECT. A first address is calculated by adding the current
/// content of the PC and the offset given by the instruction. After that, a second address is
/// obtained by getting the content stored in the first calculated address. This final address is
/// the one where data in the source register will be stored.
//         | STI opcode (1011)| source reg | PC offset |
//         |   4 bits         |  3 bits    |   9 bits  |
pub fn handle_sti(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let src_reg = ((instruction >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    let address = vm.mem_read(vm.registers[Register::PC].wrapping_add(pc_offset))?;
    vm.mem_write(address, vm.registers[src_reg]);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{VMState, operations::sti::handle_sti, registers::Register};

    #[test]
    fn stores_reg_value_in_memory() {
        let mut vm = VMState::init().unwrap();
        let default_pc_content: u16 = 0x3000; // The default value the PC is initialized with.
        let pc_offset = 0x0009; // Let's set offset to 9.
        let random_content = 0x1234; // This content will be read as an address.
        vm.registers[Register::R1] = 0x1111;
        // STI  src_reg(R1) pc_offset
        // 1011 001         000001001
        let sti_ix = 0x0000;
        // Set the address that will be read.
        vm.mem_write(default_pc_content.wrapping_add(pc_offset), random_content);
        assert_eq!(vm.mem_read(random_content).unwrap(), 0); // The memory in this address should have no content yet.

        let res = handle_sti(sti_ix, &mut vm);
        assert!(res.is_ok());
        // The memory address 0x1234 should have the same content as R1
        assert_eq!(
            vm.mem_read(random_content).unwrap(),
            vm.mem_read(vm.registers[Register::R1]).unwrap()
        );
    }
}
