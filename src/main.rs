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
        if let Ok(comp) = converter.convert(&mut sys) {
            let result = format!("{}", comp);
            *cache.clone().lock().unwrap() = result;
            thread::sleep(Duration::from_millis(converter.get_time()));
        } else {
            continue;
        }
    });
}

fn create_statusbar(general: General, list: Vec<Arc<Mutex<String>>>) {
    let separator = &general.separator.clone().unwrap();
    loop {
        let mut xsetroot = String::new();
        for value in list.iter() {
            let result = value.lock().unwrap().clone();
            if !result.is_empty() {
                xsetroot.push_str(&result);
                xsetroot.push_str(separator);
            }
        }
        xsetroot.pop();

        Command::new("xsetroot")
            .arg("-name")
            .arg(xsetroot)
            .spawn()
            .expect("Internal error executing xsetroot");

        thread::sleep(Duration::from_millis(TIME));
    }
}
