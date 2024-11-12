use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    icons::{
        ICON_CONTROLLER_SNES, ICON_COUCH, ICON_HAMMER, ICON_PLANT, ICON_PRESENTATION,
        ICON_SAUCE_PAN, ICON_SILKSCREEN, ICON_SOLDERING_IRON, ICON_TELEVISION, ICON_TOILET_FRONT,
    },
    models::{Color, Language},
};

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialOrd, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Room {
    Lounge,
    Kitchen,
    Cave,
    Workshop,
    #[default]
    Hackcenter,
    Toilet,
    WetLab,
    ELab,
    SeminarRoom,
    Garden,
}

impl Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.id())
    }
}

impl Room {
    pub fn id(self) -> &'static str {
        match self {
            Room::Lounge => "lounge",
            Room::Kitchen => "kitchen",
            Room::Cave => "cave",
            Room::Workshop => "workshop",
            Room::Hackcenter => "hackcenter",
            Room::Toilet => "toilet",
            Room::WetLab => "wet-lab",
            Room::ELab => "e-lab",
            Room::SeminarRoom => "seminar-room",
            Room::Garden => "garden",
        }
    }

    pub fn all() -> &'static [Room] {
        &[
            Room::Lounge,
            Room::Kitchen,
            Room::Cave,
            Room::Workshop,
            Room::Hackcenter,
            Room::Toilet,
            Room::WetLab,
            Room::ELab,
            Room::SeminarRoom,
            Room::Garden,
        ]
    }

    pub fn color(self) -> Color {
        match self {
            Room::Lounge => Color::Yellow,
            Room::Kitchen => Color::Green,
            Room::Cave => Color::Red,
            Room::Workshop => Color::Purple,
            Room::Hackcenter => Color::Yellow,
            Room::Toilet => Color::Blue,
            Room::WetLab => Color::Purple,
            Room::ELab => Color::Purple,
            Room::SeminarRoom => Color::Yellow,
            Room::Garden => Color::Green,
        }
    }
    pub fn name(self, lang: Language) -> &'static str {
        match lang {
            Language::De => match self {
                Room::Lounge => "Lounge",
                Room::Kitchen => "Küche",
                Room::Cave => "Cave",
                Room::Workshop => "Werkstatt",
                Room::Hackcenter => "Hackcenter",
                Room::Toilet => "Toilette",
                Room::WetLab => "Wetlab",
                Room::ELab => "E-Lab",
                Room::SeminarRoom => "Seminarraum",
                Room::Garden => "Garten",
            },
            Language::En => match self {
                Room::Lounge => "Lounge",
                Room::Kitchen => "Kitchen",
                Room::Cave => "Cave",
                Room::Workshop => "Workshop",
                Room::Hackcenter => "Hackcenter",
                Room::Toilet => "Toilet",
                Room::WetLab => "Wetlab",
                Room::ELab => "E-Lab",
                Room::SeminarRoom => "Seminar Room",
                Room::Garden => "Garden",
            },
        }
    }

    pub fn icon(self) -> Option<String> {
        match self {
            Room::Lounge => Some(ICON_COUCH.into()),
            Room::Kitchen => Some(ICON_SAUCE_PAN.into()),
            Room::Cave => Some(ICON_CONTROLLER_SNES.into()),
            Room::Workshop => Some(ICON_HAMMER.into()),
            Room::Hackcenter => Some(ICON_TELEVISION.into()),
            Room::Toilet => Some(ICON_TOILET_FRONT.into()),
            Room::WetLab => Some(ICON_SILKSCREEN.into()),
            Room::ELab => Some(ICON_SOLDERING_IRON.into()),
            Room::SeminarRoom => Some(ICON_PRESENTATION.into()),
            Room::Garden => Some(ICON_PLANT.into()),
        }
    }
}

impl FromStr for Room {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lounge" => Ok(Room::Lounge),
            "kitchen" => Ok(Room::Kitchen),
            "cave" => Ok(Room::Cave),
            "workshop" => Ok(Room::Workshop),
            "hackcenter" => Ok(Room::Hackcenter),
            "toilet" => Ok(Room::Toilet),
            "wet-lab" => Ok(Room::WetLab),
            "e-lab" => Ok(Room::ELab),
            "seminar-room" => Ok(Room::SeminarRoom),
            "garden" => Ok(Room::Garden),
            _ => Err(()),
        }
    }
}
