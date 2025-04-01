use crate::component::section::Component;

use super::components::{
    battery::Battery, cpu::Cpu, date::Date, disk::Disk, memory::Memory, network::Network,
    script::Script, temperature::Temperature, volume::Volume, weather::Weather,
};

pub enum Device {
    Battery(Battery),
    Cpu(Cpu),
    Date(Date),
    Disk(Disk),
    Memory(Memory),
    Network(Network),
    Script(Script),
    Temperature(Temperature),
    Volume(Volume),
    Weather(Weather),
}

impl Device {
    pub fn apply(&self) -> (anyhow::Result<Component>, u64) {
        match self {
            Device::Battery(d) => (d.convert(), d.time()),
            Device::Cpu(d) => (d.convert(), d.time()),
            Device::Date(d) => (d.convert(), d.time()),
            Device::Disk(d) => (d.convert(), d.time()),
            Device::Memory(d) => (d.convert(), d.time()),
            Device::Network(d) => (d.convert(), d.time()),
            Device::Script(d) => (d.convert(), d.time()),
            Device::Temperature(d) => (d.convert(), d.time()),
            Device::Volume(d) => (d.convert(), d.time()),
            Device::Weather(d) => (d.convert(), d.time()),
        }
    }
}

pub trait Converter: Send {
    fn convert(&self) -> anyhow::Result<Component>;
    fn time(&self) -> u64;
}
