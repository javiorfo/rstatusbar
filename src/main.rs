use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use configuration::components::general::General;
use configuration::converter::Converter;
use configuration::values::get_configuration;
use sysinfo::System;
mod component;
mod configuration;

const TIME: u64 = 100;

fn main() {
    let mut values: Vec<Arc<Mutex<String>>> = Vec::new();
    let (general, config) = get_configuration();

    for v in config {
        let cache = Arc::new(Mutex::new(String::new()));
        create_component(cache.clone(), v);
        values.push(cache);
    }

    create_statusbar(general, values);
}

fn create_component(cache: Arc<Mutex<String>>, converter: Box<dyn Converter>) {
    let mut sys = System::new_all();
    thread::spawn(move || loop {
        match converter.convert(&mut sys) {
            Ok(comp) => {
                let result = format!("{}", comp);
                match cache.clone().lock() {
                    Ok(mut lock) => {
                        *lock = result;
                    }
                    Err(e) => {
                        eprintln!("Component lock error: {}", e);
                        continue;
                    }
                }

                thread::sleep(Duration::from_millis(converter.get_time()));
            }
            Err(e) => {
                eprintln!("Converter error: {}", e);
                continue;
            }
        }
    });
}

fn create_statusbar(general: General, list: Vec<Arc<Mutex<String>>>) {
    let separator = &general.separator.clone().unwrap();
    loop {
        let mut xsetroot = String::new();
        for value in list.iter() {
            match value.lock() {
                Ok(lock) => {
                    let result = lock;
                    if !result.is_empty() {
                        xsetroot.push_str(&result);
                        xsetroot.push_str(separator);
                    }
                }
                Err(e) => {
                    eprintln!("Statusbar lock error: {}", e);
                    continue;
                }
            }
        }
        if !xsetroot.is_empty() {
            if xsetroot.ends_with(separator) {
                xsetroot.pop();
            }
            match Command::new("xsetroot").arg("-name").arg(&xsetroot).output() {
                Ok(output) => {
                    if output.status.success() {
                        thread::sleep(Duration::from_millis(TIME));
                    } else {
                        eprintln!("Xsetroot error output status: {} {}", output.status, xsetroot);
                    }
                }
                Err(e) => {
                    eprintln!("Xsetroot error: {}, string: {}", e, xsetroot);
                    break;
                }
            }
        }
    }
}
