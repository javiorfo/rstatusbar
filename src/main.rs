use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use component::section::Component;
use sysinfo::{Components, System};
mod config;
mod component;


fn main() {
    let result1_cache = Arc::new(Mutex::new(String::new()));
    let result2_cache = Arc::new(Mutex::new(String::new()));
    let result3_cache = Arc::new(Mutex::new(String::new()));

    process(result1_cache.clone(), Duration::from_millis(500), cpu_usage);
    process(result2_cache.clone(), Duration::from_millis(500), memory_usage);
    process(result3_cache.clone(), Duration::from_millis(500), temperature);

    statusbar(vec![result1_cache, result2_cache, result3_cache]);
}

fn process<F>(cache: Arc<Mutex<String>>, duration: Duration, fun: F)
where
    F: Fn(&mut System) -> Component + Send + 'static,
{
    let mut sys = System::new_all();
    thread::spawn(move || loop {
        let result = format!("{}", fun(&mut sys));
        *cache.clone().lock().unwrap() = result;
        thread::sleep(duration);
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

fn temperature(_: &mut System) -> Component {
    let components = Components::new_with_refreshed_list();
    let total = components.iter().map(|c| c.temperature()).sum::<f32>();
    let total = total as usize / components.len();
    let total = format!("{}󰔄 ", total);
    Component {
        name: Some("TEMP".to_string()),
        icon: Some("󰏈 ".to_string()),
        value: total,
    }
}

fn cpu_usage(sys: &mut System) -> Component {
    sys.refresh_cpu_usage();
    let total = sys.global_cpu_info().cpu_usage() as usize;
    let total = format!("{}%", total);

    Component {
        name: Some("CPU".to_string()),
        icon: Some("󰏈 ".to_string()),
        value: total,
    }
}

fn memory_usage(sys: &mut System) -> Component {
    sys.refresh_memory();
    let memory_perc = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;
    let total = format!("{:.0}%", memory_perc);
    Component {
        name: Some("RAM".to_string()),
        icon: Some(" ".to_string()),
        value: total,
    }
}
