use serde::Deserialize;
use sysinfo::System;

#[derive(Deserialize, Debug)]
pub struct Cpu {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
}
trait Compo{}
trait Papa {
    fn execute(&self) -> impl Fn(&mut System) -> Box<dyn Compo>;
}

impl Compo for Cpu {}

impl Papa for Cpu {
    fn execute(&self) -> impl Fn(&mut System) -> Box<dyn Compo> {
    move |sys: &mut System| {
        sys.refresh_cpu_usage();
//         let total = sys.global_cpu_info().cpu_usage() as usize;
//         let total = format!("{}%", total);
        Box::new(Cpu {
            name: self.name.clone(),
            icon: self.icon.clone(),
            time: None
//             value: total,
        })
    }
    }
}
