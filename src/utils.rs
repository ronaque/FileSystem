use std::time;

pub fn clear_screen() {
    /*! x1B = Escape
     * [2J = clear screen
     * x1B[1;1H = Move cursor x/y */
    print!("\x1B[2J\x1B[1;1H");
}

pub fn now_date() -> u64 {
    let now = time::SystemTime::now();
    match now.duration_since(time::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
