use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::configuration::components::general::General;
use crate::configuration::device::Device;
use crate::configuration::values::get_configuration;

const TIME: u64 = 100;

pub fn execute() {
    let mut values: Vec<Arc<Mutex<String>>> = Vec::new();
    let (general, devices) = get_configuration();

    let mut grouped_devices_by_time = HashMap::<u64, Vec<(Arc<Mutex<String>>, Device)>>::new();

    for dev in devices {
        let cache = Arc::new(Mutex::new(String::new()));
        grouped_devices_by_time
            .entry(dev.time())
            .or_default()
            .push((cache.clone(), dev));

        values.push(cache);
    }

    for (k, v) in grouped_devices_by_time {
        create_grouped_component(v, k);
    }

    create_statusbar(general, values);
}

fn create_grouped_component(devices: Vec<(Arc<Mutex<String>>, Device)>, time: u64) {
    thread::spawn(move || {
        loop {
            for (cache, device) in &devices {
                let component = device.get();
                match component {
                    Ok(comp) => {
                        if let Ok(mut lock) = cache.lock() {
                            *lock = comp.to_string();
                        } else {
                            eprintln!("Grouped component lock error");
                        }
                    }
                    Err(e) => {
                        eprintln!("Grouped converter error: {e}");
                    }
                }
            }
            thread::sleep(Duration::from_millis(time));
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
                    eprintln!("Statusbar lock error: {e}");
                    continue;
                }
            }
        }
        if !xsetroot.is_empty() {
            if xsetroot.ends_with(separator) {
                xsetroot.pop();
            }
            match Command::new("xsetroot")
                .arg("-name")
                .arg(&xsetroot)
                .output()
            {
                Ok(output) => {
                    if output.status.success() {
                        thread::sleep(Duration::from_millis(TIME));
                    } else {
                        eprintln!(
                            "Xsetroot error output status: {} {}",
                            output.status, xsetroot
                        );
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Xsetroot error: {e}, string: {xsetroot}");
                    break;
                }
            }
        }
    }
}
