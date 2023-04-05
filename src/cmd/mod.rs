mod add;
mod cli;
mod config;
mod edit;
mod foreach;
mod init;
mod inspect;
mod list;
mod remove;
mod tag;
mod update;
mod work;

use anyhow::Result;

pub use crate::cmd::cli::*;

pub trait Run {
    fn run(self) -> Result<()>;
}

impl Run for Cli {
    fn run(self) -> Result<()> {
        match self.command {
            Cmd::Add(cmd) => cmd.run(),
            Cmd::Config(cmd) => cmd.run(),
            Cmd::Edit(cmd) => cmd.run(),
            Cmd::Foreach(cmd) => cmd.run(),
            Cmd::Inspect(cmd) => cmd.run(),
            Cmd::List(cmd) => cmd.run(),
            Cmd::Remove(cmd) => cmd.run(),
            Cmd::Tag(cmd) => cmd.run(),
            Cmd::Update(cmd) => cmd.run(),
            Cmd::Work(cmd) => cmd.run(),
        }
    }
}
