use std::fs;

use serde::Deserialize;

use crate::{component::section::Component, configuration::device::Converter};

const NAME: &str = "BAT";
const ICON_FULL: &str = "󰁹";
const ICON_MEDIUM: &str = "󰁿";
const ICON_LOW: &str = "󰁺";
const TIME: u64 = 1000;
const PATH: &str = "/sys/class/power_supply/BAT0/capacity";

#[derive(Deserialize, Debug)]
pub struct Battery {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon_full: Option<String>,
    pub icon_medium: Option<String>,
    pub icon_low: Option<String>,
    pub path: Option<String>,
}

impl Converter for Battery {
    fn convert(&self) -> anyhow::Result<Component> {
        let battery_percentage = fs::read_to_string(self.path.clone().unwrap_or(PATH.to_string()))
            .map_err(anyhow::Error::msg)?;

        let battery_percentage: u8 = battery_percentage
            .trim()
            .parse()
            .map_err(anyhow::Error::msg)?;

        let total = format!("{battery_percentage}%");

        let name = self.name.as_deref().unwrap_or(NAME);

        let icon = if battery_percentage > 80 {
            self.icon_full.as_deref().unwrap_or(ICON_FULL)
        } else if battery_percentage > 40 {
            self.icon_medium.as_deref().unwrap_or(ICON_MEDIUM)
        } else {
            self.icon_low.as_deref().unwrap_or(ICON_LOW)
        };

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

impl Default for Battery {
    fn default() -> Self {
        Self {
            time: Some(TIME),
            name: Some(String::from(NAME)),
            icon_full: Some(String::from(ICON_FULL)),
            icon_medium: Some(String::from(ICON_MEDIUM)),
            icon_low: Some(String::from(ICON_LOW)),
            path: Some(String::from(PATH)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_convert_full_battery() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("capacity");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "90").unwrap();

        let battery = Battery {
            path: Some(path.to_string_lossy().to_string()),
            ..Default::default()
        };

        let component = battery.convert().unwrap();
        assert_eq!(component.name, NAME);
        assert_eq!(component.icon, ICON_FULL);
        assert_eq!(component.value, "90%");
    }

    #[test]
    fn test_convert_medium_battery() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("capacity");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "50").unwrap();

        let battery = Battery {
            path: Some(path.to_string_lossy().to_string()),
            ..Default::default()
        };

        let component = battery.convert().unwrap();
        assert_eq!(component.name, NAME);
        assert_eq!(component.icon, ICON_MEDIUM);
        assert_eq!(component.value, "50%");
    }

    #[test]
    fn test_convert_low_battery() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("capacity");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "20").unwrap();

        let battery = Battery {
            path: Some(path.to_string_lossy().to_string()),
            ..Default::default()
        };

        let component = battery.convert().unwrap();
        assert_eq!(component.name, NAME);
        assert_eq!(component.icon, ICON_LOW);
        assert_eq!(component.value, "20%");
    }

    #[test]
    fn test_convert_invalid_capacity() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("capacity");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "invalid").unwrap();

        let battery = Battery {
            path: Some(path.to_string_lossy().to_string()),
            ..Default::default()
        };

        let result = battery.convert();
        assert!(result.is_err());
    }
}
