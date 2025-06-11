use crate::{VMState, error::VMError, registers::Register};
pub fn handle_jmp(instruction: u16, vm: &mut VMState) -> Result<(), VMError> {
    let base_reg = ((instruction >> 6) & 0x7) as usize;
    vm.registers[Register::PC.usize()] = vm.registers[base_reg];
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn executes_jump() {
        // JMP  fill BaseReg fill
        // 1100 000  001     000000
        let jmp_ix = 0xC040;
        let mut vm = VMState::init().unwrap();
        assert_eq!(vm.registers[Register::PC.usize()], 0x3000); //Default value
        vm.registers[Register::R1.usize()] = 0x3100;
        let res = handle_jmp(jmp_ix, &mut vm);
        assert!(res.is_ok());
        assert_eq!(vm.registers[Register::PC.usize()], 0x3100); 
    }

}
