//! Isolated final-wrapper accounting audit. Never a performance measurement.
#![allow(unsafe_code)]
#![expect(clippy::print_stdout, reason = "machine-readable accounting audit")]
use ristretto_gc::experimental_canonical_audit as audit;
use ristretto_gc::{Error, Result};
#[path = "support/root_refund_counting.rs"]
mod counting;
fn main() -> Result<()> {
    let mut all_valid = true;
    for mode in ["leaf", "graph"] {
        let mut before = audit::snapshot();
        let mut after = audit::snapshot();
        let (mut before_charge, mut after_charge, mut checkpoints) = (0, 0, 0);
        ristretto_gc::experimental_root_refund_order(mode, |label, charge| match label {
            "before" => {
                before = audit::snapshot();
                before_charge = charge;
                checkpoints += 1;
            }
            "after" => {
                after = audit::snapshot();
                after_charge = charge;
                checkpoints += 1;
            }
            _ => {}
        })?;
        let refunds = after.refunds - before.refunds;
        let refunded = after.refunded - before.refunded;
        let frees = after.frees - before.frees;
        let valid = checkpoints == 2
            && refunds == 1
            && refunded == before_charge as u64
            && before_charge != 0
            && after_charge == 0
            && frees != 0
            && after.last_refund > after.last_free;
        println!(
            "{{\"version\":1,\"mode\":\"{mode}\",\"checkpoints\":{checkpoints},\"refunds\":{refunds},\"last_free\":{},\"last_refund\":{},\"frees\":{frees},\"refunded_bytes\":{refunded},\"before_charge\":{before_charge},\"after_charge\":{after_charge},\"valid\":{valid}}}",
            after.last_free, after.last_refund
        );
        all_valid &= valid;
    }
    if !all_valid {
        return Err(Error::RootError(
            "shared root backing freed after credit refund".into(),
        ));
    }
    Ok(())
}
