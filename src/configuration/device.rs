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
    pub fn get(&self) -> anyhow::Result<Component<'_>> {
        match self {
            Device::Battery(d) => d.convert(),
            Device::Cpu(d) => d.convert(),
            Device::Date(d) => d.convert(),
            Device::Disk(d) => d.convert(),
            Device::Memory(d) => d.convert(),
            Device::Network(d) => d.convert(),
            Device::Script(d) => d.convert(),
            Device::Temperature(d) => d.convert(),
            Device::Volume(d) => d.convert(),
            Device::Weather(d) => d.convert(),
        }
    }

    pub fn time(&self) -> u64 {
        match self {
            Device::Battery(d) => d.time(),
            Device::Cpu(d) => d.time(),
            Device::Date(d) => d.time(),
            Device::Disk(d) => d.time(),
            Device::Memory(d) => d.time(),
            Device::Network(d) => d.time(),
            Device::Script(d) => d.time(),
            Device::Temperature(d) => d.time(),
            Device::Volume(d) => d.time(),
            Device::Weather(d) => d.time(),
        }
    }
}

pub trait Converter: Send {
    fn convert(&self) -> anyhow::Result<Component<'_>>;
    fn time(&self) -> u64;
}
