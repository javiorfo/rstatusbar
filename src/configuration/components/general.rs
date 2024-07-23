use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct General {
    pub separator: Option<String>,
}
