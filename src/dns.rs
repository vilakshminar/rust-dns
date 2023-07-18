//! Builds and executes DNS queries.

// TODO: Remove this once I'm done.
#![allow(dead_code)]

use anyhow::{Error, Result};

use crate::encode;
use crate::types::*;

pub fn build_query(domain_name: &str, qn_type: QType) -> Result<Vec<u8>, Error> {
    let name = encode::dns_name(domain_name)?;

    // TODO: Parse the result properly instead of
    println!("{:?}", name);
    println!("{:?}", qn_type);

    // declare object of type DNSQuestion and populate with data.
    let dns_qn = DNSQuestion {
        q_name: name,
        q_type: qn_type.into(),
        q_class: QClass::IN.into(),
    };

    Ok(name)
}

pub fn execute_query() {
    println!("Executing query...");
}
