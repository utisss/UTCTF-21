#![feature(proc_macro_hygiene, decl_macro)]
#![feature(num_as_ne_bytes)]
#![deny(missing_debug_implementations)]

use std::error::Error;

use aes::Aes256;
use block_modes::{
    block_padding::Pkcs7,
    BlockMode,
    Cbc,
};
use so::randombytes;
use sodiumoxide as so;
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

const KEY_BYTES: usize = 32;
const BLOCK_SIZE: usize = 16;

type Key = [u8; KEY_BYTES];

type Aes = Cbc<Aes256, Pkcs7>;

pub fn init_sodium_oxide() {
    so::init().expect("Sodiumoxide library failed to initialize.");
}

fn random_iv() -> u128 {
    init_sodium_oxide();

    let mut iv = [0; 16];

    randombytes::randombytes_into(&mut iv);

    u128::from_le_bytes(iv)
}

fn random_aes_key() -> Key {
    init_sodium_oxide();

    let mut key = [0; KEY_BYTES];
    randombytes::randombytes_into(&mut key);

    key
}

#[allow(clippy::needless_lifetimes)]
async fn read_line<'s, S: Unpin + AsyncReadExt + AsyncWriteExt>(
    stream: &mut BufReader<S>, timeout: Duration, buf: &'s mut String,
) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    buf.clear();

    Ok(match time::timeout(timeout, stream.read_line(buf)).await? {
        Ok(len) => Some(buf[..len].trim()),
        Err(err) =>
            if err.kind() == tokio::io::ErrorKind::InvalidInput {
                None
            } else {
                return Err(err.into());
            },
    }
    .and_then(|dat| so::hex::decode(dat).ok()))
}

pub async fn process(
    stream: TcpStream, timeout: Duration, flag: String,
) -> Result<(), Box<dyn Error>> {
    let mut stream = BufReader::new(stream);
    let mut buf = String::new();
    let key = &random_aes_key()[..];
    let mut iv = random_iv();

    loop {
        let mut input = read_line(&mut stream, timeout, &mut buf).await?;
        while input.is_none() {
            time::timeout(timeout, stream.write_all(b"Invalid hex encoding\n"))
                .await??;

            input = read_line(&mut stream, timeout, &mut buf).await?;
        }

        let input = input.unwrap();
        let cipher = Aes::new_var(key, iv.as_ne_bytes()).unwrap();
        let padding = BLOCK_SIZE - ((input.len() + flag.len()) % BLOCK_SIZE);

        let mut bytes =
            Vec::with_capacity(BLOCK_SIZE + input.len() + flag.len() + padding);
        bytes.extend_from_slice(iv.as_ne_bytes());
        bytes.extend_from_slice(&input);
        bytes.extend_from_slice(flag.as_bytes());
        bytes.extend_from_slice(&vec![0; padding]);
        let pos = bytes[BLOCK_SIZE..].len() - padding;
        cipher.encrypt(&mut bytes[BLOCK_SIZE..], pos).unwrap();

        let hex = so::hex::encode(&bytes);

        time::timeout(timeout, stream.write_all(format!("{}\n", hex).as_bytes()))
            .await??;

        let (iv_new, _) = iv.overflowing_add(1);
        iv = iv_new;
    }
}
