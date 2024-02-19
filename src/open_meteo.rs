use thiserror::Error;
use crate::data::{Coordinates, Weather, weather};

#[derive(Debug)]
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
        coordinates.latitude(),
        coordinates.longitude(),
    );

    let api_response = reqwasm::http::Request::get(&url)
        .send()
        .await?
        .json::<api_response::Response>()
        .await?;

    let weather = Weather::from_wmo_code(api_response.current.weather_code)?;

    Ok(WeatherData { weather })
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("The weather could not be interpreted correctly: {0}")]
    WeatherError(#[from] weather::Error),

    #[error("The Open-Meteo API could not be called successfully: {0}")]
    ApiCall(#[from] reqwasm::Error),
}
