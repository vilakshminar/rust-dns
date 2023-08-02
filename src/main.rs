// Declare modules for the main binary crate.
mod decode;
mod dns;
mod encode;
mod types;

use types::{QClass, QType};

fn main() {
    run();
}

fn run() {
    println!(
        "{:?}",
        dns::build_query("google.com", QType::CNAME, QClass::IN)
    );

    let query = dns::build_query("google.com", QType::CNAME, QClass::IN);

    // create a UDP socket
    let socket = std::net::UdpSocket::bind(std::net::SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
        0,
    ))
    .unwrap();
}

#[cfg(test)]
mod test {
    use anyhow::{Error, Result};

    #[test]
    fn test_run() -> Result<(), Error> {
        super::run();

        Ok(())
    }
}
