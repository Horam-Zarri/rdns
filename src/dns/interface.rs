use crate::dns::DnsServer;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use std::process::Command;

pub fn server_list(config_file: &str) -> Vec<DnsServer> {
    if let Ok(mut file) = File::open(config_file) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        if !contents.is_empty() {
            serde_json::from_str(&contents).unwrap()
        } else {
            Vec::new()
        }
    } else {
        File::create(config_file).expect("Could not create config file!");
        Vec::new()
    }
}
pub fn write_servers(servers: &Vec<DnsServer>, config_file: &str) {
    let str_rep = serde_json::to_string(&servers).unwrap();

    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_file)
        .unwrap()
        .write_all(str_rep.as_bytes())
        .unwrap();
}

#[cfg(windows)]
fn active_connections() -> Vec<String> {
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

fn target_adapter() -> String {
    let adapters = active_connections();

    if adapters.len() == 1 {
        adapters[0].clone()
    } else {
        println!("Found more than one connection.\nWhich one do you want to set?");

        adapters
            .iter()
            .enumerate()
            .for_each(|(idx, adp)| println!("{idx}: {adp}"));

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf = buf.trim().to_string();

        let id = buf.parse::<usize>().expect("Invalid index!");

        if id >= adapters.len() {
            panic!("Invalid Index!");
        }

        adapters[id].clone()
    }
}

#[cfg(windows)]
pub(super) fn set_static_windows(dns: &DnsServer) -> Result<(), Box<dyn Error>> {
    let adp = target_adapter();
    let ip_ver = if dns.v6 { "ipv6" } else { "ipv4" };

    runas::Command::new("netsh")
        .args(&[
            "interface",
            ip_ver,
            "set",
            "dnsservers",
            &adp,
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
            &adp,
            &dns.secondary,
            "index=2",
        ])
        .status()?;

    Ok(())
}

#[cfg(windows)]
pub(super) fn set_dhcp_windows(v6: bool) -> Result<(), Box<dyn Error>> {
    let adp = target_adapter();
    let ip_ver = if v6 { "ipv6" } else { "ipv4" };

    runas::Command::new("netsh")
        .args(&[
            "interface",
            ip_ver,
            "set",
            "dnsservers",
            &adp,
            "source=dhcp",
        ])
        .status()?;

    Ok(())
}
