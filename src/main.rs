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

fn main() {
    let mut values: Vec<Arc<Mutex<String>>> = Vec::new();
    let (general, config) = get_configuration();

    for v in config {
        let cache = Arc::new(Mutex::new(String::new()));
        process(cache.clone(), v);
        values.push(cache);
    }

    statusbar(general, values);
}

fn process(cache: Arc<Mutex<String>>, converter: Box<dyn Converter>) {
    let mut sys = System::new_all();
    thread::spawn(move || loop {
        let result = format!("{}", converter.convert(&mut sys));
        *cache.clone().lock().unwrap() = result;
        thread::sleep(Duration::from_millis(converter.get_time()));
    });
}

fn statusbar(general: General, list: Vec<Arc<Mutex<String>>>) {
    loop {
        let mut xsetroot = String::new();
        for value in list.iter() {
            let result = value.lock().unwrap().clone();
            xsetroot.push_str(&result);
            xsetroot.push_str(&general.separator.clone().unwrap());
        }

        Command::new("xsetroot")
            .arg("-name")
            .arg(xsetroot)
            .spawn()
            .expect("Internal error executing xsetroot");

        thread::sleep(Duration::from_millis(100));
    }
}
