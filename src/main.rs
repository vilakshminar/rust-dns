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
    let qtype = QType::CNAME;
    let num: u16 = qtype.into();
    println!("{}", num);

    println!(
        "{:?}",
        dns::build_query("google.com", QType::CNAME, QClass::IN)
    );
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
