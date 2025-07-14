use alsa::{
    Mixer,
    mixer::{SelemChannelId, SelemId},
};
use serde::Deserialize;

use crate::{component::section::Component, configuration::device::Converter};

const NAME: &str = "VOL";
const ICON_ACTIVE: &str = " ";
const ICON_MUTED: &str = "󰖁 ";
const TIME: u64 = 100;
const MUTED: &str = "MUTED";

#[derive(Deserialize, Debug)]
pub struct Volume {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon_active: Option<String>,
    pub icon_muted: Option<String>,
}

impl Converter for Volume {
    fn convert(&self) -> anyhow::Result<Component> {
        let mixer = Mixer::new("default", false).map_err(anyhow::Error::msg)?;

        let selem_id = SelemId::new("Master", 0);
        let elem = mixer.find_selem(&selem_id).unwrap();
        let is_muted = elem
            .get_playback_switch(SelemChannelId::FrontLeft)
            .map_err(anyhow::Error::msg)?
            == 0;

        let icon = if is_muted {
            self.icon_muted.as_deref().unwrap_or(ICON_MUTED)
        } else {
            self.icon_active.as_deref().unwrap_or(ICON_ACTIVE)
        };

        let total = if !is_muted {
            let volume_range = elem.get_playback_volume_range();
            let volume = elem
                .get_playback_volume(SelemChannelId::FrontLeft)
                .map_err(anyhow::Error::msg)?;

            let volume_percentage = ((volume - volume_range.0) as f64
                / (volume_range.1 - volume_range.0) as f64)
                * 100.0;

            format!("{volume_percentage:.0}%")
        } else {
            String::from(MUTED)
        };
        let name = self.name.as_deref().unwrap_or(NAME);

        Ok(Component {
            name,
            icon,
            value: total,
        })
    }

    fn time(&self) -> u64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_get_time() {
        let volume = Volume {
            time: Some(200),
            name: None,
            icon_active: None,
            icon_muted: None,
        };
        assert_eq!(volume.time(), 200);

        let volume_default = Volume::default();
        assert_eq!(volume_default.time(), TIME);
    }

    #[test]
    fn test_volume_convert() {
        let volume = Volume::default();
        let component = volume.convert().unwrap();

        assert_eq!(component.name, NAME);
        assert!(component.icon == ICON_ACTIVE || component.icon == ICON_MUTED);
        assert!(component.value.ends_with("%") || component.value == MUTED);
    }
}
