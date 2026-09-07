//! Native and Wasmtime command entry point; browsers use the library's persistent reactor.
use std::io::{self, BufRead};
use std::path::PathBuf;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if std::env::args().any(|argument| argument == "--interactive") {
        for line in io::stdin().lock().lines() {
            match line {
                Ok(line) => ristretto_playground_engine::handle_request(line.as_bytes()).await,
                Err(_) => break,
            }
        }
    } else {
        let workspace = std::env::var_os("RISTRETTO_PLAYGROUND_WORKSPACE")
            .map_or_else(|| PathBuf::from("/workspace"), PathBuf::from);
        let bytes = std::fs::read(workspace.join("request.json")).unwrap_or_default();
        ristretto_playground_engine::handle_request(&bytes).await;
    }
}
