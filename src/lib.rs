use anyhow::{anyhow, Context, Result};
use reqwest::blocking::{multipart, Client};
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

        let login_form = multipart::Form::new().text("pwd", "");

        let res = client
            .post(self.url.clone() + "/hp_login.html")
            .multipart(login_form)
            .send()?;

        let session_cookie = res.cookies().filter(|c| c.name() == "SID").next();

        match session_cookie {
            // FIXME: use proper logging, not println!()
            Some(_) => {
                println!("found session cookie");
                Ok(client)
            }
            None => {
                println!("no session cookie");
                Err(anyhow!("could not login"))
            }
        }
    }

    pub fn describe_switch(&mut self) -> Result<()> {
        let client = self.login()?;
        let mut res = client
            .get(self.url.clone() + "/SysDescription.html")
            .send()?;

        // FIXME: parse HTML and display reasonable output
        res.copy_to(&mut std::io::stdout())?;
        Ok(())
    }
}
