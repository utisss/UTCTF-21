use std::net::{
    IpAddr,
    Ipv6Addr,
};

pub const DEFAULT_PORT: Option<u16> = Some(3000);
pub const DEFAULT_ACCEPTED_DN: Option<&str> = None;
pub const DEFAULT_FLAG: Option<&str> = None;

lazy_static! {
    pub static ref DEFAULT_IP: Option<IpAddr> =
        Some(IpAddr::V6(Ipv6Addr::from(0)));
}
