// Declare modules for the main binary crate.
mod decode;
mod dns;
mod encode;
mod types;

use anyhow::{Error, Result};
use std::net::UdpSocket;
use types::{QClass, QType};

fn main() {
    match run() {
        Ok(_) => println!("Everything worked!"),
        Err(e) => eprintln!("An error occurred: {:?}", e),
    }
}

fn run() -> Result<Vec<u8>, Error> {
    let query = dns::build_query("www.example.com", QType::A, QClass::IN)?;

    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Query Google's public DNS server.
    let dns_server = "8.8.8.8:53";

    // Send the query.
    socket.send_to(&query, dns_server)?;

    /* Buffer to hold received data.
     * UDP DNS responses are usually less than 512 bytes
     * See https://www.netmeister.org/blog/dns-size.html for MUCH more on this.
     * So reading 1024 bytes is enough.
     */
    let mut buf = [0u8; 1024];

    // Receive the DNS response.
    let (_amt, _src) = socket.recv_from(&mut buf)?;

    println!("bytes: {:?}", &buf[.._amt]);

    /*
     * TODO: Parse the DNS response from buf[..amt].
     */

    Ok(query)
}

#[cfg(test)]
mod test {
    use anyhow::{Error, Result};

    #[test]
    fn test_run() -> Result<(), Error> {
        super::run()?;
        Ok(())
    }
}
