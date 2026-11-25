//! Isolated production identity allocation under contention across native threads.
//! Usage: identities [producers=4] [identities per producer=1000000].
#![cfg_attr(
    not(target_family = "wasm"),
    expect(clippy::print_stdout, clippy::panic_in_result_fn)
)]
#[cfg(not(target_family = "wasm"))]
use ristretto_gc::{Error, Result};
// Compile the production allocator itself; no copied algorithm or public test API.
#[cfg(not(target_family = "wasm"))]
#[path = "../src/identity.rs"]
mod identity;

#[cfg(target_family = "wasm")]
fn main() {}

#[cfg(not(target_family = "wasm"))]
fn main() -> Result<()> {
    use std::sync::Barrier;
    use std::time::Instant;
    let args: Vec<_> = std::env::args().collect();
    let producers = args
        .get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(4);
    let count = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1_000_000);
    assert!((1..=32).contains(&producers) && (1..=10_000_000).contains(&count));
    let ready = Barrier::new(producers + 1);
    let start = Barrier::new(producers + 1);
    let (wall_ns, workers) = std::thread::scope(|scope| -> Result<_> {
        let mut handles = Vec::with_capacity(producers);
        for _ in 0..producers {
            let (ready, start) = (&ready, &start);
            handles.push(scope.spawn(move || -> Result<_> {
                let mut ids = Vec::with_capacity(count);
                ready.wait();
                start.wait();
                let began = Instant::now();
                for _ in 0..count {
                    ids.push(std::hint::black_box(identity::ObjectId::allocate()?));
                }
                Ok((began.elapsed().as_nanos(), ids))
            }));
        }
        ready.wait();
        let began = Instant::now();
        start.wait();
        let workers = handles
            .into_iter()
            .map(|worker| {
                worker
                    .join()
                    .map_err(|_| Error::SyncError("identity worker panicked".into()))?
            })
            .collect::<Result<Vec<_>>>()?;
        Ok((began.elapsed().as_nanos(), workers))
    })?;
    let worker_ns: Vec<_> = workers.iter().map(|(ns, _)| *ns).collect();
    let mut ids: Vec<_> = workers.into_iter().flat_map(|(_, ids)| ids).collect();
    ids.sort_unstable();
    assert_eq!(ids.len(), producers * count);
    assert!(ids.windows(2).all(|pair| pair.first() != pair.get(1)));
    assert!(
        ids.iter()
            .all(|id| id.to_i64() != 0 && identity::ObjectId::from_i64(id.to_i64()) == *id)
    );
    println!(
        "{{\"producers\":{producers},\"per_worker\":{count},\"operations\":{},\"wall_ns\":{wall_ns},\"worker_ns\":{worker_ns:?},\"unique\":true}}",
        ids.len()
    );
    Ok(())
}
