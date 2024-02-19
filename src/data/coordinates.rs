use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coordinates {
    latitude_times_100: i32,
    longitude_times_100: i32,
}

impl Coordinates {
    pub const MIN_LATITUDE: f32 = -90.;
    pub const MAX_LATITUDE: f32 = 90.;
    pub const MIN_LONGITUDE: f32 = -180.;
    pub const MAX_LONGITUDE: f32 = 180.;

    pub fn from_latitude_longitude(latitude: f32,
                                   longitude: f32) -> Result<Coordinates, Error> {
        if !(Self::MIN_LATITUDE..=Self::MAX_LATITUDE).contains(&latitude) {
            Err(Error::InvalidLatitude(latitude))
        } else if !(Self::MIN_LONGITUDE..Self::MAX_LONGITUDE).contains(&longitude) {
            Err(Error::InvalidLongitude(longitude))
        } else {
            let latitude_times_100 = (100. * latitude) as i32;
            let longitude_times_100 = (100. * longitude) as i32;

            Ok(Coordinates { latitude_times_100, longitude_times_100 })
        }
    }

    pub fn latitude(&self) -> f32 {
        (self.latitude_times_100 as f32) / 100.
    }

    pub fn longitude(&self) -> f32 {
        (self.longitude_times_100 as f32) / 100.
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}°{}, {}°{}",
            self.latitude().abs(),
            if self.latitude() < 0. { 'S' } else { 'N' },
            self.longitude().abs(),
            if self.longitude() < 0. { 'W' } else { 'E' },
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The latitude has to be inside [{}, {}], but it is {0}", Coordinates::MIN_LATITUDE, Coordinates::MAX_LATITUDE)]
    InvalidLatitude(f32),

    #[error("The longitude has to be inside [{}, {}), but it is {0}", Coordinates::MIN_LONGITUDE, Coordinates::MAX_LONGITUDE)]
    InvalidLongitude(f32),
}
