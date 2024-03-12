use crate::dns::DnsServer;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};

mod linux;
mod windows;

trait OsInterface {
    fn config_file(&self) -> &'static str;
    fn active_connections(&self) -> Vec<String>;
    fn set_static(&self, dns: &DnsServer, adapter: &str) -> Result<(), Box<dyn Error>>;
    fn set_dhcp(&self, adapter: &str, v6: bool) -> Result<(), Box<dyn Error>>;
}

pub struct DnsInterface(Box<dyn OsInterface>);

impl DnsInterface {
    pub fn new() -> Self {
        if cfg!(target_os = "windows") {
            DnsInterface(Box::new(windows::Windows))
        }
        //else if cfg!(target_os = "linux") {
        //    DnsInterface(Box::new(linux::Linux))
        //}
        else {
            panic!("Operating system is not supported");
        }
    }

    pub fn server_list(&self) -> Vec<DnsServer> {
        let config_file = self.0.config_file();

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

    pub fn write_servers(&self, servers: &Vec<DnsServer>) {
        let config_file = self.0.config_file();
        let str_rep = serde_json::to_string(&servers).unwrap();

        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(config_file)
            .unwrap()
            .write_all(str_rep.as_bytes())
            .unwrap();
    }

    pub fn set_dns_static(&self, dns_server: &DnsServer) {
        if let Err(e) = self
            .0
            .set_static(dns_server, self.target_adapter().as_str())
        {
            eprintln!("Failed to set DNS: {e:?}");
            std::process::abort();
        }
    }

    pub fn set_dns_dhcp(&self, v6: bool) {
        if let Err(e) = self.0.set_dhcp(self.target_adapter().as_str(), v6) {
            eprintln!("Failed to set DNS: {e:?}");
            std::process::abort();
        }
    }

    fn target_adapter(&self) -> String {
        let adapters = self.0.active_connections();

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
