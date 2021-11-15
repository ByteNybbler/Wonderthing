// Using newtype structs instead of enums so that the program doesn't choke when deserializing an unknown value.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Style(pub i32);

impl Style {
    pub const CAVE: Self = Self(0);
    pub const SAND: Self = Self(1);
    pub const WOOD: Self = Self(2);
    pub const PURPLE: Self = Self(3);
    pub const CASTLE: Self = Self(4);
    pub const JADE: Self = Self(5);
    pub const SPOOKY: Self = Self(6);
    pub const GARDEN: Self = Self(7);
    pub const AZTEC: Self = Self(8);
    pub const CUSTOM: Self = Self(9);
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Background(i32);

impl Background {
    pub const SKY: Self = Self(0);
    pub const FOREST: Self = Self(1);
    pub const WALLS: Self = Self(2);
    pub const STARS: Self = Self(3);
    pub const FLAT: Self = Self(4);
    pub const WATER: Self = Self(5);
    pub const LAVA: Self = Self(6);
    pub const WARP: Self = Self(7);
    pub const CITY: Self = Self(8);
    pub const RAINBOW: Self = Self(9);
    pub const CUSTOM: Self = Self(10);
    pub const PITCH_BLACK: Self = Self(12);
}

#[derive(Serialize, Deserialize)]
pub struct LevelAppearance {
    style: Style,
    background: Background,
}

impl LevelAppearance {
    pub fn new(style: Style, background: Background) -> LevelAppearance {
        LevelAppearance {
            style,
            background
        }
    }

    pub fn get_style(&self) -> &Style {
        &self.style
    }

    pub fn get_background(&self) -> &Background {
        &self.background
    }

    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn set_background(&mut self, background: Background) {
        self.background = background;
    }
}