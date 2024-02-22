use std::rc::Rc;
use crate::data::{Coordinates, Weather, weather};

#[derive(Clone, Debug)]
pub struct WeatherData {
    pub weather: Weather,
}

mod api_response {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Current {
        pub weather_code: u8,
    }

    #[derive(Debug, Deserialize)]
    pub struct Response {
        pub current: Current,
    }
}

pub async fn call_api(coordinates: Coordinates) -> Result<WeatherData, Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=weather_code",
        f32::from(coordinates.latitude),
        f32::from(coordinates.longitude),
    );

    let api_response = reqwasm::http::Request::get(&url)
        .send()
        .await?
        .json::<api_response::Response>()
        .await?;

    let weather = Weather::from_wmo_code(api_response.current.weather_code)?;

    Ok(WeatherData { weather })
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("The weather could not be interpreted correctly: {0}")]
    WeatherError(#[from] weather::Error),

    #[error("The Open-Meteo API could not be called successfully: {0}")]
    ApiCall(Rc<reqwasm::Error>),
}

// TODO is there a way to do this with a macro from this-error?
impl From<reqwasm::Error> for Error {
    fn from(error: reqwasm::Error) -> Self {
        Error::ApiCall(Rc::new(error))
    }
}
