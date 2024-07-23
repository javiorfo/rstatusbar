use serde::Deserialize;
use sysinfo::{Disks, System};

use crate::{component::section::Component, configuration::converter::Converter};

#[derive(Deserialize, Debug)]
pub struct Disk {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub unit: Option<String>,
}

impl Converter for Disk {
    fn convert(&self, sys: &mut System) -> Component {
        sys.refresh_all();
        let disk = Disks::new_with_refreshed_list();
        let selected_disk = disk
            .iter()
            .find(|d| d.mount_point().to_str() == Some(self.unit.as_deref().unwrap_or("/")))
            .unwrap();

        let total_space = selected_disk.total_space() as f64;
        let total = (total_space - selected_disk.available_space() as f64) / total_space * 100.0;

        let total = format!("{:.0}%", total);
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

impl Default for Disk {
    fn default() -> Self {
        Self {
            time: Some(1000),
            name: Some(String::from("DISK")),
            icon: Some(String::from("󰋊 ")),
            unit: Some(String::from("/")),
        }
    }
}
