use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct General {
    pub separator: Option<String>,
}

impl Default for General {
    fn default() -> Self {
        Self {
            separator: Some(String::from("|")),
        }
    }
}
