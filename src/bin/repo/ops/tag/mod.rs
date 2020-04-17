use crate::ops::CliCommand;
use anyhow::Result;
use clap::{App, ArgMatches, SubCommand};

pub struct TagCommand {}

impl CliCommand for TagCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Manage tags").subcommands(internal_commands())
    }

    fn from_matches(_: &ArgMatches) -> Result<Box<Self>> {
        Ok(Box::new(Self {}))
    }

    fn run(self, m: &ArgMatches) -> Result<()> {
        let (cmd, matches) = m.subcommand();
        internal_run(cmd, matches)
    }
}

macro_rules! define_run {
    ($( $name:expr => [$t:ty: $aliases:expr], )*) => {
        fn internal_commands<'a, 'b: 'a>() -> Vec<App<'a, 'b>> {
            vec![
                $( <$t>::app(SubCommand::with_name($name)).aliases($aliases), )*
            ]
        }

        fn internal_run(command: &str, matches: Option<&ArgMatches>) -> Result<()> {
            match (command, matches) {
                $( ($name, Some(m)) => <$t>::from_matches(m)?.run(m), )*
                _ => unreachable!(),
            }
        }
    }
}

define_run! {
    "add" => [self::add::AddCommand: &[]],
    "edit" => [self::edit::EditCommand: &[]],
    "list" => [self::list::ListCommand: &[]],
    "remove" => [self::remove::RemoveCommand: &[]],
}

mod add;
mod edit;
mod list;
mod remove;
