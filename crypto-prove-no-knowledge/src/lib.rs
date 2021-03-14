#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_debug_implementations)]
#![feature(const_option)]

use std::error::Error;

#[macro_use]
extern crate lazy_static;
use num_bigint::{
    BigUint,
    RandBigInt,
};
use rand::rngs::OsRng;
use tokio::{
    io::{
        AsyncBufReadExt,
        AsyncReadExt,
        AsyncWriteExt,
        BufReader,
    },
    net::TcpStream,
    time,
    time::Duration,
};

#[cfg(debug_assertions)]
use globals::debug_defaults::*;
#[cfg(not(debug_assertions))]
use globals::release_defaults::*;
use globals::varnames::*;

mod env;
mod globals;
lazy_static! {
    static ref ONE: BigUint = BigUint::parse_bytes(b"1", 16).unwrap();
    static ref G: BigUint = BigUint::parse_bytes(
        env::get_var(G_VAR, DEFAULT_G.map(|v| v.to_string()))
            .unwrap()
            .as_bytes(),
        16
    )
    .unwrap();
    static ref P: BigUint = BigUint::parse_bytes(
        env::get_var(P_VAR, DEFAULT_P.map(|v| v.to_string()))
            .unwrap()
            .as_bytes(),
        16
    )
    .unwrap();
}

fn rand() -> BigUint {
    OsRng.gen_biguint_below(&P)
}

fn exp(val: &BigUint) -> BigUint {
    G.modpow(val, &P)
}

async fn read_uint<S: Unpin + AsyncReadExt + AsyncWriteExt>(
    stream: &mut BufReader<S>, timeout: Duration, buf: &mut String,
) -> Result<Option<BigUint>, Box<dyn Error>> {
    buf.clear();

    Ok(match time::timeout(timeout, stream.read_line(buf)).await? {
        Ok(len) => BigUint::parse_bytes(&buf[..len].trim().as_bytes(), 10),
        Err(err) =>
            if err.kind() == tokio::io::ErrorKind::InvalidInput {
                None
            } else {
                return Err(err.into());
            },
    })
}

async fn read_write_uint_round<S: Unpin + AsyncReadExt + AsyncWriteExt>(
    stream: &mut BufReader<S>, timeout: Duration, message: &str, buf: &mut String,
) -> Result<BigUint, Box<dyn Error>> {
    time::timeout(timeout, stream.write_all(message.as_bytes())).await??;

    let mut uint = read_uint(stream, timeout, buf).await?;

    while uint.is_none() {
        time::timeout(timeout, stream.write_all(b"BadInput\n")).await??;
        time::timeout(timeout, stream.write_all(message.as_bytes())).await??;

        uint = read_uint(stream, timeout, buf).await?;
    }

    Ok(uint.unwrap())
}

async fn test_a<S: Unpin + AsyncReadExt + AsyncWriteExt>(
    stream: &mut BufReader<S>, timeout: Duration, buf: &mut String,
) -> Result<bool, Box<dyn Error>> {
    let c = read_write_uint_round(
        stream,
        timeout,
        "Pick a random r. Send g^r mod p.\n",
        buf,
    )
    .await?;

    let r = read_write_uint_round(stream, timeout, "Send r.\n", buf).await?;

    Ok(exp(&r) == c)
}

async fn test_b<S: Unpin + AsyncReadExt + AsyncWriteExt>(
    stream: &mut BufReader<S>, timeout: Duration, buf: &mut String, y: &BigUint,
) -> Result<bool, Box<dyn Error>> {
    let c = read_write_uint_round(
        stream,
        timeout,
        "Pick a random r. Send g^r mod p.\n",
        buf,
    )
    .await?;

    let rp = read_write_uint_round(
        stream,
        timeout,
        "Send (x + r) mod (p - 1).\n",
        buf,
    )
    .await?;

    Ok(exp(&rp) == (c * y).modpow(&ONE, &P))
}

pub async fn process(
    stream: TcpStream, timeout: Duration, flag: String,
) -> Result<(), Box<dyn Error>> {
    let mut stream = BufReader::new(stream);
    let mut buf = String::new();

    let x = rand();
    #[cfg(debug_assertions)]
    eprintln!("x: {}", x);
    let y = exp(&x);
    drop(x);

    time::timeout(
        timeout,
        stream
            .write_all(b"Please authenticate with the service for 256 rounds\n"),
    )
    .await??;
    time::timeout(
        timeout,
        stream.write_all(b"Prove knowledge of x such that g^x mod p = y\n"),
    )
    .await??;
    time::timeout(
        timeout,
        stream.write_all(format!("g: {}\np: {}\ny: {}\n", *G, *P, y).as_bytes()),
    )
    .await??;

    for _ in 0..128 {
        if !test_a(&mut stream, timeout, &mut buf).await? {
            time::timeout(timeout, stream.write_all(b"Authentication failed!\n"))
                .await??;

            return Ok(());
        }

        if !test_b(&mut stream, timeout, &mut buf, &y).await? {
            time::timeout(timeout, stream.write_all(b"Authentication failed!\n"))
                .await??;

            return Ok(());
        }
    }

    time::timeout(timeout, stream.write_all(b"Authentication succeeded!\n"))
        .await??;

    time::timeout(timeout, stream.write_all(format!("{}\n", flag).as_bytes()))
        .await??;

    Ok(())
}
