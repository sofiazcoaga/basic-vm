#[allow(dead_code)]
#[derive(Debug)]
pub enum VMError {
    ErrorFlushinStdout(String),
    WrongArgumentsLen,
    CouldNotReadChar(String),
    CouldNotReadFile(String),
    UnrecognizedOpcode,
    UnrecognizedTrapCode,
    TermiosError(String),
}
