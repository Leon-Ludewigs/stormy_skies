use std::sync::Arc;
use crate::data::{compass_direction, CompassDirection, Coordinates, Percentage, percentage, Pressure, Speed, Temperature, Weather, wmo_code, WmoCode};
use crate::data::weather::WeatherRegistry;

#[derive(Clone, Debug)]
pub struct WeatherData {
    pub current_weather: Weather,
    pub current_temperature: Temperature,
    pub current_wind_speed: Speed,
    pub current_wind_direction: CompassDirection,
    pub current_pressure: Pressure,
    pub current_humidity: Percentage,
}

mod api_response {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Current {
        pub weather_code: u8,
        pub temperature_2m: f32,
        pub wind_speed_10m: f32,
        pub wind_direction_10m: f32,
        pub surface_pressure: f32,
        pub relative_humidity_2m: u8,
    }

    #[derive(Debug, Deserialize)]
    pub struct Response {
        pub current: Current,
    }
}

pub async fn call_api(weather_registry: &WeatherRegistry,
                      coordinates: Coordinates) -> Result<WeatherData, Error> {

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=weather_code,temperature_2m,wind_speed_10m,wind_direction_10m,surface_pressure,relative_humidity_2m",
        f32::from(coordinates.latitude),
        f32::from(coordinates.longitude),
    );

    let api_response = reqwasm::http::Request::get(&url)
        .send()
        .await?
        .json::<api_response::Response>()
        .await?;

    let current_wmo_code = WmoCode::try_from(api_response.current.weather_code)?;
    let current_weather = weather_registry.get(current_wmo_code);

    let current_temperature = Temperature::Celsius(api_response.current.temperature_2m);
    let current_wind_speed = Speed::KilometersPerHour(api_response.current.wind_speed_10m);
    let current_wind_direction = CompassDirection::from_degrees(api_response.current.wind_direction_10m)?;
    let current_pressure = Pressure::HectoPascal(api_response.current.surface_pressure);
    let current_humidity = Percentage::try_from(api_response.current.relative_humidity_2m)?;

    Ok(WeatherData {
        current_weather,
        current_temperature,
        current_wind_speed,
        current_wind_direction,
        current_pressure,
        current_humidity,
    })
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid WMO code: {0}")]
    InvalidWmoCode(#[from] wmo_code::Error),

    #[error("The obtained WMO code ({:?}) is not contained in the registry used", u8::from(*.0))]
    WmoCodeNotRegistered(WmoCode),

    #[error("The obtained direction is invalid: {0}")]
    InvalidCompassDirection(#[from] compass_direction::Error),

    #[error("The obtained percentage value is invalid: {0}")]
    InvalidPercentage(#[from] percentage::Error),

    #[error("The Open-Meteo API could not be called successfully: {0}")]
    ApiCall(Arc<reqwasm::Error>),
}

impl From<reqwasm::Error> for Error {
    fn from(error: reqwasm::Error) -> Self {
        Error::ApiCall(Arc::new(error))
    }
}
