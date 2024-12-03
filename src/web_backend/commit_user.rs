use std::sync::LazyLock;

use rocket::{
    post, routes,
    serde::{json::Json, Deserialize},
    Route, State,
};

use super::WebBackendUserdata;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCommitReq {
    minecraft_name: String,
    question_answer: Vec<String>,
}

#[post("/commit-user", data = "<req>")]
pub fn handle(userdata: &State<WebBackendUserdata>, req: Json<UserCommitReq>) -> String {
    format!("hello {}", req.minecraft_name)
}

static Route: LazyLock<RouteExport> = LazyLock::new(|| RouteExport {
    routes: routes![handle],
    init: |userdata| {},
});
