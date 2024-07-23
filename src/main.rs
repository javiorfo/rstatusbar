use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use configuration::converter::Converter;
use configuration::values::obtain;
use sysinfo::System;
mod component;
mod configuration;

fn main() {
    let mut values: Vec<Arc<Mutex<String>>> = Vec::new();

    for v in obtain() {
        let cache = Arc::new(Mutex::new(String::new()));
        process(cache.clone(), v);
        values.push(cache);
    }

    statusbar(values);
}

fn process(cache: Arc<Mutex<String>>, converter: Box<dyn Converter>) {
    let mut sys = System::new_all();
    thread::spawn(move || loop {
        let result = format!("{}", converter.convert(&mut sys));
        *cache.clone().lock().unwrap() = result;
        thread::sleep(Duration::from_millis(converter.get_time()));
    });
}

fn statusbar(list: Vec<Arc<Mutex<String>>>) {
    loop {
        let mut bar = String::new();
        for value in list.iter() {
            let result = value.lock().unwrap().clone();
            bar.push_str(&result);
        }
        // TODO replace with xsetroot
        println!("{}", bar);

        thread::sleep(Duration::from_millis(100));
    }
}
