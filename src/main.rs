use std::time::{SystemTime, UNIX_EPOCH};


fn main() {
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(n) => println!("{}", n.as_secs()),
        Err(_) => println!("SystemTime before UNIX EPOCH!"),
    }
}
