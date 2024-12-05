use qwreey_rocket::{RouteExport, RouteList, UserdataState};
use rocket::{
    log::warn_,
    post, routes,
    serde::{json::Json, Deserialize},
};

use crate::{HcaptchaVerify, HcaptchaVerifyData};

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
}

#[post("/commit-user", data = "<req>")]
async fn handle(userdata: &UserdataState, req: Json<UserCommitReq>) -> Json<UserCommitRes> {
    let hcaptcha = userdata.get_of::<HcaptchaVerify>().unwrap();
    if hcaptcha.enabled {
        match hcaptcha.verify(&req).await {
            Err(err) => {
                warn_!("{err}");
                return Json(UserCommitRes { ok: false });
            }
            Ok(res) => {
                if !res.success() {
                    return Json(UserCommitRes { ok: false });
                }
            }
        }
    }

    Json(UserCommitRes { ok: true })
}

pub struct UserCommit;
impl RouteExport for UserCommit {
    fn routes(&self) -> RouteList {
        routes![handle]
    }
}
