use crate::{VMState, error::VMError, operations::utils::sign_extend, registers::Register};

/// Handler for instruction JUMP TO SUBROUTINE. It allows the program to unconditionally
/// jump to a subroutine, storing the previous context first (to come back when the subroutine
/// has finished). It allows two modes:
/// - without register (JSR): the address of the first instruction of the subroutine is obtained by calculating
///   the addition of the current content of the PC and the offset in the instruction.
/// - with register (JSRR): the address of the first instruction of the subroutine is inside the base register.
// JSR:
//         | JSR opcode (0100) | no reg flag (1) | PC offset |
//         |   4 bits          | 1 bit           | 11 bits   |
// JSRR:
//         | JSR opcode (0100) | no reg flag (0) | unused | base reg | unused |
//         |   4 bits          | 1 bit           | 2 bits | 3 bits   | 6 bits |
pub fn handle_jsr(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    // See if next ix address will be obtained from a register or an offset
    let without_reg_flag = ((instruction >> 11) & 1) > 0;
    // Store current PC in R7 (linker register)
    vm.registers[Register::R7] = vm.registers[Register::PC];

    if without_reg_flag {
        // Next ix address is obtained from adding offset to current PC. - JSR
        let pc_offset = sign_extend(instruction & 0x7FF, 11);
        vm.registers[Register::PC] = vm.registers[Register::PC].wrapping_add(pc_offset);
    } else {
        // Next ix address is obtained from a specific register - JSRR
        let base_reg = ((instruction >> 6) & 0x7) as usize;
        vm.registers[Register::PC] = vm.registers[base_reg];
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{VMState, operations::jsr::handle_jsr, registers::Register};

    #[test]
    fn executes_jsr() {
        let mut vm = VMState::init().unwrap();
        assert_eq!(vm.registers[Register::PC], 0x3000); // Verify it is started with the default address.
        assert_eq!(vm.registers[Register::R7], 0); // Linker register has no value yet
        // JSR  FromOffsetFlag  Offset = 5
        // 0100 1               00000000101
        let jsr_ix = 0x4805;
        let res = handle_jsr(jsr_ix, &mut vm);
        assert!(res.is_ok());
        // PC was moved 5 ixs.
        assert_eq!(vm.registers[Register::PC], 0x3005);
        // R7 is previous PC value.
        assert_eq!(vm.registers[Register::R7], 0x3000);
    }

    #[test]
    fn executes_jsr_with_negative_offset() {
        // JSR  FromOffset  Complement of -20 // we want to move the PC backwards
        // 0100 1           11111101100
        let jsr_ix = 0x4FEC;
        let mut vm = VMState::init().unwrap();
        let pc_previous_value = vm.registers[Register::PC];
        let _ = handle_jsr(jsr_ix, &mut vm);
        assert_eq!(vm.registers[Register::PC], pc_previous_value - 20);
    }

    #[test]
    fn executes_jsrr() {
        let mut vm = VMState::init().unwrap();
        vm.registers[Register::R2] = 0x3010;
        assert_eq!(vm.registers[Register::PC], 0x3000); // Verify it is started with the default address.
        assert_eq!(vm.registers[Register::R7], 0); // Linker register has no value yet
        assert_eq!(vm.registers[Register::R2], 0x3010);
        // JSR  FromOffsetFlag  Unused  BaseReg  Unused
        // 0100 0               00      010      000000
        let jsrr_ix = 0x4080;
        let res = handle_jsr(jsrr_ix, &mut vm);
        assert!(res.is_ok());
        // PC was moved to 0x3010.
        assert_eq!(vm.registers[Register::PC], 0x3010);
        // R7 is previous PC value.
        assert_eq!(vm.registers[Register::R7], 0x3000);
    }
}
