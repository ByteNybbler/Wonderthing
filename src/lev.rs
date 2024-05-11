mod name_data;
pub use name_data::*;
mod signs;
pub use signs::*;

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::path::Path;
use crate::lv6::{Lv6Error, Lv6, TimeLimit, LevelAppearance, LevelContent, Music, Style, Background, CustomContent};
use crate::serde_blitz3d;

#[derive(Serialize, Deserialize)]
pub struct Lev {
    name_data: NameData,
    time_limit: TimeLimit,
    level_appearance: LevelAppearance,
    level_content: LevelContent,
    signs: Signs
    // In TOW, music is determined by what world you're playing in rather than being an aspect of the level file itself.
}

impl Lev {
    pub fn new(name_data: NameData, time_limit: TimeLimit, level_appearance: LevelAppearance,
        level_content: LevelContent, signs: Signs) -> Lev {
        Lev {
            name_data,
            time_limit,
            level_appearance,
            level_content,
            signs
        }
    }

    pub fn to_file(&self) -> Result<(), Lv6Error> {
        let file = File::create(self.name_data.get_filename_with_extension()).map_err(Lv6Error::InputOutput)?;
        serde_blitz3d::to_writer(file, self).map_err(Lv6Error::Serde)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Lev, Lv6Error> {
        let file = File::open(path).map_err(Lv6Error::InputOutput)?;
        serde_blitz3d::from_reader(file).map_err(Lv6Error::Serde)
    }

    pub fn set_names(&mut self, filename: String, level_name: String) {
        self.name_data.set_filename(filename);
        self.name_data.set_level_name(level_name);
    }

    pub fn set_time_limit(&mut self, time_limit: TimeLimit) {
        self.time_limit = time_limit;
    }

    pub fn set_level_appearance(&mut self, level_appearance: LevelAppearance) {
        self.level_appearance = level_appearance;
    }

    pub fn set_style(&mut self, style: Style) {
        self.level_appearance.set_style(style);
    }

    pub fn set_background(&mut self, background: Background) {
        self.level_appearance.set_background(background);
    }

    pub fn borrow_level_content_mut(&mut self) -> &mut LevelContent {
        &mut self.level_content
    }
}

impl From<Lev> for Lv6 {
    fn from(lev: Lev) -> Lv6 {
        let mut level_content = lev.level_content;
        for object in level_content.borrow_objects_mut() {
            object.from_lev_to_lv6();
        }
        for tile in level_content.borrow_tiles_mut() {
            tile.from_lev_to_lv6();
        }
        // Make a best guess about the music based on the level style.
        let music = match lev.level_appearance.get_style() {
            &Style::CAVE => Music::UNDERGROUND,
            &Style::CASTLE => Music::BREEZY,
            &Style::SAND => Music::BREEZY,
            _ => {
                //println!("LEV: Found {:?}; going with default music...", other);
                Music::BLOCKY
            }
        };
        Lv6::new(lev.name_data.into(),
            CustomContent::from_none(),
            lev.time_limit,
            lev.level_appearance,
            level_content,
            lev.signs.into(),
            music
        )
    }
}