extern crate reqwest;

pub fn find_nat_ip() -> reqwest::Result<Option<std::net::Ipv4Addr>> {
    let response_text = reqwest::blocking::get("http://whatismyip.akamai.com")?.text()?;

    match parse_nat_ip(response_text) {
        Ok(ip) => Ok(Some(ip)),
        Err(_)=> Ok(None)
    }
}

fn parse_nat_ip(nat_ip: String) -> Result<std::net::Ipv4Addr, std::net::AddrParseError> {
    nat_ip.parse()
}
