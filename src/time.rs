use std::time::*;

pub fn get_current_unix_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards").as_secs()
}