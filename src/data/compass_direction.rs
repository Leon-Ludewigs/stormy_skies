use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompassDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl CompassDirection {
    pub fn from_degrees(degrees: f32) -> Result<Self, Error> {
        if !degrees.is_finite() {
            return Err(Error::InvalidDegrees(degrees));
        }

        let id = (((((degrees + 22.5) % 360.) + 360.) % 360.) as u32) / 45;

        let compass_direction = match id {
            0 => CompassDirection::N,
            1 => CompassDirection::NE,
            2 => CompassDirection::E,
            3 => CompassDirection::SE,
            4 => CompassDirection::S,
            5 => CompassDirection::SW,
            6 => CompassDirection::W,
            7 => CompassDirection::NW,
            _ => panic!("This should not be reachable")
        };

        Ok(compass_direction)
    }
}

impl Display for CompassDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            CompassDirection::N => "N",
            CompassDirection::NE => "NE",
            CompassDirection::E => "E",
            CompassDirection::SE => "SE",
            CompassDirection::S => "S",
            CompassDirection::SW => "SW",
            CompassDirection::W => "W",
            CompassDirection::NW => "NW",
        };

        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("The angle of {0}° cannot be converted to a CompassDirection")]
    InvalidDegrees(f32),
}

#[cfg(test)]
mod tests {
    use crate::data::CompassDirection;

    fn degrees_range(from_inclusive: f32, to_exclusive: f32) -> impl Iterator<Item=f32> {
        let from = (100. * from_inclusive) as i32;
        let to = (100. * to_exclusive) as i32;

        (from..to).map(|degrees| (degrees as f32) / 100.)
    }

    fn from_degrees_test(start_degrees: f32, expected: CompassDirection) {
        let end_degrees = start_degrees + 45.;

        for d in degrees_range(start_degrees - 360., end_degrees - 360.) {
            assert_eq!(CompassDirection::from_degrees(d).unwrap(), expected);
        }

        for d in degrees_range(start_degrees, end_degrees) {
            assert_eq!(CompassDirection::from_degrees(d).unwrap(), expected);
        }

        for d in degrees_range(start_degrees + 360., end_degrees + 360.) {
            assert_eq!(CompassDirection::from_degrees(d).unwrap(), expected);
        }
    }

    #[test]
    fn from_degrees_north() {
        from_degrees_test(-22.5, CompassDirection::N);
    }

    #[test]
    fn from_degrees_north_east() {
        from_degrees_test(22.5, CompassDirection::NE);
    }

    #[test]
    fn from_degrees_east() {
        from_degrees_test(67.5, CompassDirection::E);
    }

    #[test]
    fn from_degrees_south_east() {
        from_degrees_test(112.5, CompassDirection::SE);
    }

    #[test]
    fn from_degrees_south() {
        from_degrees_test(157.5, CompassDirection::S);
    }

    #[test]
    fn from_degrees_south_west() {
        from_degrees_test(202.5, CompassDirection::SW);
    }

    #[test]
    fn from_degrees_west() {
        from_degrees_test(247.5, CompassDirection::W);
    }

    #[test]
    fn from_degrees_north_west() {
        from_degrees_test(292.5, CompassDirection::NW);
    }

    #[test]
    fn from_invalid_degrees() {
        assert!(CompassDirection::from_degrees(f32::NAN).is_err());
        assert!(CompassDirection::from_degrees(f32::INFINITY).is_err());
        assert!(CompassDirection::from_degrees(f32::NEG_INFINITY).is_err());
    }
}
