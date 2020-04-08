#[macro_use]
extern crate log;

use anyhow::Result;

fn main() -> Result<()> {
    better_panic::install();
    pretty_env_logger::init();

    ops::run()
}

mod ops;
