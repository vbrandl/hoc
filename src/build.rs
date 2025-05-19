use anyhow::Result;
use ructe::Ructe;
use vergen_gix::{Emitter, GixBuilder};

fn main() -> Result<()> {
    let gix = GixBuilder::default().sha(true).build()?;
    Emitter::default().add_instructions(&gix)?.emit()?;

    let mut ructe = Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_files("static")?;
    Ok(ructe.compile_templates("templates")?)
}
