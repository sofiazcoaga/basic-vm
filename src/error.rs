#[allow(dead_code)]
#[derive(Debug)]
pub enum VMError {
    /// Wrapper for stdout.flush() errors. The original error is contained inside as a string.
    ErrorFlushinStdout(String),
    /// The input arguments for `cargo run` are not the required amount. The first number indicates
    /// the expected amount and the second one the received amount of arguments.
    WrongArgumentsLen(usize, usize),
    /// Wrapper for stdin.read_exact() errors. The original error is contained inside as a string.
    CouldNotReadChar(String),
    /// Wrapper for std::read() errors. The original error is contained inside as a string.
    CouldNotReadFile(String),
    /// The opcode was not recognized. The u16 inside is the received code.
    UnrecognizedOpcode(u16),
    /// The trap code was not recognized. The u16 inside is the received code.
    UnrecognizedTrapCode(u16),
    /// Wrapper for Termios crate errors. The original error is contained inside as a string.
    TermiosError(String),
}
