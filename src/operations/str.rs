use crate::{VMState, error::VMError, operations::utils::sign_extend};

/// Handler for instruction STORE FROM REGISTER. A memory address is calculated from adding
/// the content of the base register to the offset specified in the instruction. After this
/// the content in the source register gets stored in the previously calculated address.
//         | STR opcode (0111)| source reg | base reg | offset |
//         |   4 bits         |  3 bits    |   3 bits | 6 bits |
pub fn handle_str(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let src_reg = ((instruction >> 9) & 0x7) as usize;
    let base_reg = ((instruction >> 6) & 0x7) as usize;
    let offset = sign_extend(instruction & 0x3F, 6);
    vm.mem_write(
        vm.registers[base_reg].wrapping_add(offset),
        vm.registers[src_reg],
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{VMState, operations::str::handle_str, registers::Register};

    #[test]
    fn executes_str() {
        // Setup
        let mut vm = VMState::init().unwrap();
        let random_memory_addr = 0x4000;
        let random_content = 0x1234;
        let offset = 0x0002;
        vm.registers[Register::R3] = random_memory_addr;
        vm.registers[Register::R1] = random_content;
        let calculated_address = (random_memory_addr + offset) as usize;

        assert_eq!(vm.memory[calculated_address], 0); // Still has nothing up to this point.

        // STR  sr_reg (R1) base_reg (R3) offset (2)
        // 0111 001         011           000010
        let str_ix = 0x72C2;

        let res = handle_str(str_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(vm.memory[calculated_address], random_content);
    }
}
