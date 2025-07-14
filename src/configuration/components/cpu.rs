use std::{
    fs::File,
    io::{BufRead, BufReader},
    thread::sleep,
    time::Duration,
};

use serde::Deserialize;

use crate::{component::section::Component, configuration::device::Converter};

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
    fn convert(&self) -> anyhow::Result<Component> {
        let total = match calculate_cpu_usage() {
            Some(usage) => format!("{usage:.0}%"),
            None => "-".to_string(),
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

impl Default for Cpu {
    fn default() -> Self {
        Self {
            time: Some(TIME),
            name: Some(String::from(NAME)),
            icon: Some(String::from(ICON)),
        }
    }
}

fn read_cpu_times() -> Option<(u64, u64)> {
    let file = File::open("/proc/stat").ok()?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    if !line.starts_with("cpu ") {
        return None;
    }
    let parts: Vec<&str> = line.split_whitespace().collect();
    // parts[0] = "cpu"
    let times: Vec<u64> = parts[1..]
        .iter()
        .filter_map(|v| v.parse::<u64>().ok())
        .collect();

    if times.len() < 4 {
        return None;
    }

    let idle = times[3]; // idle time is the 4th field
    let total: u64 = times.iter().sum();

    Some((idle, total))
}

fn calculate_cpu_usage() -> Option<f64> {
    let (idle1, total1) = read_cpu_times()?;
    sleep(Duration::from_millis(100));
    let (idle2, total2) = read_cpu_times()?;

    let idle_delta = idle2.saturating_sub(idle1);
    let total_delta = total2.saturating_sub(total1);

    if total_delta == 0 {
        return None;
    }

    let usage = 100.0 * (total_delta - idle_delta) as f64 / total_delta as f64;
    Some(usage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_get_time() {
        let cpu = Cpu {
            time: Some(2000),
            name: None,
            icon: None,
        };
        assert_eq!(cpu.time(), 2000);

        let cpu_default = Cpu::default();
        assert_eq!(cpu_default.time(), TIME);
    }

    #[test]
    fn test_cpu_convert() {
        let cpu = Cpu {
            time: Some(1000),
            name: Some(String::from("Custom CPU")),
            icon: Some(String::from(ICON)),
        };

        let component = cpu.convert().unwrap();

        assert_eq!(component.name, "Custom CPU");
        assert_eq!(component.icon, ICON);
        assert!(component.value.ends_with("%"));
    }

    #[test]
    fn test_cpu_convert_with_default_values() {
        let cpu = Cpu::default();

        let component = cpu.convert().unwrap();

        assert_eq!(component.name, NAME);
        assert_eq!(component.icon, ICON);
        assert!(component.value.ends_with("%"));
    }
}
