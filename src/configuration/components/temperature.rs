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
        let total = format!("{}󰔄 ", total);
        let name = self.name.as_deref().unwrap_or(NAME);
        let icon = self.icon.as_deref().unwrap_or(ICON);

        Ok(Component {
            name,
            icon,
            value: total,
        })
    }

    fn get_time(&self) -> u64 {
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
