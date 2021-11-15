use serde::{Serialize, Deserialize};
use crate::lv6;

#[derive(Serialize, Deserialize)]
pub struct Signs {
    signs: [String; 5]
}

impl Signs {
    pub fn from_none() -> Signs {
        Signs {
            signs: Default::default()
        }
    }
}

impl From<Signs> for lv6::Signs {
    fn from(old: Signs) -> lv6::Signs {
        lv6::Signs::from_slice(&old.signs)
    }
}