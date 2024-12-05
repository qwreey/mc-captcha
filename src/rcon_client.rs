use qwreey_rocket::{RocketBuild, RouteExport};
use qwreey_utility_rs::{ErrToString, HeadingError};

use rcon_client::{AuthRequest, RCONClient, RCONConfig, RCONRequest};

use crate::cli::Cli;

pub struct Rcon(RCONClient);

impl Rcon {
    pub fn new(url: String, password: String) -> Result<Self, String> {
        let mut client = RCONClient::new(RCONConfig {
            url,
            read_timeout: Some(10),
            write_timeout: Some(10),
        })
        .err_tostring()?;

        let auth_result = client.auth(AuthRequest::new(password)).err_tostring()?;
        if !auth_result.is_success() {
            return Err(String::from("Auth failed"));
        }

        Ok(Self(client))
    }

    pub fn execute(&mut self, command: String) -> Result<String, String> {
        let result = self.0.execute(RCONRequest::new(command)).err_tostring()?;
        Ok(result.body)
    }
}

pub struct RconInit;
impl RouteExport for RconInit {
    fn build(
        &self,
        rocket: RocketBuild,
        userdata: qwreey_utility_rs::ArcRwUserdata,
    ) -> Result<RocketBuild, String> {
        let args = userdata.get_of::<Cli>().unwrap().clone();

        userdata.insert_of(
            Rcon::new(
                args.rcon_host
                    .clone()
                    .unwrap_or_else(|| String::from("localhost:25575")),
                args.rcon_password.clone(),
            )
            .heading_error("Error in creating rcon client: ")?,
        );
        Ok(rocket)
    }
}
