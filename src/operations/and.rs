use std::cmp::Ordering;

use crate::{
    VMState,
    error::VMError,
    operations::utils::{sign_extend, update_flags},
};

/// Handler for instruction AND, that performs _bitwise and_ of two numbers and stores the result in the destination register.
/// AND supports two modes: immediate mode, where the second operand is given by the instruction itself, and register mode,
/// where the second operand is obtained from a register.
//     Immediate mode:
//         | AND opcode (0101) | destination reg | 1st operand reg | imm flag (1) | 2nd operand |
//         |   4 bits          |      3 bits     |    3 bits       | 1 bit        | 5 bits      |
///     Register mode:
///         | AND opcode (0101) | destination reg | 1st operand reg | imm flag (0) | unused | 2nd operand reg |
///         |   4 bits          |      3 bits     |    3 bits       | 1 bit        | 2 bits |   3 bits        |
pub fn handle_and(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let src_reg_1 = ((instruction >> 6) & 0x7) as usize;
    let imm_mode = (instruction >> 5) & 0x1;

    let second_value = match imm_mode.cmp(&0) {
        Ordering::Greater => sign_extend(instruction & 0x1F, 5),
        _ => {
            let src_reg_2 = (instruction & 0x7) as usize;
            vm.registers[src_reg_2]
        }
    };
    vm.registers[dest_reg] = vm.registers[src_reg_1] & second_value;
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{VMState, operations::and::handle_and, registers::Register};

    #[test]
    fn immediate_mode_and() {
        let mut vm = VMState::init().unwrap();
        // AND  R5  R6  IMM 3
        // 0101 101 110 1   00011
        let and_ix = 0x5BA3;
        vm.registers[Register::R6.usize()] = 10;

        let res = handle_and(and_ix, &mut vm);
        assert!(res.is_ok());
        // 10 & 3 =
        // 0000 0000 0000 1010
        // 0000 0000 0000 0011
        //  0    0    0   0010 = 2
        assert_eq!(vm.registers[Register::R5.usize()], 2);
    }

    #[test]
    fn register_mode_and() {
        let mut vm = VMState::init().unwrap();
        // AND  R0  R1  REG Unused R2
        // 0101 000 001 0   00     010
        let and_ix = 0x5042;
        vm.registers[Register::R1.usize()] = 1234;
        vm.registers[Register::R2.usize()] = 734;

        let res = handle_and(and_ix, &mut vm);
        assert!(res.is_ok());

        // 1234 0000 0100 1101 0010
        //  734 0000 0010 1101 1110
        // res  0000 0000 1101 0010 -> 210
        assert_eq!(vm.registers[Register::R0.usize()], 210);
    }
}
