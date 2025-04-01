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
use super::device::{Converter, Device};

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

        add_device(config.cpu, &mut values, Device::Cpu);
        add_device(config.memory, &mut values, Device::Memory);
        add_device(config.temperature, &mut values, Device::Temperature);
        add_device(config.disk, &mut values, Device::Disk);
        add_device(config.volume, &mut values, Device::Volume);
        add_device(config.network, &mut values, Device::Network);
        add_device(config.battery, &mut values, Device::Battery);
        add_device(config.script, &mut values, Device::Script);
        add_device(config.weather, &mut values, Device::Weather);
        add_device(config.date, &mut values, Device::Date);

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

fn add_device<T, F>(h: Option<T>, v: &mut Vec<Device>, f: F)
where
    T: Converter,
    F: FnOnce(T) -> Device,
{
    if let Some(value) = h {
        v.push(f(value));
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
