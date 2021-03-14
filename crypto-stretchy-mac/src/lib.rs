#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_debug_implementations)]

pub async fn process(
    stream: TcpStream, timeout: Duration, flag: String, minimum_key_size: u8,
    maximum_key_size: u8,
) -> Result<(), Box<dyn Error>> {
    let mut stream = BufReader::new(stream);

    let config = config::ConfigBuilder::new()
        .set_flag(&flag)
        .set_read_timeout(timeout.as_secs())
        .set_minimum_key_size(minimum_key_size)
        .set_maximum_key_size(maximum_key_size)
        .finalize()
        .unwrap();

    internal_process(&mut stream, &config).await
}

use std::error::Error;

use crypto::{
    digest::Digest,
    sha2::Sha224,
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
    time::{
        self,
        Duration,
    },
};

use crate::config::Config;
use sodiumoxide::hex;

pub mod config;
pub mod env;
pub mod errors;

const CHALLENGE_BYTES: usize = 16;

fn gen_key(config: &Config<'_>) -> Vec<u8> {
    so::init().unwrap();

    let key_size = randombytes::randombytes_uniform(
        (1 + (config.maximum_key_size() - config.minimum_key_size())) as u32,
    ) as usize;
    let mut key = vec![0; config.minimum_key_size() as usize + key_size];
    randombytes::randombytes_into(&mut key);
    key
}

fn gen_challenge(_config: &Config<'_>) -> Vec<u8> {
    so::init().unwrap();

    let mut challenge = vec![0; CHALLENGE_BYTES];
    randombytes::randombytes_into(&mut challenge);
    challenge
}

// I think there's a bug in Clippy
// Lifetimes need to be explicit here
#[allow(clippy::needless_lifetimes)]
async fn trimmed_read_line<'a, T>(
    stream: &mut T, buf: &'a mut String, config: &Config<'_>,
) -> Result<&'a str, tokio::io::Error>
where
    T: AsyncBufReadExt + Unpin,
{
    buf.clear();
    let size = time::timeout(
        Duration::from_secs(config.read_timeout()),
        stream.read_line(buf),
    )
    .await??;
    Ok(&buf[..size].trim())
}

async fn write_line<'a, T>(
    stream: &mut T, buf: &str,
) -> Result<(), tokio::io::Error>
where
    T: AsyncBufReadExt + Unpin + AsyncWriteExt,
{
    stream.write_all(format!("{}\n", buf).as_bytes()).await?;
    Ok(())
}

async fn internal_process<T>(
    stream: &mut BufReader<T>, config: &Config<'_>,
) -> Result<(), Box<dyn Error>>
where
    T: AsyncRead + Unpin + AsyncWriteExt,
{
    let line = &mut String::new();

    let key = gen_key(config);
    let challenge = gen_challenge(config);
    let challenge_hex = hex::encode(&challenge);

    write_line(
		stream,
		"This service generates signatures using SHA224(secret_key||message). Submissions must be encoded in base 16.",
	)
		.await?;
    write_line(
		stream,
		&format!(
			"Authenticate by submitting a message containing the bytes {} (hex encoded) and the corresponding signature.",
			challenge_hex
		),
	)
		.await?;

    loop {
        write_line(stream, "Submit a message.").await?;
        let line_mod = trimmed_read_line(stream, line, config).await?;

        let line_mod_vec = match hex::decode(line_mod) {
            Ok(line) => line,
            Err(_) => {
                write_line(stream, "Bad encoding.").await?;
                continue;
            },
        };

        let cleaned_line_mod = hex::encode(&line_mod_vec);
        let sign = !cleaned_line_mod.contains(&challenge_hex);

        let mut hash = Sha224::new();
        let mut input = Vec::with_capacity(key.len() + line_mod_vec.len());
        input.extend_from_slice(&key);
        input.extend_from_slice(&line_mod_vec);
        let mut digest = vec![0; hash.output_bytes()];
        hash.input(&input);
        hash.result(&mut digest);

        if sign {
            write_line(
                stream,
                &format!("The signature is {}.", hex::encode(&digest)),
            )
            .await?;
        } else {
            #[cfg(debug_assertions)]
            println!("Valid signature {}.", hex::encode(&digest));
            write_line(stream, "Submit a signature.").await?;

            let line_mod = trimmed_read_line(stream, line, config).await?;
            let line_mod_vec = match hex::decode(line_mod) {
                Ok(line) => line,
                Err(_) => {
                    write_line(stream, "Bad encoding.").await?;
                    continue;
                },
            };

            // not constant time
            if line_mod_vec == digest {
                write_line(stream, &format!("The flag is {}.", config.flag()))
                    .await?;
                break;
            } else {
                write_line(stream, "Bad signature.").await?;
            }
        }
    }

    Ok(())
}
