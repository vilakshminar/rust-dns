//! Defines all the necessary data structures needed to implement a DNS resolver.

// TODO: Remove this once I'm done.
#![allow(dead_code)]

use anyhow::{Error, Result};
use std::io::Write;

/*
 * byteorder is a lower-level library that provides utilities for handling byte order (endianness).
 * It allows you to read or write numeric types to byte arrays directly in either big-endian
 * or little-endian order.
 */
use byteorder::{BigEndian, WriteBytesExt};

/*
 * RR (Resource Records) definitions.
 * All RRs have the same top level format: Name, Type, Class, TTL, RDLength, RData.
 * Type & Class fields are a subset of QType & QClass that are defined below.
 */
/// RR definition: <https://datatracker.ietf.org/doc/html/rfc1035#section-3.2>
#[derive(Debug)]
pub struct DNSRecord {
    /// The domain name to which this record applies.
    pub name: Vec<u8>,

    /**
     * The type of DNS record, such as A (IPv4 address), AAAA (IPv6 address), CNAME, etc.
     */
    pub r#type: QType,

    /// The class of the DNS record, typically IN for Internet.
    pub class: QClass,

    /// The time-to-live of the record, which indicates how long the record can be cached.
    pub ttl: u32,

    /// The length in octets of the data field.
    pub rdlength: u16,

    /// Additional record-specific data.
    pub rdata: Vec<u8>,
}

impl DNSRecord {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        // Create an empty byte array.
        let mut bytes = Vec::new();

        // Write the name. For simplicity, we just write the bytes vector with name.
        bytes.write_all(&self.name)?;

        // Write the other fields.
        bytes.write_u16::<BigEndian>(self.r#type.into())?;
        bytes.write_u16::<BigEndian>(self.class.into())?;
        bytes.write_u32::<BigEndian>(self.ttl)?;
        bytes.write_u16::<BigEndian>(self.rdlength)?;

        bytes.write_all(&self.rdata)?;

        Ok(bytes)
    }
}

/*
 * Values are defined by the DNS protocol and must match the RFC standard exactly.
 * That's why they've been explicitly defined.
 */
/// QType values: <https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.3>
#[derive(Debug, Copy, Clone)]
pub enum QType {
    /// A host address.
    A,

    /// An authoritative name server
    NS,

    /// A mail destination. Obsolete, use MX instead.
    // MD,

    /// A mail forwarder. Obsolete, use MX instead.
    // MF,

    /// The canonical name for an alias.
    CNAME,

    /// Marks the start of a zone of authority.
    SOA,

    /// A mailbox domain name (EXPERIMENTAL)
    MB,

    /// A mail group member (EXPERIMENTAL)
    MG,

    /// A mail rename domain name (EXPERIMENTAL)
    MR,

    /// A null RR (EXPERIMENTAL)
    NULL,

    /// A well known service description.
    WKS,

    /// A domain name pointer.
    PTR,

    /// Host information.
    HINFO,

    /// Mailbox or mail list information.
    MINFO,

    /// Mail exchange.
    MX,

    /// Text strings.
    TXT,

    /// A request for a transfer of an entire zone.
    AXFR,

    /// A request for mailbox-related records (MB, MG or MR).
    MAILB,

    /// A request for mail agent RRs. Obsolete, use MX instead.
    // MAILA,

    /// A request for all records.
    ANY,
}

/*
 * From<T> represents the conversion of a value of type T
 * into a target type (impl From<T> for TargetType).
 *
 * It's used for generic conversions across different types.
 * The following code allows us to convert QType into u16.
 *
 * None of the below methods are designated 'pub' because it is implied.
 */
impl From<QType> for u16 {
    fn from(original: QType) -> u16 {
        match original {
            QType::A => 1,
            QType::NS => 2,
            // QType::MD => 3,
            // QType::MF => 4,
            QType::CNAME => 5,
            QType::SOA => 6,
            QType::MB => 7,
            QType::MG => 8,
            QType::MR => 9,
            QType::NULL => 10,
            QType::WKS => 11,
            QType::PTR => 12,
            QType::HINFO => 13,
            QType::MINFO => 14,
            QType::MX => 15,
            QType::TXT => 16,
            QType::AXFR => 252,
            QType::MAILB => 253,
            //QType::MAILA => 254,
            QType::ANY => 255,
        }
    }
}

/// QClass values: <https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.5>
#[derive(Debug, Copy, Clone)]
pub enum QClass {
    /// The Internet
    IN,

    /// The CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS,

    /// The CHAOS class
    CH,

    /// Hesiod [Dyer 87]
    HS,

    /// Any class
    ANY,
}

impl From<QClass> for u16 {
    fn from(original: QClass) -> u16 {
        match original {
            QClass::IN => 1,
            QClass::CS => 2,
            QClass::CH => 3,
            QClass::HS => 4,
            QClass::ANY => 255,
        }
    }
}

/// Header format: <https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1>
pub struct DNSHeader {
    /// Assigned by the program that generates any kind of query.
    pub id: u16,

    // TODO: If I wanted to implement it, how would I go about it?
    /// Mostly going to be ignored.
    pub flags: u16,

    /// Specifies the number of entries in the question section.
    pub qdcount: u16,

    /// Specifies the number of resource records in the answer section.
    pub ancount: u16,

    /// Specifies the number of name server resource records in the authority records section.
    pub nscount: u16,

    /// Specifies the number of resource records in the additional records section.
    pub arcount: u16,
}

impl DNSHeader {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        /*
         * Create an empty byte array.
         * Vec::new() is a generic function that creates a new growable vector.
         * The type of a generic function or method is usually inferred from the context where it's used.
         */
        let mut bytes = Vec::new();

        /*
         * Write the fields as a 2-byte integer in network byte order (big endian).
         * The names "big endian" and "little endian" come from Gulliver's Travels.
         * Theres no real advantage to the byte order itself. For computer networking,
         * big endian is the default.
         */
        bytes.write_u16::<BigEndian>(self.id)?;
        bytes.write_u16::<BigEndian>(self.flags)?;
        bytes.write_u16::<BigEndian>(self.qdcount)?;
        bytes.write_u16::<BigEndian>(self.ancount)?;
        bytes.write_u16::<BigEndian>(self.nscount)?;
        bytes.write_u16::<BigEndian>(self.arcount)?;

        Ok(bytes)
    }
}

/**
 * Question format: <https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.2>
 *
 * The question section is used to carry the "question" in most queries,
 * i.e., the parameters that define what is being asked.
 */
pub struct DNSQuestion {
    /// A domain name represented as a sequence of labels (like example.com).
    pub qname: Vec<u8>,

    /// A code which specifies the type of the query (A, AAAA, etc.).
    pub qtype: QType,

    /// A code that specifies the class of the query.
    pub qclass: QClass,
}

impl DNSQuestion {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        // Create an empty byte array.
        let mut bytes = Vec::new();

        /*
         * Write the qname.
         * For simplicity, we just write the bytes vector with qname.
         * Depending on the actual DNS protocol, there may be more complex transformations needed.
         */
        bytes.write_all(&self.qname)?;

        // Write the other fields.
        bytes.write_u16::<BigEndian>(self.qtype.into())?;
        bytes.write_u16::<BigEndian>(self.qclass.into())?;

        Ok(bytes)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::{Error, Result};

    struct TestCase {
        record: DNSRecord,
        expected: Vec<u8>,
    }

    #[test]
    fn test_to_bytes() -> Result<(), Error> {
        let test_cases = vec![
            TestCase {
                record: DNSRecord {
                    name: vec![],
                    r#type: QType::NULL,
                    class: QClass::CS,
                    ttl: 0,
                    rdlength: 0,
                    rdata: vec![],
                },
                expected: vec![0, 10, 0, 2, 0, 0, 0, 0, 0, 0],
            },
            TestCase {
                record: DNSRecord {
                    name: vec![b' '],
                    r#type: QType::NS,
                    class: QClass::CH,
                    ttl: 0,
                    rdlength: 1,
                    rdata: vec![],
                },
                expected: vec![32, 0, 2, 0, 3, 0, 0, 0, 0, 0, 1],
            },
            TestCase {
                record: DNSRecord {
                    name: vec![b'g', b'o', b'o', b'g', b'l', b'e', b'.', b'c', b'o', b'm'],
                    r#type: QType::ANY,
                    class: QClass::IN,
                    ttl: 3600,
                    rdlength: 4,
                    rdata: vec![127, 0, 0, 1],
                },
                expected: vec![
                    103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 0, 255, 0, 1, 0, 0, 14, 16, 0,
                    4, 127, 0, 0, 1,
                ],
            },
            TestCase {
                record: DNSRecord {
                    name: vec![
                        b'f', b'a', b'c', b'e', b'b', b'o', b'o', b'k', b'.', b'c', b'o', b'm',
                    ],
                    r#type: QType::A,
                    class: QClass::HS,
                    ttl: 7200,
                    rdlength: 4,
                    rdata: vec![157, 240, 0, 1],
                },
                expected: vec![
                    102, 97, 99, 101, 98, 111, 111, 107, 46, 99, 111, 109, 0, 1, 0, 4, 0, 0, 28,
                    32, 0, 4, 157, 240, 0, 1,
                ],
            },
        ];

        for test_case in test_cases {
            let actual = test_case.record.to_bytes()?;
            assert_eq!(
                actual, test_case.expected,
                "failed for dns record: {:?}",
                test_case.record
            );
        }
        Ok(())
    }
}
