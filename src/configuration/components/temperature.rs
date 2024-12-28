use serde::Deserialize;
use sysinfo::{Components, System};

use crate::{component::section::Component, configuration::converter::Converter};

const NAME: &str = "TEMP";
const ICON: &str = "󰏈 ";
const TIME: u64 = 1000;

#[derive(Deserialize, Debug)]
pub struct Temperature {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
}

impl Converter for Temperature {
    fn convert(&self, _sys: &mut System) -> anyhow::Result<Component> {
        let components = Components::new_with_refreshed_list();
        let total = components.iter().map(|c| c.temperature()).sum::<f32>();
        let total = total as usize / components.len();
        let total = format!("{}°C", total);
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_temperature_get_time() {
        let temperature = Temperature {
            time: Some(2000),
            name: None,
            icon: None,
        };
        assert_eq!(temperature.time(), 2000);

        let temperature_default = Temperature::default();
        assert_eq!(temperature_default.time(), TIME);
    }

    #[test]
    fn test_temperature_convert() {
        let mut sys = System::new_all();

        sys.refresh_all();

        let temperature = Temperature {
            time: Some(1000),
            name: Some(String::from("Current Temperature")),
            icon: Some(String::from(ICON)),
        };

        let component = temperature.convert(&mut sys).unwrap();

        assert_eq!(component.name, "Current Temperature");
        assert_eq!(component.icon, ICON);
    }
}
