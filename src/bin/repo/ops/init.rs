use super::CliCommand;
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
use repo::shell;

pub struct InitCommand {
    shell: String,
    fzf: bool,
}

impl CliCommand for InitCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Prints the shell function used to integrate repo with shell")
            .settings(&[AppSettings::Hidden, AppSettings::NextLineHelp])
            .arg(
                Arg::with_name("SHELL")
                    .help("Name of the shell the shell function will generate")
                    .possible_values(&["bash", "zsh", "fish"])
                    .required(true),
            )
            .arg(
                Arg::with_name("fzf")
                    .help("Intilaize with fzf integration")
                    .long("fzf")
                    .short("f"),
            )
    }

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            shell: m
                .value_of("SHELL")
                .map(String::from)
                .expect("SHELL is a required argument"),
            fzf: m.is_present("fzf"),
        }
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
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
