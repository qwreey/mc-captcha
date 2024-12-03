use std::net::IpAddr;

use clap::{command, Parser};

#[derive(Parser, Debug, Default, Clone)]
#[command(version, about, long_about = None)]
/// MC Captcha backend
pub struct Cli {
    // Web ui expose port, default: 80
    #[clap(short, long, action, env)]
    pub port: Option<u16>,

    // Web ui expose address, defualt: localhost
    #[clap(long, action, env)]
    pub address: Option<IpAddr>,

    /// Rcon password to use
    #[clap(long, action, env)]
    pub rcon_password: String,

    /// Rcon hostname to connect, default: localhost
    #[clap(long, action, env)]
    pub rcon_host: Option<String>,

    /// Site title
    #[clap(long, env)]
    pub title: Option<String>,
}
