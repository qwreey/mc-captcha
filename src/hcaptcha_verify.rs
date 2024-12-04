use hcaptcha::{HcaptchaClient, HcaptchaRequest};

pub trait HcaptchaVerifyData {
    // remote_ip:
    fn get_user_captcha_result() {}
}

pub struct HcaptchaVerify {
    pub enabled: bool,
    pub client: HcaptchaClient,
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
            client: HcaptchaClient::new(),
        }
    }
    pub fn verify(&self, data: impl HcaptchaVerifyData) {
        // let req = HcaptchaRequest::new(&self.secret, )
    }
}
