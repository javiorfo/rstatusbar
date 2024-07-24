use alsa::{
    mixer::{SelemChannelId, SelemId},
    Mixer,
};
use serde::Deserialize;
use sysinfo::System;

use crate::{component::section::Component, configuration::converter::Converter};

const NAME: &str = "VOL";
const ICON_ACTIVE: &str = " ";
const ICON_MUTED: &str = "󰖁 ";
const TIME: u64 = 100;


#[derive(Deserialize, Debug)]
pub struct Volume {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon_active: Option<String>,
    pub icon_muted: Option<String>,
}

impl Converter for Volume {
    fn convert(&self, _sys: &mut System) -> Component {
        let mixer = Mixer::new("default", false).unwrap();

        let selem_id = SelemId::new("Master", 0);
        let elem = mixer.find_selem(&selem_id).unwrap();

        let icon = if elem.get_playback_switch(SelemChannelId::FrontLeft).unwrap() == 0 {
            self.icon_muted.as_deref().unwrap_or(ICON_MUTED)
        } else {
            self.icon_active.as_deref().unwrap_or(ICON_ACTIVE)
        };

        let volume_range = elem.get_playback_volume_range();
        let volume = elem.get_playback_volume(SelemChannelId::FrontLeft).unwrap();

        let volume_percentage =
            ((volume - volume_range.0) as f64 / (volume_range.1 - volume_range.0) as f64) * 100.0;

        let name = self.name.as_deref().unwrap_or(NAME);
        let total = format!("{:.0}%", volume_percentage);

        Component {
            name,
            icon,
            value: total,
        }
    }

    fn get_time(&self) -> u64 {
        self.time.unwrap_or(TIME)
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self {
            time: Some(TIME),
            name: Some(String::from(NAME)),
            icon_active: Some(String::from(ICON_ACTIVE)),
            icon_muted: Some(String::from(ICON_MUTED)),
        }
    }
}
