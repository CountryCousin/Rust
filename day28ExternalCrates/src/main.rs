// Day 28 of Rust, External crates are way to include other peoples code in your program
// thereby increasing productivity and making it easier to write complex application
//  To add crates use crates.io(is the official repo for external crates in Rust ecosystem),
// there is also an unofficail website called "lib.rs"(this categorises and ranks crates)

// usage of "humantime" crate: Humantime crates takes seconds and minutes and converts it into
// a friendly user time

use humantime::format_duration;
use std::time::Duration;
fn main() {
    let person_time = Duration::from_secs(100001); //creation of Duration

    // formating to human readable time.
    println!("{}", format_duration(person_time));
}
