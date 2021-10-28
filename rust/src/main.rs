#[macro_use]
extern crate clap;

use anyhow::Result;
use clap::{App, AppSettings};

mod cmd;

const VERSION: &str = concat!("v", crate_version!());

fn main() -> Result<()> {
    let app = create_clap_app();

    match app.get_matches().subcommand() {
        ("show", Some(sub_matches)) => cmd::show::execute(sub_matches),
        (_, _) => unreachable!(),
    }
}

fn create_clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .about(crate_description!())
        .version(VERSION)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommand(cmd::show::make_subcommand())
}
