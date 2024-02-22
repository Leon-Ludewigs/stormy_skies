use std::fmt::{self, Display, Formatter};

pub type Latitude = Coordinate<-9000, 9000, 'S', 'N'>;
pub type Longitude = Coordinate<-18000, 18000, 'W', 'E'>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coordinate<const MIN: i32, const MAX: i32, const NEG_PREF: char, const POS_PREF: char> {
    value_times_100: i32,
}

impl<const MIN: i32, const MAX: i32, const NEG_PREF: char, const POS_PREF: char>
TryFrom<f32> for Coordinate<MIN, MAX, NEG_PREF, POS_PREF> {
    type Error = Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(Error::InvalidFloat);
        }

        let min = (MIN as f32) / 100.;
        let max = (MAX as f32) / 100.;

        if (min..=max).contains(&value) {
            Ok(Coordinate { value_times_100: (100. * value) as i32 })
        } else {
            Err(Error::OutOfRange { min, max, actual: value })
        }
    }
}

impl<const MIN: i32, const MAX: i32, const NEG_PREF: char, const POS_PREF: char>
From<Coordinate<MIN, MAX, NEG_PREF, POS_PREF>> for f32 {
    fn from(coordinate: Coordinate<MIN, MAX, NEG_PREF, POS_PREF>) -> Self {
        (coordinate.value_times_100 as f32) / 100.
    }
}

impl<const MIN: i32, const MAX: i32, const NEG_PREF: char, const POS_PREF: char>
Display for Coordinate<MIN, MAX, NEG_PREF, POS_PREF> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value = f32::from(*self);

        write!(
            f,
            "{}°{}",
            value.abs(),
            if value < 0. { NEG_PREF } else { POS_PREF },
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The value must lie in [{min}, {max}], but it is {actual}")]
    OutOfRange { min: f32, max: f32, actual: f32 },

    #[error("Infinity and NaN can not be interpreted as a coordinate")]
    InvalidFloat,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Coordinates {
    pub latitude: Latitude,
    pub longitude: Longitude,
}
