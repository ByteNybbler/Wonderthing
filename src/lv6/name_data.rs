use serde::{Serialize, Deserialize};
use crate::random::*;

#[derive(Serialize, Deserialize)]
pub struct NameData {
    format_name: String,
    filename: String, // Should be without the .LV6.
    level_version: i32, // Should be made random when serializing the LV6.
    level_name: String
}

impl NameData {
    pub fn new(filename: String, level_name: String) -> NameData {
        NameData {
            format_name: "Stinky & Loof Level File v6".to_owned(),
            filename,
            level_version: 0,
            level_name
        }
    }

    pub fn randomize_level_version<R>(&mut self, rng: &mut R)
    where
        R: Rng<i32>
    {
        self.level_version = rng.random();
    }

    pub fn get_filename_with_extension(&self) -> String {
        format!("{}.LV6", self.filename)
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

    pub fn reverse_level_name(&mut self) {
        self.level_name = self.level_name.chars().rev().collect();
    }
}