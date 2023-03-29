use anyhow::anyhow;
use repo_cli::shell;

use super::{InitCmd, Run};

impl Run for InitCmd {
    fn run(self) -> anyhow::Result<()> {
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
