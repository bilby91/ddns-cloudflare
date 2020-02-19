extern crate clap;

use clap::{App, AppSettings, Arg};

pub fn create_cli<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("cloudflare-rust")
        .version("0.0")
        .author("Martín Fernández <fmartin91@gmail.com>")
        .about("Dynamic DNS agent for Cloudflare A records")
        .arg(
            Arg::with_name("auth-token")
                .long("auth-token")
                .env("CF_RS_AUTH_TOKEN")
                .help("API token generated on the \"My Account\" page")
                .takes_value(true)
            )
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .help("Frequency to check IP changes in minutes")
            )
        .arg(
            Arg::with_name("zone-identifier")
                .short("z")
                .long("zone-identifier")
                .help("Zone identifier for the DNS record")
                .takes_value(true)
                .required(true)
            )
        .arg(
            Arg::with_name("domain")
                .short("d")
                .long("domain")
                .help("Domain to attach A record")
                .takes_value(true)
                .required(true)
            )
        .setting(AppSettings::ArgRequiredElseHelp)
}
