use std::fmt::{self, Display, Formatter};
use crate::data::Measurement;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Percentage(u8);

impl TryFrom<u8> for Percentage {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 100 {
            Ok(Percentage(value))
        } else {
            Err(Error::InvalidValue(value))
        }
    }
}

impl From<Percentage> for f32 {
    fn from(value: Percentage) -> Self {
        (value.0 as f32) / 100.
    }
}

impl Display for Percentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.unit())
    }
}

impl Measurement for Percentage {
    fn unit(&self) -> &'static str {
        "%"
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("The value must lie in [0, 100], but it is {0}")]
    InvalidValue(u8),
}

#[cfg(test)]
mod tests {
    use crate::data::Percentage;

    #[test]
    fn from_valid_u8() {
        for i in 0..=100 {
            assert!(Percentage::try_from(i).is_ok());
        }
    }

    #[test]
    fn from_invalid_u8() {
        assert!(Percentage::try_from(101).is_err());
    }
}