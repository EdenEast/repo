use anyhow::Result;
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, AppSettings,
    ArgMatches, SubCommand,
};

macro_rules! define_app {
    ($( $name:expr => [$t:ty: $aliases:expr], )*) => {
        fn app<'a, 'b: 'a>() -> App<'a, 'b> {
            app_from_crate!()
                .bin_name(crate_name!()) // stop windows from adding .exe
                .max_term_width(100)
                .settings(&[
                    AppSettings::GlobalVersion,
                    AppSettings::ColoredHelp,
                    AppSettings::UnifiedHelpMessage,
                    AppSettings::VersionlessSubcommands,
                    AppSettings::SubcommandRequiredElseHelp,
                ])
                $( .subcommand(<$t>::app(SubCommand::with_name($name)).aliases($aliases)) )*
        }

        pub fn run() -> Result<()> {
            let matches = app().get_matches();
            match matches.subcommand() {
                $( ($name, Some(m)) => <$t>::from_matches(m).run(m), )*
                _ => unreachable!(),
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
    "list" => [self::list::ListCommand: &[]],
    "remove" => [self::remove::RemoveCommand: &[]],
    "tag" => [self::tag::TagCommand: &[]],
    "update" => [self::update::UpdateCommand: &[]],
    "work" => [self::work::WorkCommand: &[]],
}

pub trait CliCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b>;
    fn from_matches(m: &ArgMatches) -> Self;
    fn run(self, m: &ArgMatches) -> Result<()>;
}

mod add;
mod config;
mod edit;
mod foreach;
mod init;
mod list;
mod remove;
mod tag;
mod update;
mod work;
