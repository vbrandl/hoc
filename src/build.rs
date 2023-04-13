use anyhow::Result;
use ructe::Ructe;
use vergen::EmitBuilder;

fn main() -> Result<()> {
    EmitBuilder::builder()
        .git_sha(true)
        .git_describe(true, true, None)
        .emit()?;

    let mut ructe = Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_files("static")?;
    Ok(ructe.compile_templates("templates")?)
}
