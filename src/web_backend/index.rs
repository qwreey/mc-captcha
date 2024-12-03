use rocket::{get, http::ContentType, response::Responder, Response, State};
use rocket_dyn_templates::{context, Template};

use super::WebBackendUserdata;

// #[inline]
// pub fn init(userdata: &State<WebBackendUserdata>) -> String {
//     if userdata.caches.contains_key("index") {
//         userdata.caches.get("index")
//     } else {
//         userdata.caches.
//     }
// }

#[get("/")]
pub fn handle(userdata: &State<WebBackendUserdata>) -> Template {
    // Response::build().streamed_body("asdfasdf").status(200).header(ContentType::HTML).finalize()

    Template::render(
        "index",
        context! {
            site: context! {
                title: &userdata.title,
            },
        },
    )
}
