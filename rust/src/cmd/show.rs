use anyhow::Result;
use procurve_cli::ProCurveClient;

use clap::{ArgMatches, Command};

pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("show")
        .about("Shows switch settings")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("description").about("Show model, firmware version, contact info, etc."),
        )
        .subcommand(Command::new("vlans").about("Show VLAN configuration."))
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    let mut client = ProCurveClient::new()?;

    match args.subcommand() {
        Some(("description", _)) => client.describe_switch(),
        Some(("vlans", _)) => client.describe_vlans(),
        _ => unreachable!(),
    }
}
