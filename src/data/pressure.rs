use std::fmt::{self, Display, Formatter};
use crate::data::Measurement;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pressure {
    HectoPascal(f32),
}

impl Display for Pressure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", f32::from(*self), self.unit())
    }
}

impl From<Pressure> for f32 {
    fn from(value: Pressure) -> Self {
        match value {
            Pressure::HectoPascal(value) => value,
        }
    }
}

impl Measurement for Pressure {
    fn unit(&self) -> &'static str {
        match self {
            Pressure::HectoPascal(_) => "hPa",
        }
    }
}
