use std::net::IpAddr;

use qwreey_utility_rs::ErrToString;
use rocket::{config::Ident, Config, Ignite, Rocket};

use super::*;

pub struct WebBackendBuilder {
    port: u16,
    address: IpAddr,
    userdata: Option<WebBackendUserdata>,
}
impl Default for WebBackendBuilder {
    fn default() -> Self {
        Self {
            port: 80,
            address: "127.0.0.1".parse::<IpAddr>().unwrap(),
            userdata: None,
        }
    }
}
impl WebBackendBuilder {
    pub async fn build(self) -> Result<Rocket<Ignite>, String> {
        let rocket = rocket::build()
            .configure(Config {
                cli_colors: false,
                ident: Ident::try_new("MC-Captcha").unwrap(),
                port: self.port,
                address: self.address,
                ..Default::default()
            })
            .mount("/", ROUTES.clone())
            .manage(self.userdata.unwrap())
            .launch()
            .await
            .err_tostring()?;
        Ok(rocket)
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn port(mut self, port: Option<u16>) -> Self {
        self.port = port.unwrap_or(Self::default().port);
        self
    }
    pub fn address(mut self, address: Option<IpAddr>) -> Self {
        self.address = address.unwrap_or(Self::default().address);
        self
    }
    pub fn userdata(mut self, userdata: WebBackendUserdata) -> Self {
        self.userdata = Some(userdata);
        self
    }
}
