use std::process::Command;

use serde::Deserialize;
use sysinfo::System;

use crate::{component::section::Component, configuration::converter::Converter};

const NAME: &str = "WEA";
const ICON: &str = " ";
const TIME: u64 = 1800000;

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub location: String,
}

impl Converter for Weather {
    fn convert(&self, _sys: &mut System) -> anyhow::Result<Component> {
        let name = self.name.as_deref().unwrap_or(NAME);
        let icon = self.icon.as_deref().unwrap_or(ICON);

        let output = Command::new("curl")
            .arg(format!("wttr.in/{}?format=%t", self.location))
            .output()
            .map_err(anyhow::Error::msg)?;

        let temp = if output.status.success() {
            let mut res = String::from_utf8_lossy(&output.stdout).to_string().replace("+", "");
            res.truncate(5);
            res
        } else {
            String::from("-")
        };

        Ok(Component {
            name,
            icon,
            value: temp,
        })
    }

    fn time(&self) -> u64 {
        self.time.unwrap_or(TIME)
    }
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            time: Some(TIME),
            name: Some(String::from(NAME)),
            icon: Some(String::from(ICON)),
            location: String::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_convert_success() {
        let weather = Weather {
            time: Some(TIME),
            name: Some(String::from(NAME)),
            icon: Some(String::from(ICON)),
            location: String::from("Buenos+Aires"),
        };

        let mut sys = System::new_all();

        let component = weather.convert(&mut sys).unwrap();

        assert_eq!(component.name, NAME);
        assert_eq!(component.icon, ICON);
        assert!(!component.value.is_empty());
        assert!(component.value.len() > 4);
    }

    #[test]
    fn test_convert_failure() {
        let weather = Weather::default();

        let mut sys = System::new_all();

        let component = weather.convert(&mut sys).unwrap();

        assert_eq!(component.name, NAME);
        assert_eq!(component.icon, ICON);
        assert!(!component.value.is_empty());
    }
}

