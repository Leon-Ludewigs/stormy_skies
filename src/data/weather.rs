use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::data::{wmo_code, WmoCode};

#[derive(Clone, Debug)]
pub struct Weather {
    pub description: Arc<str>,
    pub icon_path: Arc<str>,
}

// Registry source files
const WMO_JSON: &str = include_str!("../../compile_time_configs/wmo_codes.json");
const DESCRIPTION_JSON: &str = include_str!("../../compile_time_configs/weather_descriptions.json");
const ICON_FILE_NAMES_JSON: &str = include_str!("../../compile_time_configs/icon_file_names.json");

#[derive(Debug)]
pub struct WeatherRegistry {
    descriptions: HashMap<WmoCode, Arc<str>>,
    icon_paths: HashMap<WmoCode, Arc<str>>,
}

impl WeatherRegistry {
    pub fn load() -> Result<WeatherRegistry, Error> {
        let wmo_code_values = serde_json::from_str::<HashMap<u8, String>>(WMO_JSON)?;
        let descriptions_from_keys = serde_json::from_str::<HashMap<String, String>>(DESCRIPTION_JSON)?;
        let icon_file_names_from_keys = serde_json::from_str::<HashMap<String, String>>(ICON_FILE_NAMES_JSON)?;

        let wmo_codes = wmo_code_values.into_iter().map(|(wmo_code_value, weather)| {
            WmoCode::try_from(wmo_code_value).map(|wmo_code| (wmo_code, weather))
        }).collect::<Result<HashMap<_, _>, _>>()?;

        for wmo_code in (0..=wmo_code::MAX_VALUE).flat_map(WmoCode::try_from) {
            if !wmo_codes.contains_key(&wmo_code) {
                return Err(Error::WmoCodeDefinitionMissing(wmo_code));
            }
        }

        let valid_keys = wmo_codes.values().collect::<HashSet<_>>();

        for key in descriptions_from_keys.keys() {
            if !valid_keys.contains(key) {
                return Err(Error::DescriptionForUndefinedKey(key.to_owned()));
            }
        }

        for key in icon_file_names_from_keys.keys() {
            if !valid_keys.contains(key) {
                return Err(Error::IconPathForUndefinedKey(key.to_owned()));
            }
        }

        let mut descriptions: HashMap<WmoCode, Arc<str>> = HashMap::new();
        let mut icon_paths: HashMap<WmoCode, Arc<str>> = HashMap::new();

        for (wmo_code, key) in wmo_codes {
            let description = descriptions_from_keys
                .get(&key)
                .ok_or(Error::DescriptionMissing(key.clone()))?
                .clone();

            descriptions.insert(wmo_code, description.into());

            let icon_file_name = icon_file_names_from_keys
                .get(&key)
                .ok_or(Error::IconPathMissing(key.clone()))?;

            let icon_path = format!("assets/icons/{}.svg", icon_file_name);
            icon_paths.insert(wmo_code, icon_path.into());
        }

        let weather_registry = WeatherRegistry { descriptions, icon_paths };

        Ok(weather_registry)
    }

    pub fn get(&self, wmo_code: WmoCode) -> Weather {
        Weather {
            description: self.descriptions.get(&wmo_code).unwrap().clone(),
            icon_path: self.icon_paths.get(&wmo_code).unwrap().clone(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to deserialize the source file: {0}")]
    FailedToDeserializeSources(#[from] serde_json::Error),

    #[error("Invalid value for WMO code: {0}")]
    InvalidValueForWmoCode(#[from] wmo_code::Error),

    #[error("WMO code definition missing for code {}", u8::from(*.0))]
    WmoCodeDefinitionMissing(WmoCode),

    #[error("Description defined for non-existent key {0}")]
    DescriptionForUndefinedKey(String),

    #[error("Description missing for key {0}")]
    DescriptionMissing(String),

    #[error("Icon path defined for non-existent key {0}")]
    IconPathForUndefinedKey(String),

    #[error("Icon path missing for key {0}")]
    IconPathMissing(String),
}

#[cfg(test)]
mod weather_tests {
    use crate::data::WeatherRegistry;

    #[test]
    fn weather_registry_loads_without_errors() {
        assert!(WeatherRegistry::load().is_ok());
    }
}
