extern crate dipstick;
#[macro_use]
extern crate lazy_static;

use std::time::Duration;
use std::thread;

mod metric;

fn main() {
    metric::METRICS.flush_every(Duration::from_secs(3));
    loop {
        thread::sleep(Duration::from_secs(4));
        println!("{}", "incrementing GET counter");
        metric::GET.count(1);
    }
}
