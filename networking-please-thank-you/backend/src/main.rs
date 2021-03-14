#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_debug_implementations)]
#![feature(const_option)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;

use std::{
    error::Error,
    net::IpAddr,
};

use rocket::config::Environment;

#[cfg(debug_assertions)]
use globals::debug_defaults::*;
#[cfg(not(debug_assertions))]
use globals::release_defaults::*;
use globals::varnames::*;
use please_thank_you as lib;

mod env;
mod globals;

fn rocket(
    ip: IpAddr, port: u16,
) -> Result<rocket::Rocket, rocket::config::ConfigError> {
    #[cfg(debug_assertions)]
    let rocket = {
        let config = rocket::Config::build(Environment::Development)
            .port(port)
            .address(ip.to_string())
            .finalize()?;
        rocket::custom(config)
    };

    #[cfg(not(debug_assertions))]
    let rocket = {
        let config = rocket::Config::build(Environment::Production)
            .port(port)
            .address(ip.to_string())
            .finalize()?;
        rocket::custom(config)
    };

    Ok(rocket.mount("/", routes![lib::success, lib::fail]))
}

fn main() -> Result<(), Box<dyn Error>> {
    let ip = env::get_var(IP_VAR, *DEFAULT_IP)?;
    let port = env::get_var(PORT_VAR, DEFAULT_PORT)?;
    let flag =
        env::get_var(FLAG_VAR, DEFAULT_FLAG.map(|s| s.to_string())).unwrap();
    let accepted_dn =
        env::get_var(ACCEPTED_DN_VAR, DEFAULT_ACCEPTED_DN.map(|s| s.to_string()))
            .unwrap();

    Err(rocket(ip, port)?
        .manage(lib::Flag(flag))
        .manage(lib::AcceptedDn(accepted_dn))
        .launch()
        .into())
}
