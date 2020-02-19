# ddns-cloudflare

> Dynamic IP DNS Agent for Cloudflare DNS

`ddns-cloudflare` can help you keep a DNS A record updated when your target IP is dynamic (like many home connections). Current implementation works
only with Cloudflare as the DNS provider. It uses akami to get the NAT Gateway IP.

## Usage

The cli documentation

```
Dynamic DNS agent for Cloudflare A records

USAGE:
    ddns-cloudflare [FLAGS] [OPTIONS] --domain <domain> --zone-identifier <zone-identifier>

FLAGS:
    -h, --help        Prints help information
    -i, --interval    Frequency to check IP changes in minutes
    -V, --version     Prints version information

OPTIONS:
        --auth-token <auth-token>              API token generated on the "My Account" page [env: CF_RS_AUTH_TOKEN=]
    -d, --domain <domain>                      Domain to attach A record
    -z, --zone-identifier <zone-identifier>    Zone identifier for the DNS record
```

## Installation

The following methods can be used to install the tool

### Docker

Use the pre built docker images (~10MB) `bilby91/ddns-cloudflare`.

Supported Architectures:

- bilby91/ddns-cloudflare:x86_64-latest
- bilby91/ddns-cloudflare:armv7-latest

A tipical run invocation:

```
docker run --restart=always --detach -e CF_RS_AUTH_TOKEN=XXX-e RUST_LOG=info bilby91/ddns-cloudflare:x86_64-latest ./ddns-cloudflare --interval=10  --domain="my.domain.com" --zone-identifier=MY-CLOUDFLARE-ZONE-ID
```

### Build from source

In order to build from source you will need to install Rust toolchain with [rustup](https://rustup.rs/).

Then you can build the source with the following commands:

`cargo build --release`.

If you are looking for cross target compilation, install [cross](https://github.com/rust-embedded/cross)

## Contact

- Martín Fernández <fmartin91@gmail.com>