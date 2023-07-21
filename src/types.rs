//! Defines all the necessary data structures/types needed to implement a DNS resolver.

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
     * For simplicity sake, we'll reuse the QType enum.
     */
    pub r#type: QType,

    /// The class of the DNS record, typically IN for Internet.
    pub class: QClass,

    /// The time-to-live of the record, which indicates how long the record can be cached.
    pub ttl: i32,

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
        bytes.write_u16::<BigEndian>(self.r#type as u16)?;
        bytes.write_u16::<BigEndian>(self.class as u16)?;
        bytes.write_i32::<BigEndian>(self.ttl)?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QType {
    /// A host address.
    A = 1,

    /// An authoritative name server
    NS = 2,

    /// A mail destination. Obsolete, use MX instead.
    // MD = 3,

    /// A mail forwarder. Obsolete, use MX instead.
    // MF = 4,

    /// The canonical name for an alias.
    CNAME = 5,

    /// Marks the start of a zone of authority.
    SOA = 6,

    /// A mailbox domain name (EXPERIMENTAL)
    MB = 7,

    /// A mail group member (EXPERIMENTAL)
    MG = 8,

    /// A mail rename domain name (EXPERIMENTAL)
    MR = 9,

    /// A null RR (EXPERIMENTAL)
    NULL = 10,

    /// A well known service description.
    WKS = 11,

    /// A domain name pointer.
    PTR = 12,

    /// Host information.
    HINFO = 13,

    /// Mailbox or mail list information.
    MINFO = 14,

    /// Mail exchange.
    MX = 15,

    /// Text strings.
    TXT = 16,

    /// A request for a transfer of an entire zone.
    AXFR = 252,

    /// A request for mailbox-related records (MB, MG or MR).
    MAILB = 253,

    /// A request for mail agent RRs. Obsolete, use MX instead.
    // MAILA = 254,

    /// A request for all records.
    ANY = 255,
}

/*
 * Into trait is known as a "conversion trait".
 * It's used for generic conversions across different types.
 * When implemented, it allows one type to be "converted into" another type.
 * The following code allows us to convert QType into u16.
 *
 * None of the below methods are designated 'pub' because it is implied.
 */
impl Into<u16> for QType {
    fn into(self) -> u16 {
        self as u16
    }
}

/// QClass values: <https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.5>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QClass {
    /// The Internet
    IN = 1,

    /// The CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS = 2,

    /// The CHAOS class
    CH = 3,

    /// Hesiod [Dyer 87]
    HS = 4,

    /// Any class
    ANY = 255,
}

impl Into<u16> for QClass {
    fn into(self) -> u16 {
        self as u16
    }
}

/// Header format: <https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1>
#[derive(Debug)]
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

/// Question format: <https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.2>
///
/// The question section is used to carry the "question" in most queries,
/// i.e., the parameters that define what is being asked.
#[derive(Debug)]
pub struct DNSQuestion {
    /// A domain name represented as a sequence of labels (like example.com).
    pub qname: Vec<u8>,

    /// A code which specifies the type of the query (A, AAAA, etc.).
    pub qtype: u16,

    /// A code that specifies the class of the query.
    pub qclass: u16,
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
        bytes.write_u16::<BigEndian>(self.qtype)?;
        bytes.write_u16::<BigEndian>(self.qclass)?;

        Ok(bytes)
    }
}
