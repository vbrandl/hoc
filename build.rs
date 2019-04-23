extern crate ructe;
extern crate vergen;

use ructe::Ructe;
use vergen::{generate_cargo_keys, ConstantsFlags};

fn main() {
    let flags = ConstantsFlags::SHA_SHORT;
    generate_cargo_keys(flags).expect("Unable to generate the cargo keys!");
    Ructe::from_env()
        .expect("ructe")
        .compile_templates("templates");
}
