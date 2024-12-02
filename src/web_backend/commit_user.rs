use rocket::{
    post,
    serde::{json::Json, Deserialize},
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCommitReq {
    minecraft_name: String,
    question_answer: Vec<String>,
}

#[post("/commit-user", data = "<req>")]
pub fn handle(req: Json<UserCommitReq>) -> String {
    format!("hello {}", req.minecraft_name)
}
