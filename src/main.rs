// Declare modules for the main binary crate.
mod decode;
mod encode;
mod types;

use types::QTypes;

fn main() {
    let qtype = QTypes::CNAME;
    let num: u16 = qtype.into();
    println!("{}", num);
}
