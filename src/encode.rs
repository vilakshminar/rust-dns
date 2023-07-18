//! Implements encoding logic, for example, encoding a domain name into a byte array etc.

// TODO: Remove this once I'm done.
#![allow(dead_code)]

use anyhow::{Error, Result};

/*
 * As of Rust 1.55, the Vec<u8> type implements the Write trait
 * directly. So, importing the std::io::Write trait is enough.
 */
use std::io::Write;

/*
 * Encode a domain name & translates it into a byte array.
 * For example: "google.com" becomes "6google3com0" in byte array
 * format, where 6 is the length of the word "google", 3 is the
 * length of the word "com" and 0 represents the end of the domain name.
 */
pub fn dns_name(domain_name: &str) -> Result<Vec<u8>, Error> {
    let mut encoded = Vec::new();
    for part in domain_name.split('.') {
        println!("1");
        let part_as_bytes = part.as_bytes();

        // prepend length of the part.
        encoded.push(part_as_bytes.len() as u8);

        // write the part itself.
        encoded.write_all(part_as_bytes)?;
    }

    // null byte at the end.
    encoded.push(0);
    Ok(encoded)
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        domain: &'static str,
        expected: Vec<u8>,
    }

    #[test]
    fn test_encode_dns_name() {
        let test_cases = vec![
            TestCase {
                domain: "google.com",
                expected: vec![
                    6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
                ],
            },
            TestCase {
                domain: "facebook.com",
                expected: vec![
                    8, b'f', b'a', b'c', b'e', b'b', b'o', b'o', b'k', 3, b'c', b'o', b'm', 0,
                ],
            },
            TestCase {
                domain: "",
                expected: vec![0, 0],
            },
        ];

        for test_case in test_cases {
            let actual = dns_name(test_case.domain).unwrap();
            assert_eq!(
                actual, test_case.expected,
                "failed for domain: {}",
                test_case.domain
            );
        }
    }
}
