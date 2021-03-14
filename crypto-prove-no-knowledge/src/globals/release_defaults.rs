use std::net::{
    IpAddr,
    Ipv6Addr,
};

use std::num::{
    NonZeroU64,
    NonZeroUsize,
};

pub const DEFAULT_PORT: Option<u16> = None;
pub const DEFAULT_TIMEOUT: Option<NonZeroU64> = None;
pub const DEFAULT_FLAG: Option<&str> = None;
pub const DEFAULT_G: Option<&str> = None;
pub const DEFAULT_P: Option<&str> = None;

lazy_static! {
    pub static ref DEFAULT_IP: Option<IpAddr> =
        Some(IpAddr::V6(Ipv6Addr::from(0)));
}
