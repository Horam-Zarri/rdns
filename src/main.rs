mod dns;
mod interface;

use crate::dns::DnsServer;
use crate::interface::DnsInterface;
use clap::Parser;
use clap::Subcommand;

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
    },
}

fn main() {
    let cli = Cli::parse();
    let interface = DnsInterface::new();

    match cli.command {
        Commands::Set { name } => {
            let server_list = interface.server_list();

            interface.set_dns_static(
                &server_list
                    .iter()
                    .find(|&dns| dns.name == name)
                    .expect("Specified DNS does not exist in list")
                    .clone(),
            );
        }

        Commands::Add(raw_dns) => {
            if DnsServer::verify_dns(&raw_dns).is_err() {
                eprintln!("Invalid DNS address format!");
                return;
            }

            let mut servers = interface.server_list();

            if servers.iter().any(|dns| dns.conflicts_with(&raw_dns)) {
                eprintln!("Another DNS with same name/addr exists!");
                return;
            }

            servers.push(raw_dns);
            interface.write_servers(&servers);
        }

        Commands::Rem { names } => {
            let mut servers = interface.server_list();

            for name in names {
                if let Some((dns_pos, _)) =
                    servers.iter().enumerate().find(|(_, dns)| dns.name == name)
                {
                    servers.remove(dns_pos);
                    println!("Removed DNS server: {name}");
                } else {
                    println!("DNS server {name} not found in list!");
                }
            }

            interface.write_servers(&servers);
        }
        Commands::Direct {
            primary,
            secondary,
            v6,
            dhcp,
        } => {
            if dhcp {
                interface.set_dns_dhcp(v6);
            } else {
                let dns_res = DnsServer::build(
                    primary.expect("clap should avoid empty servers when DHCP is not set"),
                    secondary.expect("clap should avoid empty servers when DHCP is not set"),
                    v6,
                );
                match dns_res {
                    Ok(ref dns_server) => interface.set_dns_static(dns_server),
                    Err(_) => eprintln!("Invalid DNS address format!"),
                }
            }
        }

        Commands::List => interface
            .server_list()
            .iter()
            .for_each(|dns| println!("{dns}")),
    }
}
