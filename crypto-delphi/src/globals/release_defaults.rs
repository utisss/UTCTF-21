use std::net::{
    IpAddr,
    Ipv6Addr,
};

use lazy_static::lazy_static;
use std::num::NonZeroU64;

pub const DEFAULT_PORT: Option<u16> = None;
pub const DEFAULT_TIMEOUT: Option<NonZeroU64> = None;
pub const DEFAULT_FLAG: Option<&str> = None;
pub const DEFAULT_CHALLENGE_BYTES: Option<usize> = None;

lazy_static! {
    pub static ref DEFAULT_IP: Option<IpAddr> =
        Some(IpAddr::V6(Ipv6Addr::from(0)));
}
