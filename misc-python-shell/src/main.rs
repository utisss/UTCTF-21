#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_debug_implementations)]

use python_shell as lib;
use std::error::Error;

mod env;
mod globals;

use crate::env::get_var;
#[cfg(debug_assertions)]
use globals::debug_defaults::*;
#[cfg(not(debug_assertions))]
use globals::release_defaults::*;
use globals::varnames::*;
use std::net::{
    IpAddr,
    SocketAddr,
};
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
    let timeout =
        Duration::from_secs(get_var(TIMEOUT_VAR, DEFAULT_TIMEOUT).unwrap().get());
    let upload_limit = get_var(UPLOAD_LIMIT_VAR, DEFAULT_UPLOAD_LIMIT).unwrap();

    loop {
        let (stream, _) = socket.accept().await.unwrap();

        tokio::spawn(async move {
            if let Err(_err) = lib::process(stream, timeout, upload_limit).await {
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
