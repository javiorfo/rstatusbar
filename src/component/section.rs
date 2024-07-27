use std::fmt::Display;

pub struct Component<'a> {
    pub icon: &'a str,
    pub name: &'a str,
    pub value: String,
}

impl Display for Component<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value.is_empty() {
            write!(f, " {} {} ", self.icon, self.name)
        } else {
            write!(f, " {} {} {} ", self.icon, self.name, self.value)
        }
    }
}
