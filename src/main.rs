use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use component::section::Component;
use sysinfo::Components;
mod config;
mod component;

fn main() {
    let result1_cache = Arc::new(Mutex::new(String::new()));
    let result2_cache = Arc::new(Mutex::new(String::new()));

    process(result1_cache.clone(), Duration::from_secs(1), || Component {
        name: Some("nada".to_string()),
        icon: None,
        value: "30".to_string(),
    });

    process(result2_cache.clone(), Duration::from_millis(500), || {
        temperature()
    });
    statusbar(vec![result1_cache, result2_cache]);
}

fn process<F>(cache: Arc<Mutex<String>>, duration: Duration, fun: F)
where
    F: Fn() -> Component + Send + 'static,
{
    thread::spawn(move || loop {
        let result = format!("{}", fun());
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

fn temperature() -> Component {
    let components = Components::new_with_refreshed_list();
    let total = components.iter().map(|c| c.temperature()).sum::<f32>();
    let total = total as usize / components.len();
    let total = format!("{}󰔄 ", total);
    Component {
        name: Some("TEMP".to_string()),
        icon: Some("󰏈 ".to_string()),
        value: total,
    }
    //     format!("󰏈  TEMP {}󰔄 ", &total.to_string())
}

