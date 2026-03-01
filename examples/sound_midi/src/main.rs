#![forbid(unsafe_code)]
#![allow(clippy::result_large_err)]

use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, VM};
use std::path::PathBuf;

#[cfg(target_family = "wasm")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    common_main().await
}

#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() -> Result<()> {
    common_main().await
}

/// Creates an embedded JVM that plays a MIDI file using `javax.sound.midi`.
async fn common_main() -> Result<()> {
    let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let class_path = ClassPath::from(std::slice::from_ref(&cargo_manifest_dir));
    let midi_file = cargo_manifest_dir.join("entertainer.mid");
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path)
        .main_class("PlayMidi")
        .build()?;
    let vm = VM::new(configuration).await?;
    let parameters = [midi_file.to_string_lossy().to_string()];
    let str_params: Vec<&str> = parameters.iter().map(String::as_str).collect();
    let _result = vm.invoke_main(&str_params).await?;
    Ok(())
}
