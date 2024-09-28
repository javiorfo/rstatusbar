use serde::Deserialize;
use std::fs;
use toml::from_str;

use super::components::battery::Battery;
use super::components::date::Date;
use super::components::general::General;
use super::components::network::Network;
use super::components::script::Script;
use super::components::volume::Volume;
use super::components::weather::Weather;
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
    pub battery: Option<Battery>,
    pub script: Option<Script>,
    pub weather: Option<Weather>,
    pub date: Option<Date>,
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
        if config.temperature.is_some() {
            values.push(Box::new(config.temperature.unwrap()));
        }
        if config.disk.is_some() {
            values.push(Box::new(config.disk.unwrap()));
        }
        if config.volume.is_some() {
            values.push(Box::new(config.volume.unwrap()));
        }
        if config.network.is_some() {
            values.push(Box::new(config.network.unwrap()));
        }
        if config.battery.is_some() {
            values.push(Box::new(config.battery.unwrap()));
        }
        if config.script.is_some() {
            values.push(Box::new(config.script.unwrap()));
        }
        if config.weather.is_some() {
            values.push(Box::new(config.weather.unwrap()));
        }
        if config.date.is_some() {
            values.push(Box::new(config.date.unwrap()));
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
                Box::new(Date::default()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_get_configuration_with_valid_toml() {
        let config_content = r#"
            [general]
            separator = "|"

            [cpu]
            time = 1000
            name = "CPU"
            icon = " "

            [memory]
            time = 1000
            name = "RAM"
            icon = " "

            [temperature]
            time = 1000
            name = "TEMP"
            icon = "󰏈 " 

            [disk]
            time = 2000
            name = "DISK"
            icon = "󰋊 "
            unit = "/"

            [volume]
            time = 100
            name = "VOL"
            icon_active = " " 
            icon_muted = "󰖁 " 

            [network]
            time = 5000
            name = "NET"
            icon_up = "󰀂 " 
            icon_down = "󰯡 " 
            
            [weather]
            name = "WEA"
            location = "Buenos+Aires"

            [date]
            time = 1000
            format = "%A %d/%m/%Y %H:%M"
            icon = " "

        "#;

        let dir = tempdir().unwrap();
        let home_path = dir.path().to_str().unwrap();
        env::set_var("HOME", home_path);

        let config_dir = format!("{}/.config/rustatusbar", home_path);
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = format!("{}/config.toml", config_dir);
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "{}", config_content).unwrap();

        let (general, converters) = get_configuration();

        assert!(general.separator.is_some());

        assert_eq!(converters.len(), 8);
    }

    #[test]
    fn test_get_configuration_with_missing_toml() {
        let dir = tempdir().unwrap();
        let home_path = dir.path().to_str().unwrap();
        env::set_var("HOME", home_path);
        let (general, converters) = get_configuration();
        assert!(general.separator.is_some());
        assert_eq!(converters.len(), 7);
    }

    #[test]
    fn test_get_configuration_with_invalid_toml() {
        let dir = tempdir().unwrap();
        let home_path = dir.path().to_str().unwrap();
        env::set_var("HOME", home_path);

        let config_dir = format!("{}/.config/rustatusbar", home_path);
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = format!("{}/config.toml", config_dir);

        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "invalid_toml").unwrap();

        let result = std::panic::catch_unwind(|| {
            get_configuration();
        });

        assert!(result.is_err());
    }
}
