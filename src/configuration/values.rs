use serde::Deserialize;
use std::fs;
use toml::from_str;

use super::components::{cpu::Cpu, disk::Disk, memory::Memory, temperature::Temperature};
use super::converter::Converter;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub cpu: Option<Cpu>,
    pub ram: Option<Memory>,
    pub disk: Option<Disk>,
    pub temperature: Option<Temperature>,
}

fn get_configuration() -> Config {
    let toml = fs::read_to_string("~/.config/rustatusbar/config.toml")
        .map_or_else(|_| fs::read_to_string("config.toml"), Ok)
        .expect("Error reading file");

    let config: Config = from_str(&toml).expect("Error parsing TOML");
    config
}

pub fn obtain() -> Vec<Box<dyn Converter>> {
    let config = get_configuration();
    vec![Box::new(config.disk.unwrap_or(Disk {
        time: Some(500),
        name: Some("DISK".to_string()),
        icon: Some("".to_string()),
        unit: Some("/".to_string()),
    })),
    Box::new(config.cpu.unwrap_or(Cpu {
        time: Some(500),
        name: Some("CPU".to_string()),
        icon: Some("".to_string()),
    })),
    Box::new(config.temperature.unwrap_or(Temperature {
        time: Some(500),
        name: Some("TEMP".to_string()),
        icon: Some("".to_string()),
    })),
    Box::new(config.ram.unwrap_or(Memory {
        time: Some(500),
        name: Some("RAM".to_string()),
        icon: Some("".to_string()),
    })),
    ]
}

#[cfg(test)]
mod tests {
    use super::get_configuration;

    #[test]
    fn config_test() {
        let config = get_configuration();
        println!("{:?}", config);
    }
}
