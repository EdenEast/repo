use anyhow::Result;
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, AppSettings,
    ArgMatches, SubCommand,
};

macro_rules! define_app {
    ($( $name:expr => [$t:ty: $aliases:expr], )*) => {
        fn app<'a, 'b: 'a>() -> App<'a, 'b> {
            app_from_crate!()
                .setting(AppSettings::VersionlessSubcommands)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                $( .subcommand(<$t>::app(SubCommand::with_name($name)).aliases($aliases)) )*
        }

        pub fn run() -> Result<()> {
            let matches = app().get_matches();
            match matches.subcommand() {
                $( ($name, Some(m)) => <$t>::from_matches(m).run(), )*
                _ => unreachable!(),
            }
        }
    }
}

define_app! {
    "add" => [self::add::AddCommand: &[]],
}

pub trait CliCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b>;
    fn from_matches(m: &ArgMatches) -> Self;
    fn run(self) -> Result<()>;
}

mod add;
