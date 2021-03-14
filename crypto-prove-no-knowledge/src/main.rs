#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_debug_implementations)]
#![feature(const_option)]

use std::{
    error::Error,
    net::{
        IpAddr,
        SocketAddr,
    },
};
#[macro_use]
extern crate lazy_static;
#[cfg(debug_assertions)]
use globals::debug_defaults::*;
#[cfg(not(debug_assertions))]
use globals::release_defaults::*;
use globals::varnames::*;
use prove_no_knowledge as lib;

use tokio::{
    io::{
        AsyncReadExt,
        AsyncWriteExt,
    },
    net::{
        TcpListener,
        TcpStream,
    },
    time::Duration,
};

mod env;
mod globals;

fn get_env_addr(
    ip_var: &'static str, default_ip: Option<IpAddr>, port_var: &'static str,
    default_port: Option<u16>,
) -> Result<SocketAddr, Box<dyn Error>> {
    let ip = env::get_var(ip_var, default_ip)?;
    let port = env::get_var(port_var, default_port)?;

    Ok(SocketAddr::new(ip, port))
}

fn is_healthcheck() -> bool {
    let args = std::env::args().collect::<Vec<_>>();

    args.len() == 2 && args[1] == "HEALTHCHECK"
}

fn healthcheck_success() {
    std::process::exit(0)
}

fn healthcheck_fail() {
    std::process::exit(1)
}

async fn healthcheck(addr: SocketAddr) {
    let mut stream = TcpStream::connect(addr).await.unwrap();

    if let Err(err) = stream.write_all(b"HEALTHCHECK").await {
        eprintln!("HEALTHCHECK FAILED: {}", err);
        healthcheck_fail()
    }

    let mut buf = [0u8];
    if let Err(err) = stream.read(&mut buf).await {
        eprintln!("HEALTHCHECK FAILED: {}", err);
        healthcheck_fail()
    }

    healthcheck_success()
}

async fn program(addr: SocketAddr) {
    let socket = TcpListener::bind(addr).await.unwrap();
    let timeout = Duration::from_secs(
        env::get_var(TIMEOUT_VAR, DEFAULT_TIMEOUT).unwrap().get(),
    );
    let flag =
        env::get_var(FLAG_VAR, DEFAULT_FLAG.map(|s| s.to_string())).unwrap();

    loop {
        let (stream, _) = socket.accept().await.unwrap();

        let flag = flag.clone();
        tokio::spawn(async move {
            if let Err(_err) = lib::process(stream, timeout, flag).await {
                #[cfg(debug_assertions)]
                eprintln!("Error in stream (line: {}): {}", line!(), _err);
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let addr = get_env_addr(IP_VAR, *DEFAULT_IP, PORT_VAR, DEFAULT_PORT).unwrap();

    if is_healthcheck() {
        healthcheck(addr).await
    } else {
        program(addr).await
    }
}
