#[macro_use]
extern crate prettytable;

use anyhow::{anyhow, bail, Context, Result};
use prettytable::{Row, Table};
use reqwest::blocking::Client;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;
use std::env;

mod model;
use model::Description;
use model::{Vlan, VlanMode, VlanPortConfig, VlanPortParticipation};

pub struct ProCurveClient {
    // The HTTP or HTTPS URL to the switch's web management page, [`SWITCH_URL`] by default.
    pub url: String,
}

impl ProCurveClient {
    pub fn new() -> Result<Self> {
        Ok(ProCurveClient {
            url: env::var("SWITCH_URL")
                .with_context(|| "SWITCH_URL environment variable missing")?,
        })
    }

    fn login_url(&self) -> String {
        format!("{}/hp_login.html", &self.url)
    }

    fn description_url(&self) -> String {
        format!("{}/SysDescription.html", &self.url)
    }

    fn vlan_port_participation_url(&self) -> String {
        format!("{}/VLANPortParticipation.html", &self.url)
    }

    fn vlan_create_config_url(&self) -> String {
        format!("{}/VlanCreateConfig.html", &self.url)
    }

    fn login(&mut self) -> Result<Client> {
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .build()?;

        let login_form = HashMap::from([("pwd", "")]);

        let res = client.post(self.login_url()).form(&login_form).send()?;

        let session_cookie = res.cookies().find(|c| c.name() == "SID");

        match session_cookie {
            Some(_) => {
                log::debug!("Found session cookie in login response.");
                Ok(client)
            }
            None => {
                log::error!("Did not find session cookie in login response.\n{:?}", &res);
                Err(anyhow!("could not login"))
            }
        }
    }

    pub fn describe_switch(&mut self) -> Result<()> {
        let client = self.login()?;
        let res = client.get(self.description_url()).send()?;

        if !res.status().is_success() {
            bail!(
                "Could not retrieve switch description.  HTTP status: {}",
                res.status()
            )
        }

        let body = res.text()?;
        let description = html_to_description(&body)?;
        print_description_as_table(&description)?;
        Ok(())
    }

    pub fn describe_vlans(&mut self) -> Result<()> {
        let client = self.login()?;
        let res = client.get(self.vlan_create_config_url()).send()?;

        if !res.status().is_success() {
            bail!(
                "Could not retrieve VLAN configuration.  HTTP status: {}",
                res.status()
            )
        }

        let body = res.text()?;
        let vlan_list = html_to_vlan_configuration(&body)?;

        let mut port_participation_map = HashMap::new();
        for vlan in &vlan_list {
            let res = client
                .post(self.vlan_port_participation_url())
                .form(&vlan_select_form(vlan.id))
                .send()?;

            if !res.status().is_success() {
                bail!(
                    "Could not retrieve VLAN {} port participation.  HTTP status: {}",
                    vlan.id,
                    res.status()
                )
            }
            let body = res.text()?;
            port_participation_map.insert(vlan.id, html_to_vlan_participation(&body, vlan.id)?);
        }

        print_vlans_as_table(&vlan_list, &port_participation_map)?;
        Ok(())
    }
}

fn vlan_select_form(vlan_id: u16) -> HashMap<String, String> {
    let mut params = HashMap::new();
    params.insert(String::from("v_1_1_1"), vlan_id.to_string());
    params.insert(String::from("submit_flag"), String::from("1"));
    params.insert(
        String::from("submit_target"),
        String::from("VLANPortParticipation.html"),
    );
    params
}

fn html_to_description(body: &str) -> Result<Description> {
    let document = Html::parse_document(body);

    let input_seletor = Selector::parse("input").unwrap();
    let mut inputs = document.select(&input_seletor);

    let description = Description {
        description: value_attribute(&inputs.next(), "description")?,
        name: value_attribute(&inputs.next(), "name")?,
        location: value_attribute(&inputs.next(), "location")?,
        contact: value_attribute(&inputs.next(), "contact")?,
        version: value_attribute(&inputs.next(), "version")?,
        object_id: value_attribute(&inputs.next(), "object_id")?,
        uptime: value_attribute(&inputs.next(), "uptime")?,
        current_time: value_attribute(&inputs.next(), "current_time")?,
        current_date: value_attribute(&inputs.next(), "current_date")?,
    };

    Ok(description)
}

fn html_to_vlan_configuration(body: &str) -> Result<Vec<Vlan>> {
    let num_trailing_inputs_to_ignore = 6;
    let document = Html::parse_document(body);

    let input_seletor = Selector::parse("input").unwrap();
    let inputs: Vec<ElementRef> = document.select(&input_seletor).skip(4).collect();
    let mut vlans = Vec::new();
    for i in (0..inputs.len() - num_trailing_inputs_to_ignore).step_by(4) {
        let vlan_id_input = Some(inputs[i].to_owned());
        let vlan_id = value_attribute(&vlan_id_input, "vlan_id")?
            .parse()
            .expect("string should be u16 integer value");

        let vlan_name_input = Some(inputs[i + 1].to_owned());
        let vlan_name = value_attribute(&vlan_name_input, "vlan_name")?;
        vlans.push(Vlan::new(vlan_id, vlan_name.as_str()));
    }
    Ok(vlans)
}

fn html_to_vlan_participation(body: &str, vlan_id: u16) -> Result<VlanPortParticipation> {
    let num_trailing_inputs_to_ignore = 6;
    let document = Html::parse_document(body);

    let input_seletor = Selector::parse("input").unwrap();
    let inputs: Vec<ElementRef> = document.select(&input_seletor).skip(6).collect();
    let mut port_config = Vec::new();
    for i in (0..inputs.len() - num_trailing_inputs_to_ignore).step_by(8) {
        let port_num_input = Some(inputs[i].to_owned());
        let port_num: u8 = value_attribute(&port_num_input, "port_num")?
            .parse()
            .expect("string should be u8 integer value");
        assert!(port_num > 0);

        let inc_exc_input = Some(inputs[i + 2].to_owned());
        let inc_exc = value_attribute(&inc_exc_input, "include_or_exclude")?;
        let tag_mode_input = Some(inputs[i + 3].to_owned());
        let tag_mode = value_attribute(&tag_mode_input, "tag_mode")?;
        let vlan_mode = match (inc_exc.as_str(), tag_mode.as_str()) {
            ("Include", "Tagged") => VlanMode::Tagged,
            ("Exclude", "UnTagged") => VlanMode::Excluded,
            ("Include", "UnTagged") => VlanMode::Untagged,
            (ie, mode) => bail!("unexpected mode: {} + {}", ie, mode),
        };

        port_config.insert((port_num - 1).into(), VlanPortConfig::new(vlan_mode));
    }

    Ok(VlanPortParticipation::new(vlan_id, port_config))
}

fn value_attribute(element: &Option<ElementRef>, field_name: &str) -> Result<String> {
    match element {
        Some(e) => Ok(e.value().attr("value").unwrap_or("").trim().to_string()),
        None => bail!("HTML element for field {} not found", field_name),
    }
}

fn print_description_as_table(description: &Description) -> Result<()> {
    let table = table!(
        [b->"Description", description.description],
        [b->"Name", description.name],
        [b->"Location", description.location],
        [b->"Contact", description.contact],
        [b->"Version", description.version],
        [b->"Object ID", description.object_id],
        [b->"Uptime", description.uptime],
        [b->"Current Time", description.current_time],
        [b->"Current Date", description.current_date]);
    Ok(table.printstd())
}

fn print_vlans_as_table(
    vlans: &[Vlan],
    port_participation: &HashMap<u16, VlanPortParticipation>,
) -> Result<()> {
    let mut table = Table::new();
    let mut header_row = Row::empty();
    header_row.add_cell(cell![b->"VLAN ID"]);
    header_row.add_cell(cell![b->"Name"]);
    let port_count = port_participation
        .values()
        .next()
        .expect("at least one participation")
        .ports
        .len();
    for port_num in 1..=port_count {
        header_row.add_cell(cell![b->port_num]);
    }

    table.add_row(header_row);
    for vlan in vlans {
        let row = match port_participation.get(&vlan.id) {
            Some(pp) => add_participation_to_row(row![vlan.id, vlan.name], pp),
            None => row![vlan.id, vlan.name],
        };
        table.add_row(row);
    }
    Ok(table.printstd())
}

fn add_participation_to_row(mut row: Row, participation: &VlanPortParticipation) -> Row {
    for port in &participation.ports {
        row.add_cell(cell![port.mode])
    }
    row
}

#[cfg(test)]
mod tests {
    use super::html_to_description;
    use super::html_to_vlan_configuration;
    use super::html_to_vlan_participation;
    use super::model::VlanMode;

    #[test]
    fn parses_description_html_correctly() {
        let body = std::fs::read_to_string("../samples/SysDescription.html")
            .expect("file open: samples/SysDescription.html");
        let description = html_to_description(&body)
            .expect("convert SysDescription.html into Description struct");
        assert_eq!(
            description.description,
            "HP ProCurve 1810G - 8 GE, P.2.22, eCos-2.0, CFE-2.1"
        );
        assert_eq!(description.name, "PROCURVE J9449A");
        assert_eq!(description.contact, "");
        assert_eq!(description.location, "");
        assert_eq!(description.version, "P.2.22");
        assert_eq!(description.object_id, "1.3.6.1.4.1.11.2.3.7.11.103");
        assert_eq!(description.uptime, "2 days, 22 hours, 20 mins, 22 secs");
        assert_eq!(description.current_time, "22:20:22");
        assert_eq!(description.current_date, "01/03/1970");
    }

    #[test]
    fn parses_vlan_configuration_correctly() {
        let body = std::fs::read_to_string("../samples/VlanCreateConfig.html")
            .expect("file open: samples/VlanCreateConfig.html");
        let vlans = html_to_vlan_configuration(&body)
            .expect("convert samples/VlanCreateConfig.html into list of VLANs");
        assert_eq!(vlans.len(), 3);
        // Default
        assert_eq!(vlans[0].id, 1);
        assert_eq!(vlans[0].name, "Default");
        // lan
        assert_eq!(vlans[1].id, 101);
        assert_eq!(vlans[1].name, "lan");
        // dmz
        assert_eq!(vlans[2].id, 102);
        assert_eq!(vlans[2].name, "dmz");
    }

    #[test]
    fn parses_vlan_participation_correctly() {
        let body = std::fs::read_to_string("../samples/VLANPortParticipation.html")
            .expect("file open: samples/VLANPortParticipation.html");
        let participation = html_to_vlan_participation(&body, 123)
            .expect("convert samples/VLANPortParticipation.html into list of VLANs");
        assert_eq!(participation.ports.len(), 8);
        assert!(matches!(participation.ports[0].mode, VlanMode::Tagged));
        assert!(matches!(participation.ports[1].mode, VlanMode::Excluded));
        assert!(matches!(participation.ports[2].mode, VlanMode::Excluded));
        assert!(matches!(participation.ports[3].mode, VlanMode::Untagged));
        assert!(matches!(participation.ports[4].mode, VlanMode::Excluded));
        assert!(matches!(participation.ports[5].mode, VlanMode::Excluded));
        assert!(matches!(participation.ports[6].mode, VlanMode::Excluded));
        assert!(matches!(participation.ports[7].mode, VlanMode::Excluded));
    }
}
