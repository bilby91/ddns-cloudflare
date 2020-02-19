extern crate cloudflare;
extern crate reqwest;
use log::{error, debug};
use std::net::Ipv4Addr;
use cloudflare::endpoints::{dns};
use cloudflare::framework::{
    apiclient::ApiClient,
    response::{ApiFailure, ApiResponse, ApiResult},
    OrderDirection,
};

///
/// Update DNS record
///
pub fn update_record<ApiClientType: ApiClient>(
    api_client: &ApiClientType,
    ip: Ipv4Addr,
    zone_identifier: &str,
    dns_name: &str,
) {
    match find_dns_record(api_client, zone_identifier, dns_name) {
        Ok(Some(record)) => update_dns_record(api_client, &record, ip),
        Ok(None) => create_dns_record(api_client, zone_identifier, dns_name, ip),
        Err(_) => error!("Updating DNS record failed")
    }
}

fn find_dns_record<ApiClientType: ApiClient>(
    api_client: &ApiClientType,
    zone_identifier: &str,
    dns_name: &str,
) -> Result<Option<cloudflare::endpoints::dns::DnsRecord>, cloudflare::framework::response::ApiFailure> {
    let response = api_client.request(&dns::ListDnsRecords {
        zone_identifier,
        params: dns::ListDnsRecordsParams {
            direction: Some(OrderDirection::Ascending),
            ..Default::default()
        },
    });

    print_response(&response);

    response.and_then(|some| {
        Ok(some.result.into_iter().find(|element| { element.name == dns_name }))
    })
}

fn create_dns_record<ApiClientType: ApiClient>(
    api_client: &ApiClientType,
    zone_identifier: &str,
    dns_name: &str,
    ip: Ipv4Addr,
) {
    let response = api_client.request(&dns::CreateDnsRecord {
        zone_identifier,
        params: dns::CreateDnsRecordParams {
            name: dns_name,
            content: dns::DnsContent::A {
                content: ip,
            },
            priority: None,
            proxied: None,
            ttl: None,
        },
    });

    print_response(&response);
}

fn update_dns_record<ApiClientType: ApiClient>(
    api_client: &ApiClientType,
    record: &cloudflare::endpoints::dns::DnsRecord,
    ip: Ipv4Addr,
) {
    let response = api_client.request(&dns::UpdateDnsRecord {
        identifier: &record.id,
        zone_identifier: &record.zone_id,
        params: dns::UpdateDnsRecordParams {
            name: &record.name,
            content: dns::DnsContent::A {
                content: ip,
            },
            proxied: None,
            ttl: None,
        },
    });

    print_response(&response);
}

fn print_response<T: ApiResult>(response: &ApiResponse<T>) {
    match response {
        Ok(success) => debug!("Success: {:#?}", success),
        Err(e) => match e {
            ApiFailure::Error(status, errors) => {
                error!("HTTP {}:", status);
                for err in &errors.errors {
                    println!("Error {}: {}", err.code, err.message);
                    for (k, v) in &err.other {
                        error!("{}: {}", k, v);
                    }
                }
                for (k, v) in &errors.other {
                    error!("{}: {}", k, v);
                }
            }
            ApiFailure::Invalid(reqwest_err) => error!("Error: {}", reqwest_err),
        },
    }
}