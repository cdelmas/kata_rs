extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate log;

extern crate kata_rs;

use kata_rs::kata::*;

fn main() {
    env_logger::init();

    info!("Hello, world!");
    it_works();
}
