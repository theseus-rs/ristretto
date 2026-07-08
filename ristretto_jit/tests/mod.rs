#![cfg(not(target_family = "wasm"))]
#![expect(
    clippy::indexing_slicing,
    clippy::panic_in_result_fn,
    reason = "integration tests use direct fixture indexing and assertions with Result-returning tests"
)]
mod functions;
mod instructions;
mod util;
