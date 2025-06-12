use crate::{VMState, error::VMError, operations::utils::sign_extend, registers::Register};
/// Handle Jump to Subroutine
pub fn handle_jsr(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    // See if next ix address will be obtained from a register or an offset
    let without_reg_flag = ((instruction >> 11) & 1) > 0;
    // Store current PC in R7 (linker register)
    vm.registers[Register::R7.usize()] = vm.registers[Register::PC.usize()];

    if without_reg_flag {
        // Next ix address is obtained from adding offset to current PC. - JSR
        let pc_offset = sign_extend(instruction & 0x7FF, 11);
        vm.registers[Register::PC.usize()] += pc_offset;
    } else {
        // Next ix address is obtained from a specific register - JSRR
        let base_reg = ((instruction >> 6) & 0x7) as usize;
        vm.registers[Register::PC.usize()] = vm.registers[base_reg];
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{VMState, operations::jsr::handle_jsr, registers::Register};

    #[test]
    fn executes_jsr() {
        let mut vm = VMState::init().unwrap();
        assert_eq!(vm.registers[Register::PC.usize()], 0x3000); // Verify it is started with the default address.
        assert_eq!(vm.registers[Register::R7.usize()], 0); // Linker register has no value yet
        // JSR  FromOffsetFlag  Offset = 5
        // 0100 1               00000000101
        let jsr_ix = 0x4805;
        let res = handle_jsr(jsr_ix, &mut vm);
        assert!(res.is_ok());
        // PC was moved 5 ixs.
        assert_eq!(vm.registers[Register::PC.usize()], 0x3005);
        // R7 is previous PC value.
        assert_eq!(vm.registers[Register::R7.usize()], 0x3000);
    }

    #[test]
    fn executes_jsrr() {
        let mut vm = VMState::init().unwrap();
        vm.registers[Register::R2.usize()] = 0x3010;
        assert_eq!(vm.registers[Register::PC.usize()], 0x3000); // Verify it is started with the default address.
        assert_eq!(vm.registers[Register::R7.usize()], 0); // Linker register has no value yet
        assert_eq!(vm.registers[Register::R2.usize()], 0x3010);
        // JSR  FromOffsetFlag  Unused  BaseReg  Unused
        // 0100 0               00      010      000000
        let jsrr_ix = 0x4080;
        let res = handle_jsr(jsrr_ix, &mut vm);
        assert!(res.is_ok());
        // PC was moved to 0x3010.
        assert_eq!(vm.registers[Register::PC.usize()], 0x3010);
        // R7 is previous PC value.
        assert_eq!(vm.registers[Register::R7.usize()], 0x3000);
    }
}
