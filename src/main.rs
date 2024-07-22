use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use component::section::Component;
use configuration::values::get_configuration;
use sysinfo::{Components, System};
mod component;
mod configuration;

fn main() {
    let config = get_configuration();
    let mut values: Vec<Arc<Mutex<String>>> = Vec::new();

    if let Some(ref cpu) = config.cpu {
        let icon: &'static str =
            Box::leak(cpu.icon.clone().unwrap_or("".to_string()).into_boxed_str());
        let name: &'static str =
            Box::leak(cpu.name.clone().unwrap_or("".to_string()).into_boxed_str());
        let duration = cpu.time.unwrap_or(500);
        let cache = Arc::new(Mutex::new(String::new()));
        process(
            cache.clone(),
            Duration::from_millis(duration),
            cpu_perc_usage(Component {
                icon,
                name,
                value: "".to_string(),
            }),
        );
        values.push(cache);
    }

    if let Some(ref ram) = config.ram {
        let icon: &'static str =
            Box::leak(ram.icon.clone().unwrap_or("".to_string()).into_boxed_str());
        let name: &'static str =
            Box::leak(ram.name.clone().unwrap_or("".to_string()).into_boxed_str());
        let duration = ram.time.unwrap_or(500);
        let cache = Arc::new(Mutex::new(String::new()));
        process(
            cache.clone(),
            Duration::from_millis(duration),
            ram_perc_usage(Component {
                icon,
                name,
                value: "".to_string(),
            }),
        );
        values.push(cache);
    }

    if let Some(ref temp) = config.temperature {
        let icon: &'static str =
            Box::leak(temp.icon.clone().unwrap_or("".to_string()).into_boxed_str());
        let name: &'static str =
            Box::leak(temp.name.clone().unwrap_or("".to_string()).into_boxed_str());
        let duration = temp.time.unwrap_or(500);
        let cache = Arc::new(Mutex::new(String::new()));
        process(
            cache.clone(),
            Duration::from_millis(duration),
            temperature(Component {
                icon,
                name,
                value: "".to_string(),
            }),
        );
        values.push(cache);
    }
    
    if let Some(ref disk) = config.disk {
        let icon: &'static str =
            Box::leak(disk.icon.clone().unwrap_or("".to_string()).into_boxed_str());
        let name: &'static str =
            Box::leak(disk.name.clone().unwrap_or("".to_string()).into_boxed_str());
        let duration = disk.time.unwrap_or(500);
        let cache = Arc::new(Mutex::new(String::new()));
        process(
            cache.clone(),
            Duration::from_millis(duration),
            disk_perc_usage(Component {
                icon,
                name,
                value: "".to_string(),
            }),
        );
        values.push(cache);
    }

    statusbar(values);
}

fn process<F>(cache: Arc<Mutex<String>>, duration: Duration, f: F)
where
    F: Fn(&mut System) -> Component + Send + 'static,
{
    let mut sys = System::new_all();
    thread::spawn(move || loop {
        let result = format!("{}", f(&mut sys));
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

fn temperature(comp: Component<'static>) -> impl Fn(&mut System) -> Component {
    move |_| {
        let components = Components::new_with_refreshed_list();
        let total = components.iter().map(|c| c.temperature()).sum::<f32>();
        let total = total as usize / components.len();
        let total = format!("{}󰔄 ", total);
        Component {
            name: comp.name,
            icon: comp.icon,
            value: total,
        }
    }
}

fn ram_perc_usage(comp: Component<'static>) -> impl Fn(&mut System) -> Component {
    move |sys: &mut System| {
        sys.refresh_cpu_usage();
        let memory_perc = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;
        let total = format!("{:.0}%", memory_perc);
        Component {
            name: comp.name,
            icon: comp.icon,
            value: total,
        }
    }
}

fn cpu_perc_usage(comp: Component<'static>) -> impl Fn(&mut System) -> Component {
    move |sys: &mut System| {
        sys.refresh_cpu_usage();
        let total = sys.global_cpu_info().cpu_usage() as usize;
        let total = format!("{}%", total);
        Component {
            name: comp.name,
            icon: comp.icon,
            value: total,
        }
    }
}

fn disk_perc_usage(comp: Component<'static>) -> impl Fn(&mut System) -> Component {
    move |sys: &mut System| {
        sys.refresh_cpu_usage();
        let total = sys.global_cpu_info().cpu_usage() as usize;
        let total = format!("{}%", total);
        Component {
            name: comp.name,
            icon: comp.icon,
            value: total,
        }
    }
}

fn volume(comp: Component<'static>) -> impl Fn(&mut System) -> Component {
    move |_| {
        let components = Components::new_with_refreshed_list();
        let total = components.iter().map(|c| c.temperature()).sum::<f32>();
        let total = total as usize / components.len();
        let total = format!("{}󰔄 ", total);
        Component {
            name: comp.name,
            icon: comp.icon,
            value: total,
        }
    }
}

fn network(comp: Component<'static>) -> impl Fn(&mut System) -> Component {
    move |_| {
        let components = Components::new_with_refreshed_list();
        let total = components.iter().map(|c| c.temperature()).sum::<f32>();
        let total = total as usize / components.len();
        let total = format!("{}󰔄 ", total);
        Component {
            name: comp.name,
            icon: comp.icon,
            value: total,
        }
    }
}

fn date(comp: Component<'static>) -> impl Fn(&mut System) -> Component {
    move |_| {
        let components = Components::new_with_refreshed_list();
        let total = components.iter().map(|c| c.temperature()).sum::<f32>();
        let total = total as usize / components.len();
        let total = format!("{}󰔄 ", total);
        Component {
            name: comp.name,
            icon: comp.icon,
            value: total,
        }
    }
}


fn battery_perc_usage(comp: Component<'static>) -> impl Fn(&mut System) -> Component {
    move |_| {
        let components = Components::new_with_refreshed_list();
        let total = components.iter().map(|c| c.temperature()).sum::<f32>();
        let total = total as usize / components.len();
        let total = format!("{}󰔄 ", total);
        Component {
            name: comp.name,
            icon: comp.icon,
            value: total,
        }
    }
}

