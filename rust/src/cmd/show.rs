use anyhow::Result;
use procurve_cli::ProCurveClient;

use clap::{App, ArgMatches, SubCommand};

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("show")
        .about("Shows switch settings")
        .subcommand(
            App::new("description").about("Show model, firmware version, contact info, etc."),
        )
        .subcommand(App::new("vlans").about("Show VLAN configuration."))
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    let mut client = ProCurveClient::new()?;

    match args.subcommand() {
        ("description", Some(_)) => client.describe_switch(),
        ("vlans", Some(_)) => client.describe_vlans(),
        (_, _) => unreachable!(),
    }
}
