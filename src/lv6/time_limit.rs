use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TimeLimit {
    seconds: i32
}

impl TimeLimit {
    pub fn from_seconds(seconds: i32) -> TimeLimit {
        TimeLimit {
            seconds
        }
    }

    pub fn from_minutes_and_seconds(minutes: i32, seconds: i32) -> TimeLimit {
        TimeLimit::from_seconds(minutes * 60 + seconds)
    }
}