//! Uninstrumented shared owning-root clone/drop diagnostic.
#![expect(clippy::print_stdout, reason = "machine-readable timing")]
use ristretto_gc::{Error, Result};
fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [mode, copies] = args.as_slice() else {
        return Err(Error::RootError("usage: root_aliases MODE COPIES".into()));
    };
    let copies = copies
        .parse()
        .map_err(|_| Error::RootError("invalid copies".into()))?;
    println!(
        "{}",
        ristretto_gc::experimental_root_aliases(mode, copies, |_| {})?
    );
    Ok(())
}
