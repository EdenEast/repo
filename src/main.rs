use std::{
    io::{self, Write},
    process::ExitCode,
};

use clap::Parser;
use cmd::{Cli, Run};

mod cmd;

fn main() -> ExitCode {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }

    pretty_env_logger::init();

    match Cli::parse().run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            _ = writeln!(io::stderr(), "{e:?}");
            ExitCode::FAILURE
        }
    }
}
