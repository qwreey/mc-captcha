use std::{collections::HashMap, sync::Arc};

use qwreey_rocket::{
    ElementResponder, RocketOrbit, RouteExport, RouteList, TemplateToContent, ToElementResponder,
    UserdataState,
};
use qwreey_utility_rs::{write_map, ArcRwUserdata, OrAsStr};
use rocket::{get, http::ContentType, routes};
use rocket_dyn_templates::context;

use crate::{cli::Cli, HcaptchaVerify, QuestionList};

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
    fn orbit(&self, rocket: &RocketOrbit, userdata: ArcRwUserdata) -> Result<(), String> {
        let hcaptcha = userdata.get_of::<HcaptchaVerify>().unwrap();
        let cli = userdata.get_of::<Cli>().unwrap();
        let question_list = userdata.get_of::<QuestionList>().unwrap();
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
                        title: cli.hcaptcha_title.or_as_str("캡챠를 진행해주세요"),
                    },
                    question_list: &*question_list,
                    global: write_map!(HashMap::<&str, String>::new(), {
                        "id" => "global".to_string(),
                        "QuestionLength" => question_list.len().to_string(),
                    }),
                    minecraft_name: context! {
                        title: cli.minecraft_name_title.or_as_str("마인크래프트 유저 이름"),
                        description: cli.minecraft_name_description.or_as_str("마인크래프트 유저 이름을 입력해주세요"),
                    },
                    additional_content: context! {
                        head: cli.additional_content_head.or_as_str(""),
                        after_question: cli.additional_content_after_question.or_as_str(""),
                        before_question: cli.additional_content_before_question.or_as_str(""),
                    }
                },
            )?,
        );
        Ok(())
    }
}
