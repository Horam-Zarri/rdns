use super::OsInterface;
use crate::dns::DnsServer;
use std::error::Error;
use std::process::Command;

pub struct Windows;
const CONFIG_FILE: &str = "rdns_servers.json";

impl OsInterface for Windows {
    fn config_file(&self) -> &'static str {
        CONFIG_FILE
    }
    fn active_connections(&self) -> Vec<String> {
        let ip_config = String::from_utf8(
            Command::new("netsh")
                .args(["interface", "show", "interface"])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();

        ip_config
            .lines()
            .filter(|&s| s.contains("Connected"))
            .map(|s| {
                let adapter_start = s.rfind(' ').unwrap() + 1;
                s[adapter_start..].to_owned()
            })
            .collect()
    }

    fn set_static(&self, dns: &DnsServer, adapter: &str) -> Result<(), Box<dyn Error>> {
        let ip_ver = if dns.v6 { "ipv6" } else { "ipv4" };

        runas::Command::new("netsh")
            .args(&[
                "interface",
                ip_ver,
                "set",
                "dnsservers",
                &adapter,
                "static",
                &dns.primary,
                "primary",
            ])
            .status()?;

        runas::Command::new("netsh")
            .args(&[
                "interface",
                ip_ver,
                "add",
                "dnsservers",
                &adapter,
                &dns.secondary,
                "index=2",
            ])
            .status()?;

        Ok(())
    }

    fn set_dhcp(&self, adapter: &str, v6: bool) -> Result<(), Box<dyn Error>> {
        let ip_ver = if v6 { "ipv6" } else { "ipv4" };

        runas::Command::new("netsh")
            .args(&[
                "interface",
                ip_ver,
                "set",
                "dnsservers",
                &adapter,
                "source=dhcp",
            ])
            .status()?;

        Ok(())
    }
}
