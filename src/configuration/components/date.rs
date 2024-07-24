use chrono::Local;
use serde::Deserialize;
use sysinfo::System;

use crate::{component::section::Component, configuration::converter::Converter};

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
    fn convert(&self, _sys: &mut System) -> Component {
        let date_time = Local::now();
        let formatted = format!(
            "{}",
            date_time.format(self.format.as_deref().unwrap_or(FORMAT))
        );
        let icon = self.icon.as_deref().unwrap_or(ICON);
        Component {
            name: "",
            icon,
            value: formatted,
        }
    }

    fn get_time(&self) -> u64 {
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
