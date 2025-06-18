use crate::error::VMError;

/// This flags indicate if the last updated register value is positive,
/// negative or zero.
pub enum Flag {
    Pos = 1 << 0,
    Zro = 1 << 1,
    Neg = 1 << 2,
}

impl TryInto<u16> for Flag {
    type Error = VMError;
    fn try_into(self) -> Result<u16, Self::Error> {
        Ok(self as u16)
    }
}
