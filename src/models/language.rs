use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Copy, Clone, PartialEq)]
pub enum Language {
    #[default]
    En,
    De,
}
