use serde::Deserialize;
use std::fs;
use toml::from_str;

use super::components::general::General;
use super::components::network::Network;
use super::components::volume::Volume;
use super::components::{cpu::Cpu, disk::Disk, memory::Memory, temperature::Temperature};
use super::converter::Converter;

#[derive(Deserialize, Debug)]
struct Config {
    pub general: Option<General>,
    pub cpu: Option<Cpu>,
    pub memory: Option<Memory>,
    pub disk: Option<Disk>,
    pub temperature: Option<Temperature>,
    pub volume: Option<Volume>,
    pub network: Option<Network>,
}

pub fn get_configuration() -> (General, Vec<Box<dyn Converter>>) {
    let home_path = std::env::var_os("HOME").expect("No HOME variable set.");

    let config_path = format!(
        "{}{}",
        home_path.to_string_lossy(),
        "/.config/rustatusbar/config.toml"
    );

    if let Ok(toml) = fs::read_to_string(config_path) {
        let config: Config = from_str(&toml).expect("Error parsing TOML");
        let mut values: Vec<Box<dyn Converter>> = Vec::new();

        if config.cpu.is_some() {
            values.push(Box::new(config.cpu.unwrap()));
        }
        if config.memory.is_some() {
            values.push(Box::new(config.memory.unwrap()));
        }
        if config.disk.is_some() {
            values.push(Box::new(config.disk.unwrap()));
        }
        if config.temperature.is_some() {
            values.push(Box::new(config.temperature.unwrap()));
        }
        if config.volume.is_some() {
            values.push(Box::new(config.volume.unwrap()));
        }
        if config.network.is_some() {
            values.push(Box::new(config.network.unwrap()));
        }

        (config.general.unwrap_or_default(), values)
    } else {
        (
            General::default(),
            vec![
                Box::new(Cpu::default()),
                Box::new(Memory::default()),
                Box::new(Temperature::default()),
                Box::new(Disk::default()),
                Box::new(Volume::default()),
                Box::new(Network::default()),
            ]
        )
    }
}
