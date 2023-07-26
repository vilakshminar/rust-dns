//! Builds and executes DNS queries.

// TODO: Remove this once I'm done.
#![allow(dead_code)]

use anyhow::{Error, Result};

use crate::encode;
use crate::types::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn build_query(domain_name: &str, qn_type: &QType) -> Result<Vec<u8>, Error> {
    let name = encode::dns_name(domain_name)?;

    /*
     * Seed is a fixed 32-byte array.
     * This will make the random number generation deterministic,
     * meaning it will produce the same sequence of numbers each time the program is run.
     */
    let seed = [1; 32];

    // Generate a random number from 0 to 65535 with the given seed.
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    /*
     * The ..= syntax is used to make the range inclusive on both ends (0 and 65535),
     * matching the Python randint behavior.
     */
    let id = rng.gen_range(0..=65535);

    /*
     * The format for the flags can be found in <https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1>
     * of the RFC. The flags are in total a 16 bit integer where the 9th bit is the recursion desired bit.
     */
    const RECURSION_DESIRED: u16 = 1 << 8;

    println!("random id: {}", id);

    // Create object of type DNSHeader.
    let hdr = DNSHeader {
        id,
        flags: RECURSION_DESIRED,
        qdcount: 1,
        ancount: 0,
        nscount: 0,
        arcount: 0,
    };

    // Declare object of type DNSQuestion.
    let qn = DNSQuestion {
        qname: name,
        qtype: qn_type.into(),
        qclass: QClass::IN.into(),
    };

    // Convert DNSQuestion to bytes.
    let query_bytes = qn.to_bytes()?;

    Ok(query_bytes)
}

pub fn execute_query() {
    println!("Executing query...");
}
