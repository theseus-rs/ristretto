# Full collection after strong-stage root deletion

Each of the 36 conditions cleared 3× its f1daebeb control in the fifth paired campaign. All per-condition lower bounds also cleared 3×. Each condition used 24 randomized pairs of fresh processes. The seed was 11219.

| Target | Conditions | Processes | Smallest paired median | 95% interval for that condition |
|---|---:|---:|---:|---:|
| Native, cooperative or two workers | 24 | 1,152 | 3.062× | 3.025–3.089× |
| WASI, cooperative | 12 | 576 | 3.898× | 3.874–3.945× |

Both minima came from gated publication and mixed garbage. The native minimum used two workers with diagnostics disabled. The WASI minimum had diagnostics enabled. Intervals use process-paired bootstrap resampling and are unadjusted for multiple comparisons. The preceding campaign recorded a native minimum of 3.043× with a lower bound of 2.978×. [Preceding campaign](../progress-collection/README.md)

Each process ran 16 complete collections over 4,095 live nodes. Mixed cycles added 1,023 garbage nodes before timing, then waited for physical reclamation. Each process verified the graph checksum, number of full cycles and marked objects, swept count, refunded bytes and empty teardown. [Native graph](native.svg) · [WASI graph](wasm.svg) · [All 1,728 measurements](processes.csv) · [Paired estimates](summary.json)

The candidate contains the source of a2fe9dba. Both arms used the same hashed fixture, workspace manifests, resolved collector dependencies, profile and toolchain. To reproduce those manifests, apply the shared patch to that revision with `git apply --unidiff-zero`. The control replaces only the collector sources with f1daebeb. [Input and binary hashes](provenance.json) · [Shared manifests](shared-manifests.patch)
