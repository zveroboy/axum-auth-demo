use std::net::{IpAddr, Ipv4Addr};

pub const ADDR_URL: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const ADDR_PORT: u16 = 3000;

pub mod domain;
pub mod infrastructure;
