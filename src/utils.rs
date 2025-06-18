use std::{fs, io::Read, os::fd::AsRawFd};

use termios::{ECHO, ICANON, TCSANOW, Termios, tcsetattr};

use crate::error::VMError;

/// Allows to read a file from a specific path passed as argument. If the file is correctly read it returns a `Vec<u8>` containing
/// its content in bytes.
pub fn read_file(path: &str) -> Result<Vec<u8>, VMError> {
    let read_result = fs::read(path).map_err(|e| VMError::CouldNotReadFile(e.to_string()))?;
    Ok(read_result)
}

/// Reads one single byte from standard input and then returns it formatted as a u16.
pub fn get_char() -> Result<u16, VMError> {
    let mut buffer: [u8; 1] = [0];
    std::io::stdin()
        .read_exact(&mut buffer)
        .map_err(|e| VMError::CouldNotReadChar(e.to_string()))?;
    let char = buffer[0] as u16;

    Ok(char)
}

/// The purpose of this function is to disable input buffering in the termial running the VM, so every input byte gets sent individually.
/// It returns the previous state of the terminal so it can be restored after the VM finishes running.
pub fn disable_input_buffering() -> Result<Termios, VMError> {
    let fd = std::io::stdin().lock().as_raw_fd();
    let mut termios = Termios::from_fd(fd).map_err(|e| VMError::TermiosError(e.to_string()))?;
    let original_setup = termios;
    termios.c_lflag &= !ICANON & !ECHO;
    tcsetattr(fd, TCSANOW, &termios).unwrap();
    Ok(original_setup)
}

/// It sets the terminal's configuration to the ones passed as argument. The idea for this function is to work with `disable_input_buffering()`
/// that provides the original configurations before changing them as return value. This function should use that value to restore the terminal
/// setup to the initial one.
pub fn restore_terminal(termios: Termios) -> Result<(), VMError> {
    let fd = std::io::stdin().lock().as_raw_fd();
    tcsetattr(fd, TCSANOW, &termios).map_err(|e| VMError::TermiosError(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_file() {
        let path = "./binary-examples/2048.obj";
        let read_file = read_file(path).unwrap();
        assert_eq!(read_file.len(), 2276);
    }
}
