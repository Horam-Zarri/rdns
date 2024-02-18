pub mod interface;

use clap::Args;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::net::{AddrParseError, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Args, Clone)]
pub struct DnsServer {
    /// Primary DNS address
    primary: String,

    /// Secondary DNS address
    secondary: String,

    /// Server name
    pub name: String,

    /// IPv6 DNS address (Default is v4)
    #[arg(long)]
    v6: bool,
}

impl DnsServer {
    pub fn build(primary: String, secondary: String, v6: bool) -> Result<Self, AddrParseError> {
        let dns = Self {
            primary,
            secondary,
            v6,
            name: "".to_string(),
        };
        dns.verify_dns()?;
        Ok(dns)
    }

    #[allow(clippy::unit_arg)]
    pub fn verify_dns(&self) -> Result<(), AddrParseError> {
        Ok({
            if self.v6 {
                Ipv6Addr::from_str(&self.primary)?;
                Ipv6Addr::from_str(&self.secondary)?;
            } else {
                Ipv4Addr::from_str(&self.primary)?;
                Ipv4Addr::from_str(&self.secondary)?;
            }
        })
    }

    pub fn conflicts_with(&self, other: &Self) -> bool {
        (self.primary == other.primary && self.secondary == other.secondary)
            || self.name == other.name
    }

    pub fn set_dns(&self) {
        if cfg!(windows) {
            if let Err(e) = interface::set_static_windows(self) {
                eprintln!("Can not set DNS :\n{e:?}");
                std::process::abort();
            }
        } else {
            panic!("Only Windows is supported for now!");
        }
    }

    pub fn set_dhcp(v6: bool) {
        if cfg!(windows) {
            if let Err(e) = interface::set_dhcp_windows(v6) {
                eprintln!("Can not set DNS:\n{e:?}");
                std::process::abort();
            }
        } else {
            panic!("Only Windows is supported for now!");
        }
    }
}

impl Display for DnsServer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:\tPrimary = [{}]\tSecondary = [{}]\tIPv{}",
            self.name,
            self.primary,
            self.secondary,
            if self.v6 { "6" } else { "4" }
        )
    }
}
