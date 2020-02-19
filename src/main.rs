extern crate clap;
extern crate cloudflare;
extern crate reqwest;

mod dns;
mod nat;
mod cli;
use std::{thread, time};
use log::{error, info};
use cloudflare::framework::{auth::Credentials,Environment, HttpApiClient, HttpApiClientConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = cli::create_cli();

    let matches = cli.get_matches();
    let token = matches.value_of("auth-token");
    let zone_identifier = matches.value_of("zone-identifier").unwrap();
    let domain = matches.value_of("domain").unwrap();
    let interval = matches.value_of("interval").unwrap_or("30");

    let credentials: Credentials = if let Some(token) = token {
        Credentials::UserAuthToken {
            token: token.to_string(),
        }
    } else {
        panic!("API token must be provided")
    };

    let mut last_nat_ip = String::from("");
    let api_client = HttpApiClient::new(
        credentials,
        HttpApiClientConfig::default(),
        Environment::Production,
    )?;

    loop {
        match nat::find_nat_ip() {
            Ok(Some(ip)) => {
                info!("Found NAT ip: {}", ip);

                // Cache the last ip to avoid DNS API calls.
                if ip.to_string() != last_nat_ip {
                    last_nat_ip = ip.to_string();

                    info!("Updating DNS A Record to: {}", last_nat_ip);
                    dns::update_record(&api_client, ip, zone_identifier, domain);
                }
            },
            Ok(None) => error!("Error while parsing external"),
            Err(err) => error!("Error while finding external ip: {}", err)
        }

        thread::sleep(time::Duration::from_secs(interval.parse().unwrap()));
    };
}
