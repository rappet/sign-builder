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
    pub fn hex(self) -> &'static str {
        match self {
            Color::Red => "#fd8c7b",
            Color::Yellow => "#d0ae32",
            Color::Green => "#7fc76f",
            Color::Blue => "#30c0f8",
            Color::Purple => "#bf9bfc",
        }
    }

    pub fn text_class(self) -> &'static str {
        match self {
            Color::Red => "text-red",
            Color::Yellow => "text-yellow",
            Color::Green => "text-green",
            Color::Blue => "text-blue",
            Color::Purple => "text-purple",
        }
    }

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
