#[allow(dead_code)]
#[derive(Debug)]
pub enum VMError {
    CouldNotReadChar(String),
    CouldNotReadFile(String),
    UnrecognizedOpcode,
    UnrecognizedTrapCode,
}
