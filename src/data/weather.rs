use std::collections::HashMap;
use std::rc::Rc;
use serde::Deserialize;
use crate::data::WmoCode;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Weather {
    pub description: Rc<str>,
    pub icon_path: Rc<str>,
}

const REGISTRY_JSON: &str = include_str!("../../compile_time_configs/wmo_codes.json");

pub struct WeatherRegistry(HashMap<WmoCode, Weather>);

impl WeatherRegistry {
    pub fn load() -> Result<WeatherRegistry, serde_json::Error> {
        let map = serde_json::from_str::<HashMap<u8, Weather>>(REGISTRY_JSON)?;

        let map = map.into_iter().flat_map(|(wmo_code_value, weather)| {
            WmoCode::try_from(wmo_code_value).ok().map(|wmo_code| (wmo_code, weather))
        }).collect::<HashMap<_, _>>();

        Ok(WeatherRegistry(map))
    }

    pub fn get(&self, wmo_code: WmoCode) -> Option<Weather> {
        Some(self.0.get(&wmo_code)?.clone())
    }
}
