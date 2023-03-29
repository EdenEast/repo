#[macro_use]
extern crate log;

use anyhow::Result;
use clap::Parser;
use ops::ExecuteableCmd;

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }

    pretty_env_logger::init();

    let cli = cli::Cli::parse();
    match cli.command {
        cli::Cmd::Add(c) => c.execute(),
        cli::Cmd::Config(c) => c.execute(),
        cli::Cmd::Edit(c) => c.execute(),
        cli::Cmd::Foreach(c) => c.execute(),
        cli::Cmd::Insepct(c) => c.execute(),
        cli::Cmd::List(c) => c.execute(),
        cli::Cmd::Remove(c) => c.execute(),
        cli::Cmd::Tag(c) => c.execute(),
        cli::Cmd::Update(c) => c.execute(),
        cli::Cmd::Work(c) => c.execute(),
    }
}

mod cli;
mod ops;
