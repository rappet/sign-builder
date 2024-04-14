use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::models::room::Room;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialOrd, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct Sign {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<Ulid>,
    pub room: Room,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub url: String,
}
