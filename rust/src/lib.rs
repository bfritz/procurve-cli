#[macro_use]
extern crate prettytable;

use anyhow::{anyhow, bail, Context, Result};
use reqwest::blocking::Client;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;
use std::env;

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

    fn login(&mut self) -> Result<Client> {
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .build()?;

        let login_form = HashMap::from([("pwd", "")]);

        let res = client
            .post(self.url.clone() + "/hp_login.html")
            .form(&login_form)
            .send()?;

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
        let res = client
            .get(self.url.clone() + "/SysDescription.html")
            .send()?;

        if !res.status().is_success() {
            bail!(
                "Could not retrieve switch description.  HTTP status: {}",
                res.status()
            )
        }

        let body = res.text()?;
        let description = html_to_description(&body)?;
        print_description_as_table(description)?;
        Ok(())
    }
}

fn html_to_description(body: &str) -> Result<Description> {
    let document = Html::parse_document(&body);

    let input_seletor = Selector::parse("input").unwrap();
    let mut inputs = document.select(&input_seletor);

    let description = Description {
        description: value_attribute(inputs.next(), "description")?,
        name: value_attribute(inputs.next(), "name")?,
        location: value_attribute(inputs.next(), "location")?,
        contact: value_attribute(inputs.next(), "contact")?,
        version: value_attribute(inputs.next(), "version")?,
        object_id: value_attribute(inputs.next(), "object_id")?,
        uptime: value_attribute(inputs.next(), "uptime")?,
        current_time: value_attribute(inputs.next(), "current_time")?,
        current_date: value_attribute(inputs.next(), "current_date")?,
    };

    Ok(description)
}

fn value_attribute<'a>(element: Option<ElementRef>, field_name: &str) -> Result<String> {
    match element {
        Some(e) => Ok(e.value().attr("value").unwrap_or("").trim().to_string()),
        None => bail!("HTML element for field {} not found", field_name),
    }
}

fn print_description_as_table(description: Description) -> Result<usize> {
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

#[derive(Debug)]
struct Description {
    description: String,
    name: String,
    location: String,
    contact: String,
    version: String,
    object_id: String,
    uptime: String,
    current_time: String,
    current_date: String,
}

#[cfg(test)]
mod tests {
    use anyhow::Context;

    use super::html_to_description;

    #[test]
    fn parses_description_html_correctly() {
        let body = std::fs::read_to_string("../samples/SysDescription.html")
            .with_context(|| "opening samples/SysDescription.html")
            .unwrap();
        let description = html_to_description(&body)
            .with_context(|| "converting SysDescription.html into Description struct")
            .unwrap();
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
}
