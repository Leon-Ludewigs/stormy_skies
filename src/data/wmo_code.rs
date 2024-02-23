pub const MAX_VALUE: u8 = 99;

/// Code following the WMO interpretation standard as documented in
/// https://www.nodc.noaa.gov/archive/arc0021/0002199/1.1/data/0-data/HTML/WMO-CODE/WMO4677.HTM
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WmoCode(u8);

impl TryFrom<u8> for WmoCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > MAX_VALUE {
            Err(Error::InvalidValue(value))
        } else { 
            Ok(WmoCode(value))
        }
    }
}

impl From<WmoCode> for u8 {
    fn from(value: WmoCode) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("The value must be <= {MAX_VALUE}, but is {0}")]
    InvalidValue(u8),
}
