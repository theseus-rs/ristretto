//! Real leaf allocation, retained reads and reclamation with multiple actors.
#![expect(clippy::print_stdout, reason = "machine-readable stress report")]
use ristretto_gc::{Error, Result};

fn number(value: &str) -> Result<usize> {
    value
        .parse()
        .map_err(|_| Error::RootError(format!("invalid unsigned integer: {value}")))
}
fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [mode, actors, helpers, operations, capacity, retained] = args.as_slice() else {
        return Err(Error::RootError(
            "usage: leaf_contention MODE ACTORS HELPERS OPERATIONS_PER_ACTOR CAPACITY RETAINED"
                .into(),
        ));
    };
    println!(
        "{}",
        ristretto_gc::experimental_leaf_contention(
            mode,
            number(actors)?,
            number(helpers)?,
            number(operations)?,
            number(capacity)?,
            number(retained)?,
        )?
    );
    Ok(())
}
