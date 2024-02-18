mod dns;

use std::str::FromStr;
use std::io::{Read, Write};
use clap::Parser;
use clap::{Args, Subcommand};
use crate::dns::DnsServer;
use crate::dns::interface::server_list;

const CONFIG_FILE: &str = "rdns_servers.json";

#[derive(Parser)]
#[command(name = "rdns")]
#[command(version = "0.1.0")]
#[command(about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

}


#[derive(Subcommand)]
pub enum Commands {
    /// Set a DNS server from your list
    Set {
        /// DNS server name
        name: String,
    },

    /// Add a DNS server to your list
    Add(DnsServer),

    /// Remove a DNS server from your list
    Rem {
        /// DNS server name
        names: Vec<String>,
    },

    /// List your saved DNS servers
    List,

    /// Directly set a DNS (or clear to DHCP)
    #[group(required = true)]
    Direct {
        /// Primary DNS address
        #[arg(conflicts_with = "dhcp", required_unless_present = "dhcp")]
        primary: Option<String>,

        /// Secondary DNS address
        #[arg(conflicts_with = "dhcp", required_unless_present = "dhcp")]
        secondary: Option<String>,

        #[arg(long)]
        /// IPv6 DNS address (Default is v4)
        v6: bool,

        /// Set to DHCP
        #[arg(long)]
        dhcp: bool,
    }
}



fn main() {

    let cli = Cli::parse();

    match cli.command {
        Commands::Set {name} => {
            let servers = dns::interface::server_list(CONFIG_FILE);

            servers.iter()
                .find(|&dns| dns.name == name)
                .expect("Specified DNS does not exist in list")
                .set_dns();
        }

        Commands::Add(raw_dns) => {

            if let Err(_) = DnsServer::verify_dns(&raw_dns) {
                eprintln!("Invalid DNS address format!");
                return;
            }

            let mut servers = dns::interface::server_list(CONFIG_FILE);

            if servers.iter()
                .find(|&dns| dns.conflicts_with(&raw_dns))
                .is_some()
            {
                eprintln!("Another DNS with same name/addr exists!");
                return;
            }

            servers.push(raw_dns);

            dns::interface::write_servers(&servers, CONFIG_FILE);
        }

        Commands::Rem{names} => {

            let mut servers = dns::interface::server_list(CONFIG_FILE);

            for name in names {
                if let Some((dns_pos, _)) = servers.iter()
                    .enumerate()
                    .find(|(idx, dns)| dns.name == name)
                {
                    servers.remove(dns_pos);
                    println!("Removed DNS server: {name}");
                }
                else {
                    println!("DNS server {name} not found in list!");
                }
            }

            dns::interface::write_servers(&servers, CONFIG_FILE);
        }
        Commands::Direct {
            primary,
            secondary,
            v6,
            dhcp
        } => {
            if dhcp {
                DnsServer::set_dhcp(v6);
            }
            else {
                let dns_res = DnsServer::build(
                    primary.expect("clap should avoid empty servers when DHCP is not set"),
                    secondary.expect("clap should avoid empty servers when DHCP is not set"),
                    v6
                );
                match dns_res {
                    Ok(dns) => dns.set_dns(),
                    Err(e) => eprintln!("Invalid DNS address format!"),
                }
            }
        }

        Commands::List => server_list(CONFIG_FILE).iter()
            .for_each(|dns| println!("{dns}")),
    }
}


