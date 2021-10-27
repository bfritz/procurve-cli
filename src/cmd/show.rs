use anyhow::Result;

use clap::{App, ArgMatches, SubCommand};

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("show")
        .about("Shows switch settings")
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    Ok(())
}
