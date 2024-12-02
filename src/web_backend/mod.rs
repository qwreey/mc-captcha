use std::sync::LazyLock;

use rocket::{routes, Route};

use crate::rcon_client::RconClient;

mod builder;
pub use builder::WebBackendBuilder;

mod commit_user;
pub static ROUTES: LazyLock<Vec<Route>> = LazyLock::new(|| routes![commit_user::handle]);

pub struct WebBackendUserdata {
    pub rcon: RconClient,
}
