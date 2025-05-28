use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use serde::Deserialize;

use crate::{component::section::Component, configuration::device::Converter};

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
    fn convert(&self) -> anyhow::Result<Component> {
        let file = File::open("/proc/meminfo").map_err(anyhow::Error::msg)?;
        let reader = BufReader::new(file);

        let mut mem_total = 0u64;
        let mut mem_free = 0u64;
        let mut buffers = 0u64;
        let mut cached = 0u64;

        for line in reader.lines() {
            let line = line.map_err(anyhow::Error::msg)?;
            if line.starts_with("MemTotal:") {
                mem_total = parse_meminfo_line(&line)?;
            }
            if line.starts_with("MemFree:") {
                mem_free = parse_meminfo_line(&line)?;
            }
            if line.starts_with("Buffers:") {
                buffers = parse_meminfo_line(&line)?;
            }
            if line.starts_with("Cached:") {
                cached = parse_meminfo_line(&line)?;
            }
        }

        if mem_total == 0 || mem_free == 0 {
            anyhow::bail!("Failed to read MemTotal and/or MemFree from /proc/meminfo");
        }

        let used = mem_total - mem_free - buffers - cached;
        let total = (used as f64 / mem_total as f64) * 100.0;

        let name = self.name.as_deref().unwrap_or(NAME);
        let icon = self.icon.as_deref().unwrap_or(ICON);

        Ok(Component {
            name,
            icon,
            value: format!("{:.0}%", total),
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

fn parse_meminfo_line(line: &str) -> anyhow::Result<u64> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        anyhow::bail!("Invalid meminfo line format: {}", line);
    }
    let value = parts[1].parse::<u64>().map_err(anyhow::Error::msg)?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let memory = Memory {
            time: Some(1000),
            name: Some(String::from("Custom RAM")),
            icon: Some(String::from(ICON)),
        };

        let component = memory.convert().unwrap();

        assert_eq!(component.name, "Custom RAM");
        assert_eq!(component.icon, ICON);
        assert!(component.value.ends_with("%"));
    }

    #[test]
    fn test_memory_convert_with_default_values() {
        let memory = Memory::default();

        let component = memory.convert().unwrap();

        assert_eq!(component.name, NAME);
        assert_eq!(component.icon, ICON);
        assert!(component.value.ends_with("%"));
    }
}
