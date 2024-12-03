use std::sync::LazyLock;

use dashmap::DashMap;
use rocket::{routes, Route};

use crate::rcon_client::RconClient;

mod builder;
pub use builder::WebBackendBuilder;

mod commit_user;
mod index;
pub static ROUTES: LazyLock<Vec<Route>> =
    LazyLock::new(|| routes![index::handle, commit_user::handle]);

pub struct WebBackendUserdata {
    pub rcon: RconClient,
    pub title: String,
}
