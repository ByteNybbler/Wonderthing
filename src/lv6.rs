mod custom_content;
pub use custom_content::*;
mod level_appearance;
pub use level_appearance::*;
mod level_content;
pub use level_content::*;
mod music;
pub use music::*;
mod name_data;
pub use name_data::*;
mod object;
pub use object::*;
mod signs;
pub use signs::*;
mod tile;
pub use tile::*;
mod time_limit;
pub use time_limit::*;

use serde::{Serialize, Deserialize};
use std::fmt::{self, Display};
use std::fs::{self, File};
use std::path::*;
use crate::random::*;
use crate::serde_blitz3d;

#[derive(Serialize, Deserialize)]
pub struct Lv6 {
    name_data: NameData,
    custom_content: CustomContent,
    time_limit: TimeLimit,
    level_appearance: LevelAppearance,
    level_content: LevelContent,
    signs: Signs,
    music: Music
}

impl Lv6 {
    pub fn new(name_data: NameData, custom_content: CustomContent, time_limit: TimeLimit, level_appearance: LevelAppearance,
        level_content: LevelContent, signs: Signs, music: Music) -> Lv6 {
        Lv6 {
            name_data,
            custom_content,
            time_limit,
            level_appearance,
            level_content,
            signs,
            music
        }
    }

    pub fn to_file<N>(&mut self, rng: &mut N, output_folder: &str) -> Result<(), Error>
    where
        N: Rng<i32>
    {
        self.name_data.randomize_level_version(rng);
        fs::create_dir_all(output_folder).map_err(Error::InputOutput)?;
        let path = PathBuf::from(output_folder).join(self.name_data.get_filename_with_extension());
        let file = File::create(path).map_err(Error::InputOutput)?;
        serde_blitz3d::to_writer(file, self).map_err(Error::Serde)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Lv6, Error> {
        let file = File::open(path).map_err(Error::InputOutput)?;
        serde_blitz3d::from_reader(file).map_err(Error::Serde)
    }

    pub fn set_names(&mut self, filename: String, level_name: String) {
        self.name_data.set_filename(filename);
        self.name_data.set_level_name(level_name);
    }

    pub fn set_time_limit(&mut self, time_limit: TimeLimit) {
        self.time_limit = time_limit;
    }

    pub fn get_filename(&self) -> &str {
        self.name_data.get_filename()
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

    pub fn set_music(&mut self, music: Music) {
        self.music = music;
    }

    pub fn clear_signs(&mut self) {
        self.signs.clear();
        for t in self.level_content.borrow_tiles_mut() {
            if t.is_sign() {
                *t = Tile::FLOOR;
            }
        }
    }

    pub fn reverse_signs(&mut self) {
        self.signs.reverse();
    }

    pub fn reverse_level_name(&mut self) {
        self.name_data.reverse_level_name();
    }
}

#[derive(Debug)]
pub enum Error {
    InputOutput(std::io::Error),
    Serde(serde_blitz3d::Error)
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InputOutput(error) => write!(f, "{}", error),
            Error::Serde(error) => write!(f, "{}", error)
        }
    }
}

impl std::error::Error for Error { }