//! Controlled cyclic-graph lifecycle timings with exact teardown checks.
#![expect(clippy::print_stdout, reason = "machine-readable benchmark report")]
use ristretto_gc::{Error, Result};

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [rounds, capacity] = args.as_slice() else {
        return Err(Error::RootError(
            "usage: graph_lifecycle ROUNDS CAPACITY".into(),
        ));
    };
    let rounds = rounds
        .parse()
        .map_err(|_| Error::RootError("invalid rounds".into()))?;
    let capacity = capacity
        .parse()
        .map_err(|_| Error::RootError("invalid capacity".into()))?;
    println!(
        "{}",
        ristretto_gc::experimental_graph_lifecycle(rounds, capacity)?
    );
    Ok(())
}
