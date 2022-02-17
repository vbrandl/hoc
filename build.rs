extern crate ructe;
extern crate vergen;

use ructe::{Ructe, RucteError};
use vergen::{vergen, Config, ShaKind};

fn main() -> Result<(), RucteError> {
    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    vergen(config).expect("Unable to generate static repo info");
    Ructe::from_env()?.compile_templates("templates")
}
