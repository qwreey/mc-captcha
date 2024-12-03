use hcaptcha::{HcaptchaClient, HcaptchaRequest};

trait HcaptchaVerifyData {
    // remote_ip:
    fn get_user_captcha_result() {}
}

struct HcaptchaVerify {
    client: HcaptchaClient,
    secret: String,
}
impl HcaptchaVerify {
    fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),
            client: HcaptchaClient::new(),
        }
    }
    fn verify(&self, data: impl HcaptchaVerifyData) {
        // let req = HcaptchaRequest::new(&self.secret, )
    }
}
