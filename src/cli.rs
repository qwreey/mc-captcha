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

    /// Site title
    #[clap(long, env)]
    pub title: Option<String>,

    /// Optional questions, format: [{"title":"","description":"","answer_regex":""}, ...] (json)
    #[clap(long, env)]
    pub questions: Option<String>,

    /// Additional content in head element
    #[clap(long, env)]
    pub additional_content_head: Option<String>,

    /// Additional content before question
    #[clap(long, env)]
    pub additional_content_before_question: Option<String>,

    /// Additional content after question
    #[clap(long, env)]
    pub additional_content_after_question: Option<String>,

    /// Translation for hcaptcha title (if enabled)
    #[clap(long, action, env)]
    pub lang_hcaptcha_title: Option<String>,

    /// Translation for minecraft name question title
    #[clap(long, env)]
    pub lang_minecraft_name_title: Option<String>,

    /// Translation for minecraft name question description
    #[clap(long, env)]
    pub lang_minecraft_name_description: Option<String>,

    /// Translation for lang_empty_field
    #[clap(long, env)]
    pub lang_empty_field: Option<String>,

    /// Translation for lang_no_captcha
    #[clap(long, env)]
    pub lang_no_captcha: Option<String>,

    /// Translation for lang_comfirm
    #[clap(long, env)]
    pub lang_comfirm: Option<String>,

    /// Translation for lang_ok
    #[clap(long, env)]
    pub lang_ok: Option<String>,

    /// Translation for lang_dialog_err_title
    #[clap(long, env)]
    pub lang_dialog_err_title: Option<String>,

    /// Translation for lang_dialog_ok_title
    #[clap(long, env)]
    pub lang_dialog_ok_title: Option<String>,
    
    /// Translation for lang_result_already
    #[clap(long, env)]
    pub lang_result_already: Option<String>,

    /// Translation for lang_result_ok
    #[clap(long, env)]
    pub lang_result_ok: Option<String>,

    /// Translation for lang_result_name_err
    #[clap(long, env)]
    pub lang_result_name_err: Option<String>,

    /// Translation for lang_result_answer_err
    #[clap(long, env)]
    pub lang_result_answer_err: Option<String>,

    /// Translation for lang_result_not_exist
    #[clap(long, env)]
    pub lang_result_not_exist: Option<String>,

    /// Translation for lang_result_captcha_err
    #[clap(long, env)]
    pub lang_result_captcha_err: Option<String>,

    /// Translation for lang_result_unknown_err
    #[clap(long, env)]
    pub lang_result_unknown_err: Option<String>,
}
