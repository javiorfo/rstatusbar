use serde::Deserialize;
use sysinfo::{Components, System};

use crate::{component::section::Component, configuration::converter::Converter};

#[derive(Deserialize, Debug)]
pub struct Temperature {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
}

impl Converter for Temperature {
    fn convert(&self, _sys: &mut System) -> Component {
        let components = Components::new_with_refreshed_list();
        let total = components.iter().map(|c| c.temperature()).sum::<f32>();
        let total = total as usize / components.len();
        let total = format!("{}󰔄 ", total);
        let name = self.name.as_deref().unwrap_or("");
        let icon = self.icon.as_deref().unwrap_or("");
        Component {
            name,
            icon,
            value: total,
        }
    }

    fn get_time(&self) -> u64 {
        self.time.unwrap_or(500)
    }
}

impl Default for Temperature {
    fn default() -> Self {
        Self {
            time: Some(1000),
            name: Some(String::from("TEMP")),
            icon: Some(String::from("󰏈 ")),
        }
    }
}
