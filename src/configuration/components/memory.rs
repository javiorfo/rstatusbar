use serde::Deserialize;
use sysinfo::System;

use crate::{component::section::Component, configuration::converter::Converter};

const NAME: &str = "RAM";
const ICON: &str = " ";
const TIME: u64 = 1000;

#[derive(Deserialize, Debug)]
pub struct Memory {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
}

impl Converter for Memory {
    fn convert(&self, sys: &mut System) -> anyhow::Result<Component> {
        sys.refresh_memory();
        let memory_perc = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;
        let total = format!("{:.0}%", memory_perc);
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

impl Default for Memory {
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
    fn test_memory_get_time() {
        let memory = Memory {
            time: Some(2000),
            name: None,
            icon: None,
        };
        assert_eq!(memory.time(), 2000);

        let memory_default = Memory::default();
        assert_eq!(memory_default.time(), TIME);
    }

    #[test]
    fn test_memory_convert() {
        let mut sys = System::new_all();

        sys.refresh_memory();

        let memory = Memory {
            time: Some(1000),
            name: Some(String::from("Custom RAM")),
            icon: Some(String::from(ICON)),
        };

        let component = memory.convert(&mut sys).unwrap();

        assert_eq!(component.name, "Custom RAM");
        assert_eq!(component.icon, ICON);
        assert!(component.value.ends_with("%"));
    }

    #[test]
    fn test_memory_convert_with_default_values() {
        let mut sys = System::new_all();

        sys.refresh_memory();

        let memory = Memory::default();

        let component = memory.convert(&mut sys).unwrap();

        assert_eq!(component.name, NAME);
        assert_eq!(component.icon, ICON);
        assert!(component.value.ends_with("%"));
    }
}
