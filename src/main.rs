// Declare modules for the main binary crate.
mod decode;
mod dns;
mod encode;
mod types;

use types::{QClass, QType};

fn main() {
    let qtype = QType::CNAME;
    let num: u16 = qtype.into();
    println!("{}", num);

    println!(
        "{:?}",
        dns::build_query("google.com", QType::CNAME, QClass::IN)
    );
}
