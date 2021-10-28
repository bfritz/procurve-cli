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
    pub fn from_env() -> Result<Self> {
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
            // FIXME: use proper logging, not println!()
            Some(_) => {
                println!("found session cookie");
                Ok(client)
            }
            None => {
                println!("no session cookie");
                dbg!(&res);
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
            bail!("Could not retrieve switch description.  HTTP status: {}", res.status())
        }

        let body = res.text()?;
        let document = Html::parse_document(&body);

        let input_seletor = Selector::parse("input").unwrap();
        let mut inputs = document.select(&input_seletor);

        let description = Description {
            description: value_attribute(inputs.next()),
            name: value_attribute(inputs.next()),
            location: value_attribute(inputs.next()),
            contact: value_attribute(inputs.next()),
            version: value_attribute(inputs.next()),
            object_id: value_attribute(inputs.next()),
            uptime: value_attribute(inputs.next()),
            current_time: value_attribute(inputs.next()),
            current_date: value_attribute(inputs.next()),
        };

        ptable!(
            [b->"Description", description.description],
            [b->"Name", description.name],
            [b->"Location", description.location],
            [b->"Contact", description.contact],
            [b->"Version", description.version],
            [b->"Object ID", description.object_id],
            [b->"Uptime", description.uptime],
            [b->"Current Time", description.current_time],
            [b->"Current Date", description.current_date]);
        Ok(())
    }
}

fn value_attribute(element: Option<ElementRef>) -> &str {
    match element {
        Some(e) => e.value().attr("value").unwrap_or("").trim(),
        None => "",
    }
}

#[derive(Debug)]
struct Description<'a> {
    description: &'a str,
    name: &'a str,
    location: &'a str,
    contact: &'a str,
    version: &'a str,
    object_id: &'a str,
    uptime: &'a str,
    current_time: &'a str,
    current_date: &'a str,
}
