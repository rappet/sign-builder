use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialOrd, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl Color {
    pub fn accent_class(self) -> &'static str {
        match self {
            Color::Red => "accent-red",
            Color::Yellow => "accent-yellow",
            Color::Green => "accent-green",
            Color::Blue => "accent-blue",
            Color::Purple => "accent-purple",
        }
    }
}
