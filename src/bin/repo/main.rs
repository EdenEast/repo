use anyhow::Result;

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    better_panic::install();
    pretty_env_logger::init();

    ops::run()
}

mod ops;
