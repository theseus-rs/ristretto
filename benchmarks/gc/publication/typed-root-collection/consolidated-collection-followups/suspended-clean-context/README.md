# A suspended context can stall full collection

On `c02357ca`, an explicit context flush claimed one clean edge and suspended before publishing its mark work. With zero background workers, a different thread requested a fresh full collection and attempted an eight-byte managed allocation. Across 2,048 bounded polls and collector steps, neither completed under either publication policy. The object shard was available throughout. A separate unreachable allocation provided the funding.

| Publication policy | Pending edge records | Fresh full ticket completed | Capped allocation completed |
|---|---:|---|---|
| Legacy | 1 | No | No |
| Counted | 1 | No | No |

After the flusher resumed, the test checked the original weak identity, collected the now-unreachable graph and verified that the managed charge returned to zero. An unbounded wait on this schedule remains an open alpha release issue. [Regression patch](control.patch.gz) · [Observed assertion and toolchain output](control.log.gz).

Run from a clean checkout of the measured revision. The final assertion is expected to fail on that revision.

```sh
gzip -dc /path/to/control.patch.gz | git apply -
cargo test --locked -p ristretto_gc --lib fresh_full_and_capped_allocation_finish_while_clean_context_flusher_is_suspended
```
