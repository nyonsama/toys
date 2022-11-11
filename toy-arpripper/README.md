# Toy ARP Ripper
Just a simple(and useless) arp attack tool
## Build
```
$ cargo build --release
```
If you prefer statically linked:
```
$ cargo build --release --target=x86_64-unknown-linux-musl
```
## Usage
```
$ cargo run --release -- --help
    Finished release [optimized] target(s) in 0.02s
     Running `./toy-arpripper --help`
toy-arpripper 0.1.0

USAGE:
    toy-arpripper [FLAGS] [OPTIONS] <target-ip> <target-mac> <sender-ip> <sender-mac> <interface>

FLAGS:
    -b, --brute      brute mode(just send packet at a fixed interval without waiting requests)
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --interval <interval>    The interval(ms) of brute mode

ARGS:
    <target-ip>     ip of the host that receives arp packet
    <target-mac>    mac of the host that receives arp packet
    <sender-ip>     ip that will be polluted
    <sender-mac>    the mac address in payload
    <interface>
```
## Environment variables
### Log level
`RUST_LOG=debug` or `RUST_LOG=info`  
### Translation
set `LANG=zh_CN.UTF-8` for Simplified Chinese  
Other value for English

## License
WTFPL