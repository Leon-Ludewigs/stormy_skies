use std::fmt::{self, Display, Formatter};
use crate::data::Measurement;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Temperature {
    Celsius(f32),
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let unit = self.unit();

        let padded_unit = if unit.starts_with('°') {
            unit.to_owned()
        } else {
            format!(" {}", unit)
        };

        write!(f, "{}{}", f32::from(*self), padded_unit)
    }
}

impl From<Temperature> for f32 {
    fn from(value: Temperature) -> Self {
        match value {
            Temperature::Celsius(value) => value,
        }
    }
}

impl Measurement for Temperature {
    fn unit(&self) -> &'static str {
        match self {
            Temperature::Celsius(_) => "°C",
        }
    }
}
