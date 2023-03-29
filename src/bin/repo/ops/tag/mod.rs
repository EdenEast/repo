use crate::cli::TagCmd;

use super::ExecuteableCmd;

impl ExecuteableCmd for TagCmd {
    fn execute(self) -> anyhow::Result<()> {
        match self.cmd {
            crate::cli::TagSubCmd::Add(c) => c.execute(),
            crate::cli::TagSubCmd::Edit(c) => c.execute(),
            crate::cli::TagSubCmd::List(c) => c.execute(),
            crate::cli::TagSubCmd::Remove(c) => c.execute(),
        }
    }
}

mod add;
mod edit;
mod list;
mod remove;
