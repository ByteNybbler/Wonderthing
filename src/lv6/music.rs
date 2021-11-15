use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Music(pub i32);

impl Music {
    pub const PSEUDO_SILENT: Self = Self(0);
    pub const RAINBOW: Self = Self(1);
    pub const BLOCKY: Self = Self(2);
    pub const BREEZY: Self = Self(3);
    pub const UNDERGROUND: Self = Self(4);
    pub const NOSTALGIA: Self = Self(5);
    pub const SPOOKY: Self = Self(6);
}