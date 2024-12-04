use clap::Parser;
use qwreey_rocket::WebBackendBuilder;

mod cli;
mod web;
mod hcaptcha_verify;
mod rcon_client;

use qwreey_utility_rs::ArcRwUserdata;
use web::export_all;
use cli::Cli;
use hcaptcha_verify::HcaptchaVerify;
use rcon_client::Rcon;

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Cli::parse();

    let userdata = ArcRwUserdata::new();

    userdata.insert_of(args.clone());
    userdata.insert_of(Rcon::new(
        args.rcon_host
            .unwrap_or_else(|| String::from("localhost:25575")),
        args.rcon_password,
    )?);
    userdata.insert_of(HcaptchaVerify::new(
        args.hcaptcha_secret,
        args.hcaptcha_sitekey,
    ));

    let web = WebBackendBuilder::new()
        .port(args.port)
        .bind(args.bind)
        .userdata(userdata)
        .add_export_many(export_all());

    web.build().await?;

    Ok(())
}
