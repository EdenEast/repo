#[macro_use]
extern crate log;

use anyhow::Result;

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }

    pretty_env_logger::init();

    ops::run()
}

mod ops;
