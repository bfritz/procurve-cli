use anyhow::Result;
use procurve_cli::ProCurveClient;

use clap::{App, ArgMatches, SubCommand};

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("show")
        .about("Shows switch settings")
        .subcommand(
            App::new("description")
                .about("Show model, firmware version, contact info, etc."))
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    let mut client = ProCurveClient::from_env()?;

    match args.subcommand() {
        ("description", Some(_)) => client.describe_switch(),
        (_, _) => unreachable!(),
    }
}
