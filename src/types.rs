/*!
 * All communications inside of the domain protocol are carried in a single format called a DNS message.
 *
 * The top level format of a message is divided into 5 sections: header, question, answer, authority
 * and additional info.
 */

// TODO: Remove this once I'm done with the project.
#![allow(dead_code)]

/// Header format: <https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1>
#[derive(Debug)]
pub struct DNSHeader {
    /// Assigned by the program that generates any kind of query.
    id: u16,

    // TODO: If I wanted to implement it, how would I go about it?
    /// Mostly going to be ignored.
    flags: u16,

    /// Specifies the number of entries in the question section.
    qd_count: u16,

    /// Specifies the number of resource records in the answer section.
    an_count: u16,

    /// Specifies the number of name server resource records in the authority records section.
    ns_count: u16,

    /// Specifies the number of resource records in the additional records section.
    ar_count: u16,
}

/// Question format: <https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.2>
///
/// The question section is used to carry the "question" in most queries,
/// i.e., the parameters that define what is being asked.
#[derive(Debug)]
pub struct DNSQuestion {
    /// A domain name represented as a sequence of labels (like example.com).
    q_name: Vec<u8>,

    /// A code which specifies the type of the query (A, AAAA, etc.).
    q_type: u16,

    /// A code that specifies the class of the query.
    q_class: u16,
}

/*
 * Values are defined by the DNS protocol and must match the RFC standard exactly.
 * That's why they've been explicitly defined.
 */
/// QTypes values: <https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.3>
#[derive(Debug)]
pub enum QTypes {
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
 * The following code allows us to convert QTypes into u16.
 */
impl Into<u16> for QTypes {
    fn into(self) -> u16 {
        self as u16
    }
}

/// QClass values: <https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.5>
#[derive(Debug)]
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
