pub mod add;
pub mod config;
pub mod edit;
pub mod foreach;
pub mod init;
pub mod inspect;
pub mod list;
pub mod remove;
pub mod tag;
pub mod update;
pub mod work;

use anyhow::Result;

pub trait ExecuteableCmd {
    fn execute(self) -> Result<()>;
}
