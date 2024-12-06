use qwreey_rocket::{RocketBuild, RouteExport, RouteList, UserdataState};
use qwreey_utility_rs::ErrToString;
use regex::Regex;
use rocket::{
    log::{info_, warn_},
    post, routes,
    serde::{json::Json, Deserialize},
};

use crate::{question::QuestionRegexList, rcon_client::Rcon, HcaptchaVerify, HcaptchaVerifyData};

#[derive(Deserialize)]
struct UserCommitReq {
    minecraft_name: String,
    question_answer: Vec<String>,
    captcha_response: Option<String>,
}
impl HcaptchaVerifyData for Json<UserCommitReq> {
    fn get_user_captcha_response(&self) -> Option<&String> {
        self.captcha_response.as_ref()
    }
}

#[derive(serde::Serialize)]
struct UserCommitRes {
    ok: bool,
    alreay: bool,
    name_err: bool,
    captcha_err: bool,
    answer_err: bool,
    not_exist: bool,
}
impl Default for UserCommitRes {
    fn default() -> Self {
        Self {
            ok: false,
            alreay: false,
            name_err: false,
            captcha_err: false,
            answer_err: false,
            not_exist: false,
        }
    }
}

#[post("/user-commit", data = "<req>")]
async fn handle(userdata: &UserdataState, req: Json<UserCommitReq>) -> Json<UserCommitRes> {
    // Check captcha response
    let hcaptcha = userdata.get_of::<HcaptchaVerify>().unwrap();
    if hcaptcha.enabled {
        match hcaptcha.verify(&req).await {
            Err(err) => {
                warn_!("{err}");
                return Json(UserCommitRes {
                    ok: false,
                    captcha_err: true,
                    ..Default::default()
                });
            }
            Ok(res) => {
                if !res.success() {
                    return Json(UserCommitRes {
                        ok: false,
                        captcha_err: true,
                        ..Default::default()
                    });
                }
            }
        }
    }

    for (index, question_regex) in (&**userdata.get_of::<QuestionRegexList>().unwrap())
        .iter()
        .enumerate()
    {
        match req.question_answer.get(index) {
            Some(answer) => {
                if !question_regex.is_match(answer) {
                    return Json(UserCommitRes {
                        ok: false,
                        answer_err: true,
                        ..Default::default()
                    });
                }
            }
            None => {
                return Json(UserCommitRes {
                    ok: false,
                    answer_err: true,
                    ..Default::default()
                })
            }
        }
    }

    // Check name valid
    let name_len = req.minecraft_name.len();
    if name_len > 16 || name_len < 3 {
        // length err
        return Json(UserCommitRes {
            ok: false,
            name_err: true,
            ..Default::default()
        });
    }
    if !userdata
        .get::<Regex>("minecraft_name_regex")
        .unwrap()
        .is_match(&req.minecraft_name)
    {
        // regex not match
        return Json(UserCommitRes {
            ok: false,
            name_err: true,
            ..Default::default()
        });
    }

    // Execute command
    let rcon_res = match userdata
        .get_of_mut::<Rcon>()
        .unwrap()
        .execute(format!("whitelist add {}", req.minecraft_name))
    {
        Ok(res) => res,
        Err(err) => {
            warn_!("rcon execute failed: {err}");
            return Json(UserCommitRes {
                ok: false,
                ..Default::default()
            });
        }
    };

    // Already whitelisted
    if rcon_res == "Player is already whitelisted" {
        return Json(UserCommitRes {
            ok: true,
            alreay: true,
            ..Default::default()
        });
    }

    // Not exist
    if rcon_res == "That player does not exist" {
        return Json(UserCommitRes {
            ok: false,
            not_exist: true,
            ..Default::default()
        });
    }

    info_!("{rcon_res}");
    Json(UserCommitRes {
        ok: true,
        ..Default::default()
    })
}

pub struct UserCommit;
impl RouteExport for UserCommit {
    fn build(
        &self,
        rocket: RocketBuild,
        userdata: qwreey_utility_rs::ArcRwUserdata,
    ) -> Result<RocketBuild, String> {
        userdata.insert(
            "minecraft_name_regex",
            Regex::new("^[0-9a-zA-Z_]+$").err_tostring()?,
        );
        Ok(rocket)
    }
    fn routes(&self) -> RouteList {
        routes![handle]
    }
}
