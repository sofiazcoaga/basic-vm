use std::{fs, io::Read, os::fd::AsRawFd};

use termios::{ECHO, ICANON, TCSANOW, Termios, tcsetattr};

use crate::error::VMError;

pub fn read_file(path: &str) -> Result<Vec<u8>, VMError> {
    let read_result = fs::read(path).map_err(|e| VMError::CouldNotReadFile(e.to_string()))?;
    Ok(read_result)
}

pub fn get_char() -> Result<u16, VMError> {
    let mut buffer: [u8; 1] = [0];
    std::io::stdin()
        .read_exact(&mut buffer)
        .map_err(|e| VMError::CouldNotReadChar(e.to_string()))?;
    let char = buffer[0] as u16;

    Ok(char)
}

pub fn disable_input_buffering() -> Result<(), VMError> {
    let fd = std::io::stdin().lock().as_raw_fd();
    let mut termios = Termios::from_fd(fd).map_err(|e| VMError::TermiosError(e.to_string()))?;
    termios.c_lflag &= !ICANON & !ECHO;
    tcsetattr(fd, TCSANOW, &termios).unwrap();
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
