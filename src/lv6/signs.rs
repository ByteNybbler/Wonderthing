use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Signs {
    signs: [String; 20]
}

impl Signs {
    pub fn from_none() -> Signs {
        Signs {
            signs: Default::default()
        }
    }

    pub fn from_slice(strings: &[String]) -> Signs {
        let mut signs: [String; 20] = Default::default();
        for i in 0..strings.len() {
            signs[i] = strings[i].to_owned();
        }
        Signs {
            signs
        }
    }

    pub fn clear(&mut self) {
        self.signs = Default::default();
    }

    pub fn reverse(&mut self) {
        for i in 0..20 {
            self.signs[i] = self.signs[i].chars().rev().collect();
        }
    }
}