use crate::{
    VMState,
    error::VMError,
    operations::utils::{sign_extend, update_flags},
};

pub fn handle_add(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let src_reg_1 = ((instruction >> 6) & 0x7) as usize;
    let imm_mode = ((instruction >> 5) & 0x1) > 0;
    if imm_mode {
        // Immediate mode
        let imm_operand = sign_extend(instruction & 0x1F, 5);
        vm.registers[dest_reg] = vm.registers[src_reg_1].wrapping_add(imm_operand);
    } else {
        // Register mode
        let src_reg_2 = (instruction & 0x7) as usize;
        vm.registers[dest_reg] = vm.registers[src_reg_1].wrapping_add(vm.registers[src_reg_2]);
    }
    update_flags(vm, vm.registers[dest_reg])?;
    Ok(())
}

#[cfg(test)]
mod test {

    use crate::{flags::Flag, registers::Register};

    use super::*;

    #[test]
    fn add_register_mode_pos() {
        let mut vm = VMState::init().unwrap();
        // ADD  R0  R1  RegMode Emtpy bits  R2
        // 0001 000 001 0       00          010
        let add_ix: u16 = 0x1042;
        vm.registers[Register::R1.usize()] = 30;
        vm.registers[Register::R2.usize()] = 25;
        assert_eq!(vm.registers[Register::R0.usize()], 0);
        assert_eq!(
            vm.registers[Register::Cond.usize()],
            Flag::Zro.try_into().unwrap()
        );
        let res = handle_add(add_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(vm.registers[Register::R0.usize()], 55);
        assert_eq!(
            vm.registers[Register::Cond.usize()],
            Flag::Pos.try_into().unwrap()
        );
    }

    #[test]
    fn add_register_mode_neg() {
        let mut vm = VMState::init().unwrap();
        let add_ix: u16 = 0x1042;
        vm.registers[Register::R1.usize()] = 30;
        // The complement of -20
        vm.registers[Register::R2.usize()] = 65516;

        assert_eq!(vm.registers[Register::R0.usize()], 0);
        assert_eq!(
            vm.registers[Register::Cond.usize()],
            Flag::Zro.try_into().unwrap()
        );
        let res = handle_add(add_ix, &mut vm);
        assert!(res.is_ok());
        // The result of 30 + (-20)
        assert_eq!(vm.registers[Register::R0.usize()], 10);
        assert_eq!(
            vm.registers[Register::Cond.usize()],
            Flag::Pos.try_into().unwrap()
        );
    }

    #[test]
    fn add_register_mode_neg_2() {
        let mut vm = VMState::init().unwrap();
        let add_ix: u16 = 0x1042;
        vm.registers[Register::R1.usize()] = 30;
        // The complement of -150
        vm.registers[Register::R2.usize()] = 65386;
        assert_eq!(vm.registers[Register::R0.usize()], 0);
        assert_eq!(
            vm.registers[Register::Cond.usize()],
            Flag::Zro.try_into().unwrap()
        );
        let res = handle_add(add_ix, &mut vm);
        assert!(res.is_ok());
        // The complement of -120 which is the result of 30 + (-150)
        assert_eq!(vm.registers[Register::R0.usize()], 65416);
        // The flag indicates value stored is negative
        assert_eq!(
            vm.registers[Register::Cond.usize()],
            Flag::Neg.try_into().unwrap()
        );
    }

    #[test]
    fn add_immediate_mode_pos() {
        let mut vm = VMState::init().unwrap();
        // ADD  R2  R4  IMM 10
        // 0001 010 100 1   01010
        let add_ix: u16 = 0x152A;
        vm.registers[Register::R4.usize()] = 20;
        let res = handle_add(add_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(vm.registers[Register::R2.usize()], 30);
    }

    #[test]
    fn add_immediate_mode_neg() {
        let mut vm = VMState::init().unwrap();
        // ADD  R2  R4  IMM -12 (complemento)
        // 0001 010 100 1   10100
        let add_ix: u16 = 0x1534;
        vm.registers[Register::R4.usize()] = 20;
        let res = handle_add(add_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(vm.registers[Register::R2.usize()], 8);
    }
}
