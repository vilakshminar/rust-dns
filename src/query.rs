/*
 * All communications inside of the domain protocol are carried in a single
 * format called a DNS message. The top level format of a message is divided into
 * 5 sections: header, question, answer, authority, and additional info.
 *
 * Header section format: https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1
 * for more details.
 */
#[derive(Default, Debug)]
struct DNSHeader {
    // Assigned by the program that generates any kind of query.
    id: u16,

    // Mostly going to be ignored.
    flags: u16,

    // Specifies the number of entries in the question section.
    qd_count: u16,

    // Specifies the number of resource records in the answer section.
    an_count: u16,

    // Specifies the number of name server resource records in the authority records section.
    ns_count: u16,

    // Specifies the number of resource records in the additional records section.
    ar_count: u16,
}
