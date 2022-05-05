#[macro_use]
extern crate clap;

use anyhow::Result;
use clap::Command;

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

fn create_clap_app<'a>() -> Command<'a> {
    Command::new(crate_name!())
        .about(crate_description!())
        .version(VERSION)
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(cmd::show::make_subcommand())
}
