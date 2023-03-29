use anyhow::anyhow;
use repo_cli::shell;

use crate::cli::InitCmd;

use super::ExecuteableCmd;

impl ExecuteableCmd for InitCmd {
    fn execute(self) -> anyhow::Result<()> {
        let shell = match self.shell.as_str() {
            "bash" => shell::Shell::Bash,
            "zsh" => shell::Shell::Zsh,
            "fish" => shell::Shell::Fish,
            _ => return Err(anyhow!("unknown shell: {}", self.shell)),
        };

        let script = shell::init(shell, self.fzf);
        println!("{}", script);

        Ok(())
    }
}
