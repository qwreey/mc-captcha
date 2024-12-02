use clap::Parser;
use web_backend::WebBackendUserdata;

mod cli;
mod rcon_client;
mod web_backend;

use self::{cli::Cli, rcon_client::RconClient, web_backend::WebBackendBuilder};

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Cli::parse();

    let rcon = RconClient::new(
        args.rcon_host
            .unwrap_or_else(|| String::from("localhost:25575")),
        args.rcon_password,
    )?;

    let web = WebBackendBuilder::new()
        .port(args.port)
        .address(args.address)
        .userdata(WebBackendUserdata { rcon });

    web.build().await?;

    Ok(())
}
