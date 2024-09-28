use serde::Deserialize;
use sysinfo::System;

use crate::{component::section::Component, configuration::converter::Converter};

const NAME: &str = "CPU";
const ICON: &str = " ";
const TIME: u64 = 1000;

#[derive(Deserialize, Debug)]
pub struct Cpu {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
}

impl Converter for Cpu {
    fn convert(&self, sys: &mut System) -> anyhow::Result<Component> {
        sys.refresh_cpu_usage();
        let total = sys.global_cpu_info().cpu_usage() as usize;
        let total = format!("{}%", total);
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

impl Default for Cpu {
    fn default() -> Self {
        Self {
            time: Some(TIME),
            name: Some(String::from(NAME)),
            icon: Some(String::from(ICON)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_cpu_get_time() {
        let cpu = Cpu {
            time: Some(2000),
            name: None,
            icon: None,
        };
        assert_eq!(cpu.get_time(), 2000);

        let cpu_default = Cpu::default();
        assert_eq!(cpu_default.get_time(), TIME);
    }

    #[test]
    fn test_cpu_convert() {
        let mut sys = System::new_all();

        let cpu = Cpu {
            time: Some(1000),
            name: Some(String::from("Custom CPU")),
            icon: Some(String::from(ICON)),
        };

        let component = cpu.convert(&mut sys).unwrap();

        assert_eq!(component.name, "Custom CPU");
        assert_eq!(component.icon, ICON);
        assert!(component.value.ends_with("%"));
    }

    #[test]
    fn test_cpu_convert_with_default_values() {
        let mut sys = System::new_all();

        let cpu = Cpu::default();

        let component = cpu.convert(&mut sys).unwrap();

        assert_eq!(component.name, NAME);
        assert_eq!(component.icon, ICON);
        assert!(component.value.ends_with("%"));
    }
}
