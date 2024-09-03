use crate::dns::DnsServer;
use std::cell::OnceCell;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::ops::Deref;
use std::path::{self, Path, PathBuf};
use std::io;
use std::io::{Read, Write};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
type SYSTEM_INTERFACE = linux::Linux;

#[cfg(target_os = "windows")]
type SYSTEM_INTERFACE = windows::Windows;

const CONFIG_DIR: &str = "rdns";
const CONFIG_FILE: &str = "rdns_servers.json";

trait OsInterface {
    fn active_connections() -> Vec<String>;
    fn set_static(dns: &DnsServer, adapter: &str) -> Result<(), Box<dyn Error>>;
    fn set_dhcp(adapter: &str, v6: bool) -> Result<(), Box<dyn Error>>;

    fn config_dir() -> String {
        format!("{}{}{}", directories::BaseDirs::new()
            .unwrap().config_local_dir().to_str()
            .unwrap().to_string(), std::path::MAIN_SEPARATOR, CONFIG_DIR)
    }

    fn config_path() -> String {
        format!("{}{}{}", Self::config_dir(),
            std::path::MAIN_SEPARATOR, CONFIG_FILE)
    }
}

pub struct DnsInterface;

impl DnsInterface {
    pub fn server_list() -> Vec<DnsServer> {
        let config_file = SYSTEM_INTERFACE::config_path();
        let mut configs = vec![];

        if let Ok(mut file) = File::open(config_file.clone()) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            configs = serde_json::from_str(&contents).unwrap();
        }

        configs
    }

    pub fn write_servers(servers: &Vec<DnsServer>) {
        let config_dir = SYSTEM_INTERFACE::config_dir();
        if !Path::new(&config_dir).exists() {
            if let Err(e) = std::fs::create_dir(config_dir) {
                eprintln!("Could not create config file's directory: {e:?}");
                std::process::abort();
            }
        }
        let config_file = SYSTEM_INTERFACE::config_path();
        let str_rep = serde_json::to_string(&servers).unwrap();

        std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(config_file)
            .unwrap()
            .write_all(str_rep.as_bytes())
            .unwrap();
    }

    pub fn set_dns_static(dns_server: &DnsServer) {
        if let Err(e) = SYSTEM_INTERFACE::set_static(dns_server, Self::target_adapter().as_str()) {
            eprintln!("Failed to set DNS: {e:?}");
            std::process::abort();
        }
    }

    pub fn set_dns_dhcp(v6: bool) {
        if let Err(e) = SYSTEM_INTERFACE::set_dhcp(Self::target_adapter().as_str(), v6) {
            eprintln!("Failed to set DNS: {e:?}");
            std::process::abort();
        }
    }

    fn target_adapter() -> String {
        let adapters = SYSTEM_INTERFACE::active_connections();

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
}
