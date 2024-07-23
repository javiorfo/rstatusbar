use serde::Deserialize;
use sysinfo::System;

use crate::{component::section::Component, configuration::converter::Converter};

#[derive(Deserialize, Debug)]
pub struct Memory {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
}

impl Converter for Memory {
    fn convert(&self, sys: &mut System) -> Component {
        sys.refresh_memory();
        let memory_perc = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;
        let total = format!("{:.0}%", memory_perc);
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
