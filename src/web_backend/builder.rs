use std::{collections::HashMap, net::IpAddr};

use qwreey_utility_rs::ErrToString;
use rocket::{config::Ident, Config, Ignite, Rocket, State};
use rocket_dyn_templates::{tera::Tera, Template};

use super::*;

pub struct RouteExport {
    pub routes: Vec<Route>,
    pub init: Option<fn(userdata: &mut WebBackendUserdata)>,
    pub render: Option<fn(tera: &mut Tera, userdata: &mut WebBackendUserdata)>,
}

pub struct WebBackendBuilder {
    routes: Vec<RouteExport>,
    port: u16,
    address: IpAddr,
    userdata: Option<WebBackendUserdata>,
}
impl Default for WebBackendBuilder {
    fn default() -> Self {
        Self {
            routes: Vec::<RouteExport>::new(),
            port: 80,
            address: "127.0.0.1".parse::<IpAddr>().unwrap(),
            userdata: None,
        }
    }
}
impl WebBackendBuilder {
    pub async fn build(self) -> Result<Rocket<Ignite>, String> {
        let mut userdata = self.userdata.unwrap();
        let userdata_borrow = &mut userdata;

        let template_fairing = Template::custom(move |engines| {
            engines.tera.register_function(
                "compactive_js",
                |_value: &std::collections::HashMap<
                    std::string::String,
                    rocket_dyn_templates::tera::Value,
                >|
                 -> Result<
                    rocket_dyn_templates::tera::Value,
                    rocket_dyn_templates::tera::Error,
                > {
                    Ok(rocket_dyn_templates::tera::Value::String(String::from(
                        include_str!("../../compactive-js/compactive.min.js"),
                    )))
                },
            );
            for route in &self.routes {
                if let Some(render) = route.render {
                    render(&mut engines.tera, userdata_borrow)
                }
            }
            engines.tera.autoescape_on(vec![]);
        });

        let rocket = rocket::build()
            .configure(Config {
                cli_colors: false,
                ident: Ident::try_new("MC-Captcha").unwrap(),
                port: self.port,
                address: self.address,
                ..Default::default()
            })
            .mount("/", ROUTES.clone())
            .manage(userdata)
            .attach(template_fairing)
            .launch()
            .await
            .err_tostring()?;
        Ok(rocket)
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn port(mut self, port: Option<u16>) -> Self {
        self.port = port.unwrap_or(Self::default().port);
        self
    }
    pub fn address(mut self, address: Option<IpAddr>) -> Self {
        self.address = address.unwrap_or(Self::default().address);
        self
    }
    pub fn userdata(mut self, userdata: WebBackendUserdata) -> Self {
        self.userdata = Some(userdata);
        self
    }
    pub fn add_route(mut self, route: RouteExport) -> Self {
        self.routes.push(route);
        self
    }
}
