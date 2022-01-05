#[macro_use]
extern crate clap;

use anyhow::Result;
use clap::{App, AppSettings};

mod cmd;

const VERSION: &str = concat!("v", crate_version!());

fn main() -> Result<()> {
    env_logger::init();

    let app = create_clap_app();

    match app.get_matches().subcommand() {
        Some(("show", sub_matches)) => cmd::show::execute(sub_matches),
        _ => unreachable!(),
    }
}

fn create_clap_app<'a>() -> App<'a> {
    App::new(crate_name!())
        .about(crate_description!())
        .version(VERSION)
        .setting(AppSettings::PropagateVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(cmd::show::make_subcommand())
}
