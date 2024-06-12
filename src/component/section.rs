use std::fmt::Display;

pub struct Component {
    pub icon: Option<String>,
    pub name: Option<String>,
    pub value: String,
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon = self.icon.as_deref().unwrap_or("");
        let name = self.name.as_deref().unwrap_or("");
        write!(f, " {} {} {} ", icon, name, self.value)
    }
}
