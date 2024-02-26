use std::time;

pub fn now_date() -> u64 {
    let now = time::SystemTime::now();
    match now.duration_since(time::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
