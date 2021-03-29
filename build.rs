extern crate ructe;
extern crate vergen;

use ructe::Ructe;
use vergen::{vergen, Config, ShaKind};

fn main() {
    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    vergen(config).expect("Unable to generate static repo info");
    Ructe::from_env()
        .expect("ructe")
        .compile_templates("templates")
        .unwrap();
}
