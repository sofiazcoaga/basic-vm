use crate::{VMState, error::VMError};

pub fn handle_not(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let dest_reg = ((instruction >> 9) & 0x7) as usize;
    let src_reg = ((instruction >> 6) & 0x7) as usize;
    vm.registers[dest_reg] = !vm.registers[src_reg];
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{VMState, registers::Register};

    #[test]
    fn inverts_register_with_not() {
        let mut vm = VMState::init().unwrap();
        // NOT  R1  R1  FILL
        // 1001 001 001 111111
        let not_ix = 0x927F;
        vm.registers[Register::R1.usize()] = 0;
        let res = handle_not(not_ix, &mut vm);
        assert!(res.is_ok());
        //  0 - 0000 0000 0000 0000
        // !0 - 1111 1111 1111 1111 - MAX VALUE 2^16 - 1 = 65.535
        assert_eq!(vm.registers[Register::R1.usize()], 65535);
    }
}
