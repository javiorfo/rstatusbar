use chrono::Local;
use serde::Deserialize;

use crate::{component::section::Component, configuration::device::Converter};

const FORMAT: &str = "%A %d/%m/%Y %H:%M";
const ICON: &str = " ";
const TIME: u64 = 1000;

#[derive(Deserialize, Debug)]
pub struct Date {
    pub time: Option<u64>,
    pub format: Option<String>,
    pub icon: Option<String>,
}

impl Converter for Date {
    fn convert(&self) -> anyhow::Result<Component> {
        let date_time = Local::now();
        let formatted = format!(
            "{}",
            date_time.format(self.format.as_deref().unwrap_or(FORMAT))
        );
        let icon = self.icon.as_deref().unwrap_or(ICON);

        Ok(Component {
            name: "",
            icon,
            value: formatted,
        })
    }

    fn time(&self) -> u64 {
        self.time.unwrap_or(TIME)
    }
}

impl Default for Date {
    fn default() -> Self {
        Self {
            time: Some(TIME),
            format: Some(String::from(FORMAT)),
            icon: Some(String::from(ICON)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_get_time() {
        let date = Date {
            time: Some(2000),
            format: None,
            icon: None,
        };
        assert_eq!(date.time(), 2000);

        let date_default = Date::default();
        assert_eq!(date_default.time(), TIME);
    }

    #[test]
    fn test_date_convert() {
        let date = Date {
            time: Some(1000),
            format: Some(String::from("%Y-%m-%d")),
            icon: Some(String::from(ICON)),
        };

        let component = date.convert().unwrap();

        assert_eq!(component.icon, ICON);
        assert_eq!(component.value.len(), 10);
    }

    #[test]
    fn test_date_convert_with_default_values() {
        let date = Date::default();

        let component = date.convert().unwrap();

        assert_eq!(component.icon, ICON);
    }

    #[test]
    fn test_date_convert_with_invalid_format() {
        let date = Date {
            time: Some(1000),
            format: Some(String::from("invalid_format")),
            icon: Some(String::from(ICON)),
        };

        let component = date.convert().unwrap();

        assert_eq!(component.icon, ICON);
        assert!(!component.value.is_empty());
    }
}
