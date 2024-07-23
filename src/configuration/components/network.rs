use reqwest::blocking::get;
use serde::Deserialize;
use sysinfo::System;

use crate::{component::section::Component, configuration::converter::Converter};

#[derive(Deserialize, Debug)]
pub struct Network {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon_up: Option<String>,
    pub icon_down: Option<String>,
}

impl Converter for Network {
    fn convert(&self, _sys: &mut System) -> Component {
        let icon = match get("https://www.google.com") {
            Ok(resp) => {
                if resp.status().is_success() {
                    self.icon_up.as_deref().unwrap_or("")
                } else {
                    self.icon_down.as_deref().unwrap_or("")
                }
            }
            Err(_) => self.icon_down.as_deref().unwrap_or(""),
        };

        let name = self.name.as_deref().unwrap_or("");

        Component {
            name,
            icon,
            value: String::from(""),
        }
    }

    fn get_time(&self) -> u64 {
        self.time.unwrap_or(500)
    }
}

impl Default for Network {
    fn default() -> Self {
        Self {
            time: Some(1000),
            name: Some(String::from("NET")),
            icon_up: Some(String::from("󰀂 ")),
            icon_down: Some(String::from("󰯡 ")),
        }
    }
}
