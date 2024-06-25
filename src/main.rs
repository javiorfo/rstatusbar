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

    process(result1_cache.clone(), Duration::from_secs(200), cpu_usage);
    process(result2_cache.clone(), Duration::from_millis(500), temperature);

    statusbar(vec![result1_cache, result2_cache]);
}

fn process<F>(cache: Arc<Mutex<String>>, duration: Duration, fun: F)
where
    F: Fn(&mut System) -> Component + Send + 'static,
{
    let mut sys = System::new();
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

fn temperature(_sys: &mut System) -> Component {
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

fn cpu_usage(sys: &mut System) -> Component {
    sys.refresh_cpu_usage();
//     let l = sys.cpus().len();
//     let total = sys.cpus().len();
//     let total = sys.cpus().iter().map(|v| v.cpu_usage()).sum::<f32>() as usize / l;
//     let total = sys.cpus().iter().map(|v| v.cpu_usage()).sum::<f32>();
    let total = sys.global_cpu_info().cpu_usage();
    let total = format!("{}%", total);
    Component {
        name: Some("CPU".to_string()),
        icon: Some("󰏈 ".to_string()),
        value: total,
    }
    //     format!("󰏈  TEMP {}󰔄 ", &total.to_string())
}
