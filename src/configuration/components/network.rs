use reqwest::blocking::get;
use serde::Deserialize;
use sysinfo::System;

use crate::{component::section::Component, configuration::converter::Converter};

const NAME: &str = "NET";
const ICON_UP: &str = "󰀂 ";
const ICON_DOWN: &str = "󰯡 ";
const TIME: u64 = 1000;
const URL: &str = "https://www.google.com";

#[derive(Deserialize, Debug)]
pub struct Network {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon_up: Option<String>,
    pub icon_down: Option<String>,
}

impl Converter for Network {
    fn convert(&self, _sys: &mut System) -> anyhow::Result<Component> {
        let icon = match get(URL) {
            Ok(resp) => {
                if resp.status().is_success() {
                    self.icon_up.as_deref().unwrap_or(ICON_UP)
                } else {
                    self.icon_down.as_deref().unwrap_or(ICON_DOWN)
                }
            }
            Err(_) => self.icon_down.as_deref().unwrap_or(ICON_DOWN),
        };

        let name = self.name.as_deref().unwrap_or(NAME);

        Ok(Component {
            name,
            icon,
            value: String::from(""),
        })
    }

    fn get_time(&self) -> u64 {
        self.time.unwrap_or(TIME)
    }
}

impl Default for Network {
    fn default() -> Self {
        Self {
            time: Some(TIME),
            name: Some(String::from(NAME)),
            icon_up: Some(String::from(ICON_UP)),
            icon_down: Some(String::from(ICON_DOWN)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_network_get_time() {
        let network = Network {
            time: Some(2000),
            name: None,
            icon_up: None,
            icon_down: None,
        };
        assert_eq!(network.get_time(), 2000);

        let network_default = Network::default();
        assert_eq!(network_default.get_time(), TIME);
    }

    #[test]
    fn test_network_convert_success() {
        let network = Network {
            time: Some(1000),
            name: Some(String::from("Custom Network")),
            icon_up: Some(String::from(ICON_UP)),
            icon_down: Some(String::from(ICON_DOWN)),
        };

        let component = network.convert(&mut System::new_all()).unwrap();

        assert_eq!(component.name, "Custom Network");
        assert_eq!(component.icon, ICON_UP);
        assert_eq!(component.value, "");
    }
}
