use std::net::{
    IpAddr,
    Ipv6Addr,
};

use std::num::NonZeroU64;

pub const DEFAULT_PORT: Option<u16> = Some(3000);
pub const DEFAULT_TIMEOUT: Option<NonZeroU64> =
    Some(NonZeroU64::new(300).unwrap());
pub const DEFAULT_FLAG: Option<&str> = Some("utflag{TESTING_ONLY}");
pub const DEFAULT_G: Option<&str> = Some("2");
pub const DEFAULT_P: Option<&str> = Some("8326966916984930512421890814640194939048549852876349599676988272731857890202244994435187088272367201082342980071680289507175272756866726621416161691461667");

lazy_static! {
    pub static ref DEFAULT_IP: Option<IpAddr> =
        Some(IpAddr::V6(Ipv6Addr::from(0)));
}
