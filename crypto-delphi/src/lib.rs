#![feature(proc_macro_hygiene, decl_macro)]
#![feature(num_as_ne_bytes)]
#![deny(missing_debug_implementations)]

use std::{
    error::Error,
    fmt::{
        self,
        Display,
        Formatter,
    },
};

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
        AsyncRead,
        AsyncWriteExt,
        BufReader,
    },
    net::TcpStream,
    time,
    time::{
        Duration,
        Instant,
    },
};

pub use config::{
    Config,
    ConfigBuilder,
};
use sodiumoxide::hex;

type Aes = Cbc<Aes256, Pkcs7>;

const KEY_BYTES: usize = 32;
const BLOCK_SIZE: usize = 16;
const TRIES_PER_BYTE: usize = (u8::MAX as usize) + 1;

type Key = [u8; KEY_BYTES];

pub mod config;
pub mod env;
pub mod errors;

pub fn init_sodium_oxide() {
    so::init().expect("Sodiumoxide library failed to initialize.");
}

async fn read_line<T>(
    stream: &mut T, buf: &mut String, config: &Config,
) -> Result<usize, tokio::io::Error>
where
    T: AsyncBufReadExt + Unpin,
{
    buf.clear();
    Ok(time::timeout(
        Duration::from_secs(config.read_timeout()),
        stream.read_line(buf),
    )
    .await??)
}

async fn random_aes_key() -> Key {
    init_sodium_oxide();

    let mut key = [0; KEY_BYTES];
    randombytes::randombytes_into(&mut key);

    key
}

async fn random_challenge(config: &Config) -> Vec<u8> {
    init_sodium_oxide();

    let mut challenge = vec![0; config.challenge_bytes()];
    randombytes::randombytes_into(&mut challenge);

    challenge
}

#[derive(Debug)]
enum Unauthorized {
    InvalidChallenge,
    DecryptionFailed,
}

impl Display for Unauthorized {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChallenge => write!(f, "Invalid challenge provided."),
            Self::DecryptionFailed => write!(f, "Decryption failed."),
        }
    }
}

fn constant_time_byte_slice_compare(truth: &[u8], untrusted: &[u8]) -> bool {
	truth.len() == untrusted.len() && so::utils::memcmp(truth, untrusted)
}

impl Error for Unauthorized {}

async fn validate_input<T>(
    stream: &mut BufReader<T>, config: &Config, line: &mut String, key: &[u8],
    challenge: &[u8],
) -> Result<(), Unauthorized>
where
    T: AsyncRead + Unpin + AsyncWriteExt,
{
    let len = read_line(stream, line, config)
        .await
        .map_err(|_| Unauthorized::DecryptionFailed)?;

    let mut data = match hex::decode(&line[..len].trim_end()) {
        Ok(data) => data,
        Err(_) => {
            return Err(Unauthorized::DecryptionFailed);
        },
    };

    if data.len() < BLOCK_SIZE {
        return Err(Unauthorized::DecryptionFailed);
    }

    let cipher = Aes::new_var(key, &data[..BLOCK_SIZE])
        .map_err(|_| Unauthorized::DecryptionFailed)?;
    let plaintext = cipher
        .decrypt(&mut data[BLOCK_SIZE..])
        .map_err(|_| Unauthorized::DecryptionFailed)?;

    if !constant_time_byte_slice_compare(challenge, plaintext) {
        return Err(Unauthorized::InvalidChallenge);
    }

    Ok(())
}

async fn get_flag<T>(
    stream: &mut BufReader<T>, config: &Config, mut tries: usize, key: &[u8],
    challenge: &[u8], timeout: &Instant,
) -> Result<(), std::io::Error>
where
    T: AsyncRead + Unpin + AsyncWriteExt,
{
    let mut line = String::new();

    loop {
        if tries == 0 {
            break;
        }

        stream
            .write_all(
                format!(
                    "{} seconds remain. {} tries left.\n",
                    seconds_left(timeout),
                    tries
                )
                .as_bytes(),
            )
            .await?;
        stream
            .write_all(b"Please submit authorization token.\n")
            .await?;

        match validate_input(stream, config, &mut line, key, challenge).await {
            Ok(_) => {
                stream
                    .write_all(
                        format!("The flag is {}\n", config.flag()).as_bytes(),
                    )
                    .await?;
                break;
            },
            Err(err) => {
                stream.write_all(format!("{}\n\n", err).as_bytes()).await?;
            },
        }

        tries -= 1;
    }

    Ok(())
}

fn seconds_left(instant: &Instant) -> u64 {
    instant.saturating_duration_since(Instant::now()).as_secs()
}

async fn internal_process<T>(
    stream: &mut BufReader<T>, config: &Config,
) -> Result<(), Box<dyn Error>>
where
    T: AsyncRead + Unpin + AsyncWriteExt,
{
    let key = &random_aes_key().await[..];
    #[cfg(debug_assertions)]
    eprintln!("Key: {}", hex::encode(key));
    let challenge = &random_challenge(config).await[..];
    let tries = (config.challenge_bytes() +
        (16 - (config.challenge_bytes() % BLOCK_SIZE))) *
        TRIES_PER_BYTE +
        1;

    stream
        .write_all(b"Welcome to the secure flag vault.\n")
        .await?;
    stream
        .write_all(b"Only authorized users can retrieve the flag.\n")
        .await?;
    stream
		.write_all(b"Just encrypt the following challenge in bytes with the secret key.\n")
		.await?;
    stream
		.write_all(b"Use AES-256-CBC with PKCS7 padding. The first 16 bytes should be the IV.\n")
		.await?;
    stream
        .write_all(b"Submit it in hex encoded form.\n")
        .await?;
    stream
        .write_all(
            format!(
                "For security reasons, only {} tries are permitted.\n",
                tries
            )
            .as_bytes(),
        )
        .await?;
    stream
        .write_all(format!("Challenge: {}\n", hex::encode(challenge)).as_bytes())
        .await?;

    stream.write_all(b"\n").await?;

    time::sleep(Duration::from_secs(2)).await;
    let timeout = Instant::now() + Duration::from_secs(config.game_timeout());
    stream
        .write_all(b"Uh-oh. Someone is trying to shut down the system.\n")
        .await?;
    stream
        .write_all(
            format!(
                "The connection will close in {} seconds.\n",
                seconds_left(&timeout)
            )
            .as_bytes(),
        )
        .await?;

    stream.write_all(b"\n").await?;

    match time::timeout_at(
        timeout,
        get_flag(stream, config, tries, key, challenge, &timeout),
    )
    .await
    {
        Ok(ok) => match ok {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        },
        Err(err) => {
            stream.write_all(b"Connection has been closed.\n").await?;
            Err(err.into())
        },
    }
}

pub async fn process(stream: TcpStream, config: &Config) {
    let mut stream = BufReader::new(stream);

    if let Err(_err) = internal_process(&mut stream, config).await {
        #[cfg(debug_assertions)]
        eprintln!("{}", _err);
    }
}
