use qwreey_utility_rs::ErrToString;

use rcon_client::{AuthRequest, RCONClient, RCONConfig, RCONRequest};

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
        let result = self
            .0
            .execute(RCONRequest::new(command))
            .err_tostring()?;
        Ok(result.body)
    }
}
