use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CustomContentString {
    flag: bool,
    name: String
}

impl CustomContentString {
    pub fn from_none() -> CustomContentString {
        CustomContentString {
            flag: false,
            name: "".to_owned()
        }
    }

    pub fn from_existing(name: String) -> CustomContentString {
        CustomContentString {
            flag: true,
            name
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CustomContent {
    custom_houses: CustomContentString,
    custom_models: CustomContentString,
    custom_texture: CustomContentString,
    custom_background: CustomContentString
}

impl CustomContent {
    pub fn from_none() -> CustomContent {
        CustomContent {
            custom_houses: CustomContentString::from_none(),
            custom_models: CustomContentString::from_none(),
            custom_texture: CustomContentString::from_none(),
            custom_background: CustomContentString::from_none()
        }
    }
}