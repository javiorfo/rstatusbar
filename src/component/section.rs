use std::fmt::Display;

pub struct Component<'a> {
    pub icon: &'a str,
    pub name: &'a str,
    pub value: String,
}

impl Display for Component<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon_str = get_final_string(self.icon);
        let name_str = get_final_string(self.name);

        if self.value.is_empty() {
            write!(f, " {icon_str}{name_str}")
        } else {
            write!(f, " {}{}{} ", icon_str, name_str, self.value)
        }
    }
}

fn get_final_string(string: &str) -> String {
    if string.is_empty() {
        String::from("")
    } else {
        let mut with_trailing_space = string.to_string();
        with_trailing_space.push(' ');
        with_trailing_space
    }
}
