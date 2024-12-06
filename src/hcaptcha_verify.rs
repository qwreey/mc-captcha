use hcaptcha::{HcaptchaCaptcha, HcaptchaClient, HcaptchaRequest, HcaptchaResponse};
use qwreey_rocket::{RocketBuild, RouteExport};
use qwreey_utility_rs::ErrToString;

use crate::cli::Cli;

pub trait HcaptchaVerifyData {
    fn get_user_captcha_response(&self) -> Option<&String>;
}

pub struct HcaptchaVerify {
    pub enabled: bool,
    pub secret: String,
    pub sitekey: String,
}
impl HcaptchaVerify {
    pub fn new(secret: Option<String>, sitekey: Option<String>) -> Self {
        if secret.is_some() != sitekey.is_some() {
            panic!("To enable Hcaptcha support, you must provide Sitekey and Secret. please check `mc-captcha --help` to get more information");
        }

        Self {
            enabled: secret.is_some(),
            secret: secret.unwrap_or_else(|| String::from("")),
            sitekey: sitekey.unwrap_or_else(|| String::from("")),
        }
    }
    pub async fn verify(&self, data: &impl HcaptchaVerifyData) -> Result<HcaptchaResponse, String> {
        let user_res = data
            .get_user_captcha_response()
            .ok_or_else(|| String::from("No client response"))?;
        let captcha = HcaptchaCaptcha::new(user_res.as_str()).err_tostring()?;
        let request = HcaptchaRequest::new(&self.secret.trim(), captcha)
            .err_tostring()?
            .set_sitekey(self.sitekey.as_str())
            .err_tostring()?;
        dbg!(&self.secret);
        let api_response = HcaptchaClient::new()
            .verify_client_response(request)
            .await
            .err_tostring()?;
        Ok(api_response)
    }
}

pub struct HcaptchaInit;
impl RouteExport for HcaptchaInit {
    fn build(&self, rocket: RocketBuild, userdata: qwreey_utility_rs::ArcRwUserdata) -> Result<RocketBuild, String> {
        let args = userdata.get_of::<Cli>().unwrap().clone();
        // Push hcaptcha provider
        userdata.insert_of(HcaptchaVerify::new(
            args.hcaptcha_secret.clone(),
            args.hcaptcha_sitekey.clone(),
        ));
        Ok(rocket)
    }
}
