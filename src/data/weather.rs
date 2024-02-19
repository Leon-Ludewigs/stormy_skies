use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Weather {
    /// Code following the WMO interpretation standard as documented in
    /// https://www.nodc.noaa.gov/archive/arc0021/0002199/1.1/data/0-data/HTML/WMO-CODE/WMO4677.HTM
    wmo_code: u8,
}

impl Weather {
    pub fn from_wmo_code(wmo_code: u8) -> Result<Weather, Error> {
        if wmo_code <= 99 {
            Ok(Weather { wmo_code })
        } else {
            Err(Error::InvalidWmoCode(wmo_code))
        }
    }

    pub fn wmo_code(&self) -> u8 {
        self.wmo_code
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("The code {0} is not a valid WMO weather interpretation code")]
    InvalidWmoCode(u8),
}
