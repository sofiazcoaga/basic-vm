#[allow(dead_code)]
#[derive(Debug)]
pub enum VMError {
    ErrorFlushingStdout(String),
    CouldNotReadChar(String),
    CouldNotReadFile(String),
    UnrecognizedOpcode,
    UnrecognizedTrapCode,
}
