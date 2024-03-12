use clap::Args;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::net::{AddrParseError, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Args, Clone)]
pub struct DnsServer {
    /// Primary DNS address
    pub(crate) primary: String,

    /// Secondary DNS address
    pub(crate) secondary: String,

    /// Server name
    pub name: String,

    /// IPv6 DNS address (Default is v4)
    #[arg(long)]
    pub(crate) v6: bool,
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
