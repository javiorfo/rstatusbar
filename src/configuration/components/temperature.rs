use std::{fs, path::PathBuf};

use serde::Deserialize;

use crate::{component::section::Component, configuration::device::Converter};

const NAME: &str = "TEMP";
const ICON: &str = "󰏈 ";
const TIME: u64 = 1000;
const ZONE: u8 = 0;

#[derive(Deserialize, Debug)]
pub struct Temperature {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub zone: Option<u8>,
}

impl Converter for Temperature {
    fn convert(&self) -> anyhow::Result<Component<'_>> {
        let path = PathBuf::from(format!(
            "/sys/class/thermal/thermal_zone{}/temp",
            self.zone.unwrap_or(ZONE)
        ));

        let contents = fs::read_to_string(path).map_err(anyhow::Error::msg)?;
        let temp_milli: i64 = contents.trim().parse().map_err(anyhow::Error::msg)?;

        let total = format!("{:.0}°C", temp_milli as f32 / 1000.0);
        let name = self.name.as_deref().unwrap_or(NAME);
        let icon = self.icon.as_deref().unwrap_or(ICON);

        Ok(Component {
            name,
            icon,
            value: total,
        })
    }

    fn time(&self) -> u64 {
        self.time.unwrap_or(TIME)
    }
}

impl Default for Temperature {
    fn default() -> Self {
        Self {
            time: Some(TIME),
            name: Some(String::from(NAME)),
            icon: Some(String::from(ICON)),
            zone: Some(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_get_time() {
        let temperature = Temperature {
            time: Some(2000),
            name: None,
            icon: None,
            zone: None,
        };
        assert_eq!(temperature.time(), 2000);

        let temperature_default = Temperature::default();
        assert_eq!(temperature_default.time(), TIME);
    }

    #[test]
    fn test_temperature_convert() {
        let temperature = Temperature {
            time: Some(1000),
            name: Some(String::from("Current Temperature")),
            icon: Some(String::from(ICON)),
            zone: Some(2),
        };

        let component = temperature.convert().unwrap();

        assert_eq!(component.name, "Current Temperature");
        assert_eq!(component.icon, ICON);
    }
}
