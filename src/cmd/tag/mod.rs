use super::{Run, TagCmd};

impl Run for TagCmd {
    fn run(self) -> anyhow::Result<()> {
        match self.cmd {
            super::TagSubCmd::Add(cmd) => cmd.run(),
            super::TagSubCmd::Edit(cmd) => cmd.run(),
            super::TagSubCmd::List(cmd) => cmd.run(),
            super::TagSubCmd::Remove(cmd) => cmd.run(),
        }
    }
}

mod add;
mod edit;
mod list;
mod remove;
