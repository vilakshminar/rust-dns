//! Builds and executes DNS queries.

// TODO: Remove this once I'm done.
#![allow(dead_code)]

use anyhow::{Error, Result};

use crate::encode;
use crate::types::QType;

pub fn build_query(domain_name: &str, q_type: QType) -> Result<(), Error> {
    let name = encode::dns_name(domain_name)?;

    // TODO: Parse the result properly instead of
    println!("{:?}", name);
    println!("{:?}", q_type);

    Ok(())
}
