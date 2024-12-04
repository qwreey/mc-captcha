use std::sync::Arc;

use qwreey_rocket::{
    ElementResponder, RocketOrbit, RouteExport, RouteList, TemplateToContent, ToElementResponder,
    UserdataState,
};
use qwreey_utility_rs::ArcRwUserdata;
use rocket::{get, http::ContentType, routes};
use rocket_dyn_templates::context;

use crate::hcaptcha_verify::HcaptchaVerify;

#[get("/")]
pub fn handle(userdata: &UserdataState) -> ElementResponder {
    userdata
        .get::<Arc<str>>("index-cached")
        .unwrap()
        .to_element_responder(ContentType::HTML)
}

pub struct Index;
impl RouteExport for Index {
    fn routes(&self) -> RouteList {
        routes![handle]
    }
    fn orbit(&self, rocket: &RocketOrbit, userdata: ArcRwUserdata) {
        let hcaptcha = userdata.get_of::<HcaptchaVerify>().unwrap();
        userdata.insert::<Arc<str>>(
            "index-cached",
            rocket.template_to_content(
                "index",
                context! {
                    site: context! {
                        title: &"Test",
                    },
                    hcaptcha: context! {
                        enabled: hcaptcha.enabled,
                        sitekey: &hcaptcha.sitekey,
                        secret: &hcaptcha.secret,
                    }
                },
            ),
        );
    }
}
