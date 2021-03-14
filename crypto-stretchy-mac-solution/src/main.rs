#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_debug_implementations)]
#![feature(const_option)]

extern crate stretchy_mac_solution as lib;

use std::error::Error;

use lib::{
    sha224_extended_hash,
    sha224_length_extension_attack,
};
use std::{
    collections::HashMap,
    convert::TryInto,
};
use tokio::{
    io::{
        AsyncBufReadExt,
        AsyncWriteExt,
        BufReader,
    },
    net::TcpStream,
};

async fn weak_mac<S: AsyncBufReadExt + AsyncWriteExt + Unpin>(
    stream: &mut S, m: &[u8], read_buf: &mut String,
) -> Result<[u8; 224 / 8], Box<dyn Error>> {
    stream
        .write_all(format!("{}\n", hex::encode(m)).as_bytes())
        .await?;

    stream.read_line(&mut String::new()).await?;
    read_buf.clear();
    let len = stream.read_line(read_buf).await?;
    let sig = &read_buf[..len]
        .trim()
        .split_whitespace()
        .nth(3)
        .ok_or_else::<tokio::io::Error, _>(|| {
        tokio::io::ErrorKind::InvalidInput.into()
    })?[..224 / 8 * 2];

    Ok(hex::decode(sig)?
        .try_into()
        .map_err::<tokio::io::Error, _>(|_| {
            tokio::io::ErrorKind::InvalidInput.into()
        })?)
}

fn pad(m: &mut Vec<u8>, pre_length: usize) {
    let len = m.len() + pre_length;
    let zeros = (64 - ((9 + len) % 64)) % 64;

    m.push(0x80);
    m.extend_from_slice(&vec![0; zeros]);
    m.extend_from_slice(&((len * 8) as u64).to_be_bytes());
}

async fn init<S: AsyncBufReadExt + AsyncWriteExt + Unpin>(
    stream: &mut S, read_buf: &mut String,
) -> Result<Vec<u8>, Box<dyn Error>> {
    stream.read_line(&mut String::new()).await?;

    read_buf.clear();
    let len = stream.read_line(read_buf).await?;
    let challenge = read_buf[..len].trim().split_whitespace().nth(8);

    let challenge = challenge.ok_or_else::<tokio::io::Error, _>(|| {
        tokio::io::ErrorKind::InvalidInput.into()
    })?;

    Ok(hex::decode(challenge)?)
}

async fn finish<S: AsyncBufReadExt + AsyncWriteExt + Unpin>(
    stream: &mut S, m: &[u8], sig: &[u8], read_buf: &mut String,
) -> Result<String, Box<dyn Error>> {
    stream
        .write_all(format!("{}\n", hex::encode(m)).as_bytes())
        .await?;
    stream
        .write_all(format!("{}\n", hex::encode(sig)).as_bytes())
        .await?;

    stream.read_line(&mut String::new()).await?;
    stream.read_line(&mut String::new()).await?;
    read_buf.clear();
    let len = stream.read_line(read_buf).await?;

    Ok(read_buf[..len].trim().to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let read_buf = &mut String::new();
    let mut stream =
        TcpStream::connect(&std::env::args().nth(1).unwrap()[..]).await?;
    let stream = &mut BufReader::new(&mut stream);

    let challenge = init(stream, read_buf).await?;

    let initial_message = b"".to_vec();
    let initial_state = weak_mac(stream, &initial_message, read_buf).await?;

    let prior_blocks = 1usize;
    let min_key_length =
        (prior_blocks.checked_sub(1).expect("Prior blocks too low.") * 64 + 1)
            .saturating_sub(initial_message.len() + 9);
    let max_key_length = (prior_blocks * 64)
        .checked_sub(initial_message.len() + 9)
        .expect("Message too large for prior_blocks.");

    let mut possible_hashes_map = HashMap::with_capacity(512 / 8);
    for key_len in min_key_length..=max_key_length {
        let mut m = initial_message.clone();
        pad(&mut m, key_len);
        possible_hashes_map
            .insert(weak_mac(stream, &m, read_buf).await?, key_len);
    }

    let mut possible_hashes: Vec<_> =
        possible_hashes_map.keys().copied().collect();

    let (matched_hash, exposed_state) = sha224_length_extension_attack(
        &mut possible_hashes,
        &initial_state,
        prior_blocks,
        b"",
    )
    .expect("HASH_NOT_FOUND");

    let key_len = *possible_hashes_map.get(&matched_hash).unwrap();

    let mut m = initial_message;
    pad(&mut m, key_len);
    m.extend_from_slice(&challenge);
    let sig = sha224_extended_hash(
        &initial_state,
        exposed_state,
        prior_blocks,
        &challenge,
    );

    #[cfg(debug_assertions)]
    {
        eprintln!("Message:       {:02X?}", m);
        eprintln!("Predicted MAC: {:02X?}", sig);
    }

    println!("{}", (finish(stream, &m, &sig, read_buf).await?));

    Ok(())
}
