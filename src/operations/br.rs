use crate::{VMState, error::VMError, operations::utils::sign_extend, registers::Register};

/// Handler for instruction BRANCH, that evaluates conditions set in `n` (condition register is negative),
/// `z` (condition register is zero) and `p` (condition register is positive) and if they are met the program jumps
/// to the instruction specified in the address calculated by the current program counter (PC) value plus the offset that comes
/// in the instruction. More than one flag can be set to true (1) indicating that either of them is required.
//         | BR opcode (0000) |  n  |  z  |  p  | PC offset |
//         |   4 bits         |1 bit|1 bit|1 bit|   9 bits  |
pub fn handle_br(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    let cond_flag = (instruction >> 9) & 0x7;
    if (cond_flag & vm.registers[Register::Cond]) > 0 {
        vm.registers[Register::PC] = vm.registers[Register::PC].wrapping_add(pc_offset);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{VMState, flags::Flag, operations::br::handle_br, registers::Register};

    #[test]
    fn branches_if_zero() {
        // Set previous state
        let mut vm = VMState::init().unwrap(); // PC starts by default on 0x3000
        // Set the condition flag to not meet requirements
        vm.registers[Register::Cond] = Flag::Pos.try_into().unwrap();
        // BR   n z p  pc_offset
        // 0000 0 1 0  000001010
        let br_ix = 0x040A;
        let res = handle_br(br_ix, &mut vm);
        assert!(res.is_ok());
        // Verify state did not change because flag is not zero
        assert_eq!(vm.registers[Register::PC], 0x3000);

        // Set condition flag to zero
        vm.registers[Register::Cond] = Flag::Zro.try_into().unwrap();
        let res = handle_br(br_ix, &mut vm);
        assert!(res.is_ok());
        // Verify state changed to PC + 10 -> 0x3000 + 0x000A (offset) = 0x300A
        assert_eq!(vm.registers[Register::PC], 0x300A);
    }

    #[test]
    fn branches_if_positive() {
        let mut vm = VMState::init().unwrap(); // PC starts by default on 0x3000 and Condition Flag is ZERO.
        // BR   n z p  pc_offset
        // 0000 0 0 1  000001010
        let br_ix = 0x020A;
        let res = handle_br(br_ix, &mut vm);
        assert!(res.is_ok());
        // Verify state did not change because flag is not positive
        assert_eq!(vm.registers[Register::PC], 0x3000);
        // Set condition flag to positive
        vm.registers[Register::Cond] = Flag::Pos.try_into().unwrap();
        let res = handle_br(br_ix, &mut vm);
        assert!(res.is_ok());
        // Verify state changed to PC + 10 -> 0x3000 + 0x000A (offset) = 0x300A
        assert_eq!(vm.registers[Register::PC], 0x300A);
    }

    #[test]
    fn branches_if_negative() {
        let mut vm = VMState::init().unwrap(); // PC starts by default on 0x3000 and Condition Flag is ZERO.
        // BR   n z p  pc_offset
        // 0000 1 0 0  000001010
        let br_ix = 0x080A;
        let res = handle_br(br_ix, &mut vm);
        assert!(res.is_ok());
        // Verify state did not change because flag is not negative
        assert_eq!(vm.registers[Register::PC], 0x3000);
        // Set condition flag to negative
        vm.registers[Register::Cond] = Flag::Neg.try_into().unwrap();
        let res = handle_br(br_ix, &mut vm);
        assert!(res.is_ok());
        // Verify state changed to PC + 10 -> 0x3000 + 0x000A (offset) = 0x300A
        assert_eq!(vm.registers[Register::PC], 0x300A);
    }
}
