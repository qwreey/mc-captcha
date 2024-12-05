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
    pub bind: Option<IpAddr>,

    /// Rcon password to use
    #[clap(long, action, env)]
    pub rcon_password: String,

    /// Rcon hostname to connect, default: localhost
    #[clap(long, action, env)]
    pub rcon_host: Option<String>,

    /// Hcaptcha sitekey if enabled
    #[clap(long, action, env)]
    pub hcaptcha_sitekey: Option<String>,

    /// Hcaptcha secret if enabled
    #[clap(long, action, env)]
    pub hcaptcha_secret: Option<String>,

    /// Hcaptcha title if enabled
    #[clap(long, action, env)]
    pub hcaptcha_title: Option<String>,

    /// Site title
    #[clap(long, env)]
    pub title: Option<String>,

    /// Optional questions, format: [{"title":"","description":"","answer_regex":""}, ...] (json)
    #[clap(long, env)]
    pub questions: Option<String>,

    /// Minecraft name question title
    #[clap(long, env)]
    pub minecraft_name_title: Option<String>,

    /// Minecraft name question titledescription
    #[clap(long, env)]
    pub minecraft_name_description: Option<String>,

    /// Additional content in head element
    #[clap(long, env)]
    pub additional_content_head: Option<String>,

    /// Additional content before question
    #[clap(long, env)]
    pub additional_content_before_question: Option<String>,

    /// Additional content after question
    #[clap(long, env)]
    pub additional_content_after_question: Option<String>,
}
