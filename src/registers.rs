#[allow(dead_code)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    PC = 8,
    Cond = 9,
}
impl Register {
    pub const COUNT: usize = 10;

    pub fn usize(self) -> usize {
        self as usize
    }
}
