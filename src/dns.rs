//! Builds and executes DNS queries.

// TODO: Remove this once I'm done.
#![allow(dead_code)]

use anyhow::{Error, Result};

use crate::encode;
use crate::types::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn build_query(
    domain_name: &str,
    record_type: QType,
    record_class: QClass,
) -> Result<Vec<u8>, Error> {
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
        qtype: record_type.into(),
        qclass: record_class.into(),
    };

    // Concatenate the header and question bytes.
    let mut res = hdr.to_bytes()?;
    res.append(&mut qn.to_bytes()?);

    Ok(res)
}

pub fn execute_query() {
    println!("Executing query...");
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::{Error, Result};

    struct TestCase {
        domain: &'static str,
        record_type: QType,
        record_class: QClass,

        expected: Vec<u8>,
    }

    #[test]
    fn test_build_query() -> Result<(), Error> {
        let test_cases = vec![
            TestCase {
                domain: "",
                record_type: QType::A,
                record_class: QClass::IN,

                /*
                 * Explanation of the expected output:
                 *
                 * build_query() contatenates the header and question bytes. The header is 12 bytes long
                 * and the question will be atleast 4 bytes long (record name can take up more).
                 * So, the total length of the output will be atleast 16 bytes in length.
                 *
                 * Consider the DNS header section:
                 *
                 * id: 307 - This value is deterministic thanks to the seed used by the build_query
                 * logic. In binary, 307 is 00000001 00110011. The first 8 bits are 00000001 which is
                 * 1 in decimal or 0x01 in hexadecimal. The next 8 bits are 00110011 which is 51 in
                 * decimal or 0x33 in hexadecimal. So, the bytes would be [1, 51] or [0x01, 0x33] in
                 * big endian format.
                 *
                 * flags: 1 << 8 -> This operation is equivalent to 256 which in hexadecimal is 0x0100.
                 * So, the bytes would be [0x01, 0x00] in hexadecimal or [1, 0] in decimal.
                 *
                 * qdcount: 1 -> 1 in hexadecimal is 0x0001. So, the bytes would be [0x00, 0x01] in
                 * hexadecimal or [0, 1] in decimal.
                 *
                 * ancount: 0 -> 0 in hexadecimal is 0x0000. So, the bytes would be [0x00, 0x00] in
                 * hexadecimal or [0, 0] in decimal.
                 *
                 * nscount: 0 -> same as above, the bytes would be [0x00, 0x00] in hex or [0, 0] in decimal.
                 *
                 * arcount: 0 -> same as above.
                 *
                 * So, the header bytes would be [1, 51, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0].
                 *
                 * Now for the DNS question section:
                 *
                 * qname: This is the encoded domain name. Since the domain name is empty, the bytes would be
                 * [0x00, 0x00] in hexadecimal or [0, 0] in decimal.
                 * qtype: QType::A -> 1 in hexadecimal is 0x0001. So, the bytes would be [0x00, 0x01] in
                 * hexadecimal or [0, 1] in decimal.
                 * qclass: 1 -> same as above.
                 * So, the question bytes would be [0, 0, 0, 1, 0, 1].
                 *
                 * Concatenating the header and question bytes, we get:
                 * [1, 51, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1]
                 *
                 */
                expected: vec![1, 51, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            },
            TestCase {
                domain: "google.com",
                record_type: QType::A,
                record_class: QClass::IN,
                expected: vec![
                    1, 51, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, // header
                    6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, // question
                    0, 1, 0, 1,
                ],
            },
            TestCase {
                domain: "google.com",
                record_type: QType::TXT,
                record_class: QClass::ANY,
                expected: vec![
                    1, 51, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, // header
                    6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, // question
                    0, 16, 0, 255,
                ],
            },
        ];

        for test_case in test_cases {
            let actual = build_query(
                test_case.domain,
                test_case.record_type,
                test_case.record_class,
            )?;
            assert_eq!(
                actual, test_case.expected,
                "failed for domain: {}, type: {:?} & class: {:?}",
                test_case.domain, test_case.record_type, test_case.record_class
            );
        }
        Ok(())
    }
}
