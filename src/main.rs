use std::env;

mod error;
mod flags;
mod opcodes;
mod operations;
mod registers;
mod utils;
mod vm;

use crate::error::VMError;

use crate::utils::read_file;
use crate::vm::VMState;

fn main() -> Result<(), VMError> {
    let expected_arguments_len = 2;

    // Get terminal arguments to obtain the path to the binary file to be executed.
    let console_args: Vec<_> = env::args().collect();
    // Arguments length must be two - the first argument is for cargo and the second should be the path.
    if console_args.len() != expected_arguments_len {
        return Err(VMError::WrongArgumentsLen(
            expected_arguments_len,
            console_args.len(),
        ));
    }
    let path = console_args[1].clone();

    // Read the file.
    let file = read_file(&path)?;

    // Initialize VM state with default values
    let mut vm = VMState::init()?;

    vm.run(file)?;

    Ok(())
}
