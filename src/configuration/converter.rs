use sysinfo::System;

use crate::component::section::Component;

pub trait Converter: Send {
    fn convert(&self, sys: &mut System) -> Component;
    fn get_time(&self) -> u64;
}
