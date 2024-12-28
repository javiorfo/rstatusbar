use std::process::Command;

use serde::Deserialize;
use sysinfo::System;

use crate::{component::section::Component, configuration::converter::Converter};

const NAME: &str = "SCR";
const ICON: &str = "󰯁 ";
const TIME: u64 = 1000;

#[derive(Deserialize, Debug)]
pub struct Script {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub path: String,
}

impl Converter for Script {
    fn convert(&self, _sys: &mut System) -> anyhow::Result<Component> {
        let name = self.name.as_deref().unwrap_or(NAME);
        let icon = self.icon.as_deref().unwrap_or(ICON);

        let output = Command::new("sh")
            .arg(&self.path)
            .output()
            .map_err(anyhow::Error::msg)?;

        let temp = if output.status.success() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            format!("Error {}", String::from_utf8_lossy(&output.stderr))
        };

        Ok(Component {
            name,
            icon,
            value: temp,
        })
    }

    fn time(&self) -> u64 {
        self.time.unwrap_or(TIME)
    }
}

impl Default for Script {
    fn default() -> Self {
        Self {
            time: Some(TIME),
            name: Some(String::from(NAME)),
            icon: Some(String::from(ICON)),
            path: String::from(""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{metadata, remove_file, set_permissions, File};
    use std::{io::Write, os::unix::fs::PermissionsExt};

    #[test]
    fn test_convert_success() {
        let script_path = "/tmp/script.sh";
        let mut file = File::create(script_path).unwrap();
        writeln!(file, "echo 'Hello, World!'").unwrap();
        let mut permissions = metadata(script_path).unwrap().permissions();
        permissions.set_mode(0o755);

        set_permissions(script_path, permissions).unwrap();
        std::mem::drop(file);

        let script = Script {
            time: Some(1000),
            name: Some(String::from("Test Script")),
            icon: Some(String::from(ICON)),
            path: String::from(script_path),
        };

        let mut sys = System::new_all();
        let component = script.convert(&mut sys).unwrap();

        assert_eq!(component.name, "Test Script");
        assert_eq!(component.icon, ICON);
        assert_eq!(component.value, "Hello, World!\n");

        remove_file(script_path).unwrap();
    }

    #[test]
    fn test_convert_failure() {
        let script = Script {
            time: Some(1000),
            name: Some(String::from("Nonexistent Script")),
            icon: Some(String::from("❌")),
            path: String::from("/nonexistent/path"),
        };

        let mut sys = System::new_all();
        let component = script.convert(&mut sys).unwrap();

        assert!(component.value.contains("Error"));
    }
}
