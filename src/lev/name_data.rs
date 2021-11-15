use serde::{Serialize, Deserialize};
use crate::lv6;

#[derive(Serialize, Deserialize)]
pub struct NameData {
    format_name: String,
    filename: String, // Should be without the file extension.
    level_name: String
}

impl NameData {
    pub fn new(filename: String, level_name: String) -> NameData {
        NameData {
            format_name: "Stinky & Loof Level File".to_owned(),
            filename,
            level_name
        }
    }

    pub fn get_filename_with_extension(&self) -> String {
        format!("{}.LEV", self.filename)
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }

    pub fn get_level_name(&self) -> &str {
        &self.level_name
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename;
    }

    pub fn set_level_name(&mut self, level_name: String) {
        self.level_name = level_name;
    }
}

impl From<NameData> for lv6::NameData {
    fn from(old: NameData) -> lv6::NameData {
        lv6::NameData::new(old.filename, old.level_name)
    }
}