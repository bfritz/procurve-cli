use anyhow::Result;
use procurve_cli::ProCurveClient;

use clap::{App, AppSettings, ArgMatches};

pub fn make_subcommand<'a>() -> App<'a> {
    App::new("show")
        .about("Shows switch settings")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("description").about("Show model, firmware version, contact info, etc."),
        )
        .subcommand(App::new("vlans").about("Show VLAN configuration."))
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    let mut client = ProCurveClient::new()?;

    match args.subcommand() {
        Some(("description", _)) => client.describe_switch(),
        Some(("vlans", _)) => client.describe_vlans(),
        _ => unreachable!(),
    }
}
