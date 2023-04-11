use std::{fs, io::Result};

use clap::{Command, CommandFactory};

include!("./src/cmd/cli.rs");

struct ManPage<'a> {
    name: &'a str,
    command: Command,
}

impl<'a> ManPage<'a> {
    fn new(name: &'a str, command: Command) -> Self {
        Self { name, command }
    }
}

const MAN_PATH: &str = "./target/man";

fn generate_manpage(name: &str, app: &Command) -> Result<()> {
    let man = clap_mangen::Man::new(app.to_owned());
    let mut man_buffer: Vec<u8> = Default::default();
    man.render(&mut man_buffer)?;
    let out_dir = std::env::current_dir()?;
    std::fs::write(out_dir.join(format!("{MAN_PATH}/{name}.1")), man_buffer)
}

fn generate_manpages() -> Result<()> {
    let pages: Vec<ManPage> = vec![
        ManPage::new("repo", Cli::command()),
        ManPage::new("repo-add", AddCmd::command()),
        ManPage::new("repo-config", ConfigCmd::command()),
        ManPage::new("repo-edit", EditCmd::command()),
        ManPage::new("repo-foreach", ForeachCmd::command()),
        ManPage::new("repo-inspect", InspectCmd::command()),
        ManPage::new("repo-init", InitCmd::command()),
        ManPage::new("repo-list", ListCmd::command()),
        ManPage::new("repo-remove", RemoveCmd::command()),
        ManPage::new("repo-tag", TagCmd::command()),
        ManPage::new("repo-tag-add", TagAddCmd::command()),
        ManPage::new("repo-tag-edit", TagEditCmd::command()),
        ManPage::new("repo-tag-list", TagListCmd::command()),
        ManPage::new("repo-tag-remove", TagRemoveCmd::command()),
        ManPage::new("repo-update", UpdateCmd::command()),
        ManPage::new("repo-update", UpdateCmd::command()),
        ManPage::new("repo-work", WorkCmd::command()),
    ];

    fs::create_dir_all(MAN_PATH)?;
    for page in pages {
        generate_manpage(page.name, &page.command)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    generate_manpages()?;
    Ok(())
}
