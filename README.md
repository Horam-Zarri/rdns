# rdns
A CLI app to set and organize your favorite DNS servers.

## Introduction
rdns is a CLI utility that can set your system DNS, either directly or by making your own list of preferred DNS servers,  
so that you can change your internet DNS settings blazingly fast🔥  

### **CAUTION**  
⚠️ rdns is currently only available for Windows and Linux  
⚠️ rdns uses user elevation to change DNS settings

## Usage
Simply build the app using `cargo run`. To access help page run:
```
rdns --help
```
```
Usage: rdns.exe <COMMAND>

Commands:
  set     Set a DNS server from your list
  add     Add a DNS server to your list
  rem     Remove a DNS server from your list
  list    List your saved DNS servers
  direct  Directly set a DNS (or clear to DHCP)
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
Or access help for a custom subcommand with:
```
rdns <COMMAND> --help
```
The following is the output of `rdns add --help`: 
```
Add a DNS server to your list

Usage: rdns.exe add [OPTIONS] <PRIMARY> <SECONDARY> <NAME>

Arguments:
  <PRIMARY>    Primary DNS address
  <SECONDARY>  Secondary DNS address
  <NAME>       Server name

Options:
      --v6    IPv6 DNS address (Default is v4)
  -h, --help  Print help
```

## Why?
Residing in Iran, I encounter challenges in accessing various online resources due to the necessity of utilizing VPN software or DNS services.  
A large number of software are blocked in Iran due to US trade restrictions, and I need to change DNS frequently (e.g Some DNS that works for
ChatGPT doesn't work for downloading Nvidia drivers, and in case of games one that works for Battlefield doesn't work for CoD).  

Consequently, it can get extremely inconvenient and exhaustive to change my DNS settings manually everytime using the OS settings. While there are
some GUI apps available for managing DNS configurations, I perceive them as overly elaborate and often susceptible to the same 
repetitiveness. As a result I wrote rdns to make this faster for me and anyone who may also benefit from it.
