//! Builds and executes DNS queries.

// TODO: Remove this once I'm done.
#![allow(dead_code)]

use anyhow::{Error, Result};

use crate::encode;
use crate::types::*;

pub fn build_query(domain_name: &str, qn_type: QType) -> Result<Vec<u8>, Error> {
    let name = encode::dns_name(domain_name)?;

    // declare object of type DNSQuestion and populate with data.
    let dns_qn = DNSQuestion {
        qname: name,
        qtype: qn_type.into(),
        qclass: QClass::IN.into(),
    };

    // Convert DNSQuestion to bytes.
    let query_bytes = dns_qn.to_bytes()?;

    Ok(query_bytes)
}

pub fn execute_query() {
    println!("Executing query...");
}
