use std::net::{
    IpAddr,
    Ipv6Addr,
};

use std::num::NonZeroU64;

pub const DEFAULT_PORT: Option<u16> = Some(3000);
pub const DEFAULT_TIMEOUT: Option<NonZeroU64> =
    Some(NonZeroU64::new(300).unwrap());
pub const DEFAULT_FLAG: Option<&str> = Some("utflag{TESTING_ONLY}");
pub const DEFAULT_MINIMUM_KEY_SIZE: Option<u8> = Some(16);
pub const DEFAULT_MAXIMUM_KEY_SIZE: Option<u8> = Some(32);

lazy_static! {
    pub static ref DEFAULT_IP: Option<IpAddr> =
        Some(IpAddr::V6(Ipv6Addr::from(0)));
}
