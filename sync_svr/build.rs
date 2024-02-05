use std::io::Result;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/protos/");
    prost_build::compile_protos(&["src/protos/sync_game.proto"], &["src/protos/"])?;
    Ok(())
}
