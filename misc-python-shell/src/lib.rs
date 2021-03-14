#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_debug_implementations)]

use std::{
    error::Error,
    num::NonZeroUsize,
};
use std::collections::{BTreeMap, HashMap, HashSet};

use lazy_static::lazy_static;
use regex::bytes as bregex;
use so::crypto::hash::sha256;
use sodiumoxide as so;
use tokio::{
    fs,
    io::{
        AsyncReadExt,
        AsyncWriteExt,
    },
    net::TcpStream,
    process,
    time::{
        self,
        Duration,
    },
};

const DANGEROUS_STRINGS_STRING: &str = r"(eval)|(import)|(open)|(with)|(as)|(from)|(lambda)|(\s*print\s*=\s*)|(?P<paren>\()";
const PRINT_STRINGS_STRING: &str = r"(\s*print\s*(?P<paren>\())";

lazy_static! {
    static ref DANGEROUS_STRINGS:bregex::Regex = bregex::Regex::new(DANGEROUS_STRINGS_STRING).unwrap();
    static ref PRINT_STRINGS:bregex::Regex = bregex::Regex::new(PRINT_STRINGS_STRING).unwrap();
}

pub async fn process(
    mut stream: TcpStream, timeout: Duration, upload_limit: NonZeroUsize,
) -> Result<(), Box<dyn Error>> {
    time::timeout(timeout, stream.write_all(format!(
        "Files must be uploaded within {} seconds. Scripts must run within {} seconds. Only the first {} bytes will be considered.\n",
        timeout.as_secs(),
        timeout.as_secs(),
        upload_limit,
    ).as_bytes())).await??;

    let mut buf = Vec::new();
    let len = time::timeout(timeout, stream.read_to_end(&mut buf)).await??;
    let len = std::cmp::min(upload_limit.get(), len);
    let dat = &buf[..len];

    let bad_match: HashMap<_, _> = DANGEROUS_STRINGS.captures_iter(&dat)
        .flat_map(|c| c.iter().collect::<Vec<_>>())
        .filter_map(|c| c.map(|m| (m.end(), (m.start(), String::from_utf8_lossy(m.as_bytes())))))
        .collect();
    let allow_match: HashSet<_> = PRINT_STRINGS.captures_iter(&dat)
        .filter_map(|c| c.name("paren").map(|m| m.end()))
        .collect();
    let bad_match: BTreeMap<_, _> = bad_match.into_iter()
        .filter_map(|(e, (s, v))| if !allow_match.contains(&e) { Some((s, v)) } else { None })
        .collect();

    if !bad_match.is_empty() {
        let out:String = bad_match.into_iter()
            .map(|(s, v)| format!("Char {}: {}\n", s, v))
            .collect();

        time::timeout(timeout, stream.write_all(
            format!("Blacklist: {}\nWhitelist: {}\nProhibited substrings:\n{}", DANGEROUS_STRINGS_STRING, PRINT_STRINGS_STRING, out).as_bytes()
        )).await??;

        return Ok(());
    }

    let filename = &so::hex::encode(sha256::hash(dat).0);
    fs::write("tmp/".to_string() + filename, dat).await?;
    if let Ok(child) = process::Command::new("python3")
        .arg(filename)
        .env_clear()
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .kill_on_drop(true)
        .uid(65534)
        .gid(65534)
        .current_dir("tmp")
        .spawn()
    {
        if let Ok(Ok(output)) = time::timeout(timeout, child.wait_with_output()).await {
            let _result =
                time::timeout(timeout, stream.write_all(&output.stdout)).await;
        }
    }
    fs::remove_file("tmp/".to_string() + filename).await?;

    Ok(())
}
