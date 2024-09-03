#![cfg(target_os = "linux")]

use crate::dns::DnsServer;
use crate::interface::OsInterface;
use std::error::Error;
use std::process::{Command, Stdio};

pub struct Linux;
impl OsInterface for Linux {
    fn active_connections() -> Vec<String> {
        let nmcli = String::from_utf8(
            Command::new("nmcli")
                .args(["device", "status"])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();

        nmcli
            .lines()
            .filter(|&s| s.contains("connected") && !s.contains("disconnected"))
            .map(|s| {
                let adapter_start = s.find("connected").unwrap() + 9;
                s[adapter_start..].trim().to_string()
            })
            .collect()
    }

    fn set_static(dns: &DnsServer, adapter: &str) -> Result<(), Box<dyn Error>> {
        let ip_ver = if dns.v6 { "ipv6.dns" } else { "ipv4.dns" };

        let dns_concat = format!("{} {}", dns.primary, dns.secondary);

        runas::Command::new("nmcli")
            .args(&["connection", "modify", adapter, ip_ver, &dns_concat])
            .status()?;

        Ok(())
    }

    fn set_dhcp(adapter: &str, v6: bool) -> Result<(), Box<dyn Error>> {
        panic!("Setting to DHCP is currently not supported for Linux");
    }
}
