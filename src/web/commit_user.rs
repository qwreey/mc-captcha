use qwreey_rocket::{RouteExport, RouteList, UserdataState};
use rocket::{
    post, routes,
    serde::{json::Json, Deserialize},
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserCommitReq {
    minecraft_name: String,
    question_answer: Vec<String>,
}

#[post("/commit-user", data = "<req>")]
fn handle(userdata: &UserdataState, req: Json<UserCommitReq>) -> Result<String, String> {
    // userdata.read().err_tostring()?;
    // userdata.
    Ok(format!("hello {}", req.minecraft_name))
}

pub struct UserCommit;

impl RouteExport for UserCommit {
    fn routes(&self) -> RouteList {
        routes![handle]
    }
}
