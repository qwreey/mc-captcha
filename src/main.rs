use clap::Parser;
use qwreey_rocket::{RouteExport, TeraError, TeraValue, WebBackendBuilder};

mod cli;
mod hcaptcha_verify;
mod question;
mod rcon_client;
mod web;
mod lang;

use cli::Cli;
use hcaptcha_verify::{HcaptchaInit, HcaptchaVerify, HcaptchaVerifyData};
use question::{QuestionInit, QuestionList};
use qwreey_utility_rs::ArcRwUserdata;
use rcon_client::RconInit;
use web::export_all;

struct Main;
impl RouteExport for Main {
    fn tera(&self, tera: &mut rocket_dyn_templates::tera::Tera, _userdata: ArcRwUserdata) {
        tera.register_function(
            "compactive_js",
            |_value: &std::collections::HashMap<
                std::string::String,
                TeraValue,
            >|
             -> Result<
                TeraValue,
                TeraError,
            > {
                Ok(TeraValue::String(String::from(
                    include_str!("../compactive-js/compactive.min.js"),
                )))
            },
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let userdata = ArcRwUserdata::new();
    let args = Cli::parse();
    userdata.insert_of(args.clone());

    let web = WebBackendBuilder::new()
        .port(args.port)
        .bind(args.bind)
        .userdata(userdata)
        .add_export(Main)
        .add_export(QuestionInit)
        .add_export(RconInit)
        .add_export(HcaptchaInit)
        .add_export_many(export_all());
    web.build().await?;

    Ok(())
}
