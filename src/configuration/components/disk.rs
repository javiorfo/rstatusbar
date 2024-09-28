use serde::Deserialize;
use sysinfo::{Disks, System};

use crate::{component::section::Component, configuration::converter::Converter};

const NAME: &str = "DISK";
const ICON: &str = "󰋊 ";
const UNIT: &str = "/";
const TIME: u64 = 2000;

#[derive(Deserialize, Debug)]
pub struct Disk {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub unit: Option<String>,
}

impl Converter for Disk {
    fn convert(&self, sys: &mut System) -> anyhow::Result<Component> {
        sys.refresh_all();
        let disk = Disks::new_with_refreshed_list();
        let selected_disk = disk
            .iter()
            .find(|d| d.mount_point().to_str() == Some(self.unit.as_deref().unwrap_or(UNIT)))
            .ok_or_else(|| anyhow::anyhow!(format!("Disk: Not a valid unit {:?}", self.unit)))?;

        let total_space = selected_disk.total_space() as f64;
        let total = (total_space - selected_disk.available_space() as f64) / total_space * 100.0;

        let total = format!("{:.0}%", total);
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

impl Default for Disk {
    fn default() -> Self {
        Self {
            time: Some(TIME),
            name: Some(String::from(NAME)),
            icon: Some(String::from(ICON)),
            unit: Some(String::from(UNIT)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_disk_get_time() {
        let disk = Disk {
            time: Some(3000),
            name: None,
            icon: None,
            unit: None,
        };
        assert_eq!(disk.get_time(), 3000);

        let disk_default = Disk::default();
        assert_eq!(disk_default.get_time(), TIME);
    }

    #[test]
    fn test_disk_convert() {
        let mut sys = System::new_all();

        sys.refresh_all();

        let disk = Disk {
            time: Some(2000),
            name: Some(String::from("Custom Disk")),
            icon: Some(String::from(ICON)),
            unit: Some(String::from("/")),
        };

        let component = disk.convert(&mut sys).unwrap();

        assert_eq!(component.name, "Custom Disk");
        assert_eq!(component.icon, ICON);
        assert!(component.value.ends_with("%"));
    }

    #[test]
    fn test_disk_convert_with_default_values() {
        let mut sys = System::new_all();

        sys.refresh_all();

        let disk = Disk::default();

        let component = disk.convert(&mut sys).unwrap();

        assert_eq!(component.name, NAME);
        assert_eq!(component.icon, ICON);
        assert!(component.value.ends_with("%"));
    }

    #[test]
    fn test_disk_convert_with_invalid_unit() {
        let mut sys = System::new_all();

        sys.refresh_all();

        let disk = Disk {
            time: Some(2000),
            name: Some(String::from("Invalid Disk")),
            icon: Some(String::from(ICON)),
            unit: Some(String::from("invalid_unit")),
        };

        let result = disk.convert(&mut sys);

        assert!(result.is_err());
    }
}
