use std::fmt::{self, Display, Formatter};
use crate::data::Measurement;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Speed {
    KilometersPerHour(f32),
}

impl Display for Speed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", f32::from(*self), self.unit())
    }
}

impl From<Speed> for f32 {
    fn from(value: Speed) -> Self {
        match value {
            Speed::KilometersPerHour(value) => value,
        }
    }
}

impl Measurement for Speed {
    fn unit(&self) -> &'static str {
        match self {
            Speed::KilometersPerHour(_) => "km/h",
        }
    }
}
