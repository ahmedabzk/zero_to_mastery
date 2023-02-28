use chrono::prelude::*;
fn main() {
    let utc = Utc::now();

    let local = Local::now();

    println!("time in utc: {}", utc);
    println!("local time: {}", local.format("%Y-%m-%d %H:%M:%S").to_string());
}
