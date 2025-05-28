use libc::{c_char, statvfs};
use serde::Deserialize;
use std::ffi::CString;

use crate::{component::section::Component, configuration::device::Converter};

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
    fn convert(&self) -> anyhow::Result<Component> {
        let c_path =
            CString::new(self.unit.as_deref().unwrap_or(UNIT)).map_err(anyhow::Error::msg)?;

        let mut stat: statvfs = unsafe { std::mem::zeroed() };
        let ret = unsafe { statvfs(c_path.as_ptr() as *const c_char, &mut stat) };

        let total = if ret != 0 {
            anyhow::bail!("Invalid Unit");
        } else {
            let total = stat.f_blocks * stat.f_frsize;
            let available = stat.f_bavail * stat.f_frsize;
            let used = total.saturating_sub(available);
            format!("{:.0}%", used * 100 / total)
        };

        let name = self.name.as_deref().unwrap_or(NAME);
        let icon = self.icon.as_deref().unwrap_or(ICON);

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

    #[test]
    fn test_disk_get_time() {
        let disk = Disk {
            time: Some(3000),
            name: None,
            icon: None,
            unit: None,
        };
        assert_eq!(disk.time(), 3000);

        let disk_default = Disk::default();
        assert_eq!(disk_default.time(), TIME);
    }

    #[test]
    fn test_disk_convert() {
        let disk = Disk {
            time: Some(2000),
            name: Some(String::from("Custom Disk")),
            icon: Some(String::from(ICON)),
            unit: Some(String::from("/")),
        };

        let component = disk.convert().unwrap();

        assert_eq!(component.name, "Custom Disk");
        assert_eq!(component.icon, ICON);
        assert!(component.value.ends_with("%"));
    }

    #[test]
    fn test_disk_convert_with_default_values() {
        let disk = Disk::default();

        let component = disk.convert().unwrap();

        assert_eq!(component.name, NAME);
        assert_eq!(component.icon, ICON);
        assert!(component.value.ends_with("%"));
    }

    #[test]
    fn test_disk_convert_with_invalid_unit() {
        let disk = Disk {
            time: Some(2000),
            name: Some(String::from("Invalid Disk")),
            icon: Some(String::from(ICON)),
            unit: Some(String::from("invalid_unit")),
        };

        let result = disk.convert();

        assert!(result.is_err());
    }
}
