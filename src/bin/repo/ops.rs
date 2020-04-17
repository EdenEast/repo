use anyhow::Result;
use clap::{
    crate_authors, crate_description, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand,
};

macro_rules! define_app {
    ($( $name:expr => [$t:ty: $aliases:expr], )*) => {
        pub fn app<'a, 'b: 'a>() -> App<'a, 'b> {
            App::new("repo")
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!())
                .bin_name("repo") // stop windows from adding .exe
                .max_term_width(100)
                .settings(&[
                    AppSettings::GlobalVersion,
                    AppSettings::ColoredHelp,
                    AppSettings::UnifiedHelpMessage,
                    AppSettings::VersionlessSubcommands,
                ])
                .arg(Arg::with_name("list-commands-option").help("List installed commands").long("list"))
                $( .subcommand(<$t>::app(SubCommand::with_name($name)).aliases($aliases)) )*
        }

        pub fn commands<'a, 'b: 'a>() -> Vec<App<'a, 'b>> {
            vec![
                $( <$t>::app(SubCommand::with_name($name)).aliases($aliases), )*
            ]
        }

        pub fn run() -> Result<()> {
            let matches = app().get_matches();
            if matches.is_present("list-commands-option") {
                for cmd in list_commands() {
                    let summary = cmd.about.unwrap_or_default();
                    let summary = summary.lines().next().unwrap_or(&summary); // display only the first line
                    println!("{:>15}    {}", cmd.name, summary);
                }
                return Ok(());
            }

            match matches.subcommand() {
                $( ($name, Some(m)) => <$t>::from_matches(m)?.run(m), )*
                _ => {
                    app().print_help()?;
                    println!("");
                    Ok(())
                }
            }
        }
    }
}

define_app! {
    "add" => [self::add::AddCommand: &[]],
    "config" => [self::config::ConfigCommand: &[]],
    "edit" => [self::edit::EditCommand: &[]],
    "foreach" => [self::foreach::ForeachCommand: &[]],
    "init" => [self::init::InitCommand: &[]],
    "inspect" => [self::inspect::InspectCommand: &[]],
    "list" => [self::list::ListCommand: &[]],
    "remove" => [self::remove::RemoveCommand: &[]],
    "tag" => [self::tag::TagCommand: &[]],
    "update" => [self::update::UpdateCommand: &[]],
    "work" => [self::work::WorkCommand: &[]],
}

pub trait CliCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b>;
    fn from_matches(m: &ArgMatches) -> Result<Box<Self>>;
    fn run(self, m: &ArgMatches) -> Result<()>;
}

mod add;
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

struct CommandInfo {
    pub name: String,
    pub about: Option<String>,
}

fn list_commands() -> Vec<CommandInfo> {
    let commands = commands();
    let mut result = Vec::with_capacity(commands.len());
    for cmd in commands {
        result.push(CommandInfo {
            name: cmd.get_name().to_string(),
            about: cmd.p.meta.about.map(|s| s.to_string()),
        })
    }
    result
}
