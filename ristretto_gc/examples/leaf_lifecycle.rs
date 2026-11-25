//! Controlled leaf allocation/read/root-drop/reclamation cost, including cleanup.
#![expect(clippy::print_stdout, reason = "machine-readable benchmark report")]
use ristretto_gc::{Error, Result};

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [mode, operations, batch] = args.as_slice() else {
        return Err(Error::RootError(
            "usage: leaf_lifecycle MODE OPERATIONS BATCH".into(),
        ));
    };
    let operations = operations
        .parse()
        .map_err(|_| Error::RootError("invalid operations".into()))?;
    let batch = batch
        .parse()
        .map_err(|_| Error::RootError("invalid batch".into()))?;
    println!(
        "{}",
        ristretto_gc::experimental_leaf_lifecycle(mode, operations, batch)?
    );
    Ok(())
}
