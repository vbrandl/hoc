extern crate ructe;
extern crate vergen;

use ructe::Ructe;
use vergen::{gen, ConstantsFlags};

fn main() {
    let flags = ConstantsFlags::SHA_SHORT;
    gen(flags).expect("Unable to generate the cargo keys!");
    Ructe::from_env()
        .expect("ructe")
        .compile_templates("templates")
        .unwrap();
}
