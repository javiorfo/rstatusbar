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
use super::converter::Device;

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

pub fn get_configuration() -> (General, Vec<Device>) {
    let home_path = std::env::var_os("HOME").expect("No HOME variable set.");

    let config_path = format!(
        "{}{}",
        home_path.to_string_lossy(),
        "/.config/rstatusbar/config.toml"
    );

    if let Ok(toml) = fs::read_to_string(config_path) {
        let config: Config = from_str(&toml).expect("Error parsing TOML");
        let mut values: Vec<Device> = Vec::new();

        if let Some(value) = config.cpu {
            values.push(Device::Cpu(value));
        }
        if let Some(value) = config.memory {
            values.push(Device::Memory(value));
        }
        if let Some(value) = config.temperature {
            values.push(Device::Temperature(value));
        }
        if let Some(value) = config.disk {
            values.push(Device::Disk(value));
        }
        if let Some(value) = config.volume {
            values.push(Device::Volume(value));
        }
        if let Some(value) = config.network {
            values.push(Device::Network(value));
        }
        if let Some(value) = config.battery {
            values.push(Device::Battery(value));
        }
        if let Some(value) = config.script {
            values.push(Device::Script(value));
        }
        if let Some(value) = config.weather {
            values.push(Device::Weather(value));
        }
        if let Some(value) = config.date {
            values.push(Device::Date(value));
        }

        (config.general.unwrap_or_default(), values)
    } else {
        (
            General::default(),
            vec![
                Device::Cpu(Cpu::default()),
                Device::Memory(Memory::default()),
                Device::Temperature(Temperature::default()),
                Device::Disk(Disk::default()),
                Device::Volume(Volume::default()),
                Device::Network(Network::default()),
                Device::Date(Date::default()),
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

        let config_dir = format!("{}/.config/rstatusbar", home_path);
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

        let config_dir = format!("{}/.config/rstatusbar", home_path);
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
