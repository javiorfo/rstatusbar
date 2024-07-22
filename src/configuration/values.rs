use serde::Deserialize;
use std::fs;
use toml::from_str;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub cpu: Option<Cpu>,
    pub ram: Option<Ram>,
    pub disk: Option<Disk>,
    pub temperature: Option<Temperature>,
}

#[derive(Deserialize, Debug)]
pub struct Cpu {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Temperature {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Ram {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Disk {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
}

pub fn get_configuration() -> Config {
    let toml = fs::read_to_string("~/.config/rustatusbar/config.toml")
        .map_or_else(|_| fs::read_to_string("config.toml"), Ok)
        .expect("Error reading file");

    let config: Config = from_str(&toml).expect("Error parsing TOML");
    config
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
