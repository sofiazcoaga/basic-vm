#[allow(dead_code)]
#[derive(Debug)]
pub enum VMError {
    CouldNotReadFile(String),
    UnrecognizedOpcode,
}
