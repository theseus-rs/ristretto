# Java execution during the queue-root qualification

At `ae693278`, enabling the native JIT left every measured roots operation and node constructor interpreted. Within the 250 ms measurement window, the VM entered `GcBench.operation` 349 times and `GcBench$Node.<init>` 5,584 times. Every operation constructed its expected 16 nodes. The ordinary compiler's first rejection in the operation was the `new` instruction at bytecode offset 51. It rejected the instance constructor as a non-static method.

| Policy and workload | Interpreted operations | Interpreted node constructors |
|---|---:|---:|
| candidate-allocation | 1,879 | — |
| candidate-control | 2,398 | — |
| candidate-jit-allocation | 2,232 | — |
| candidate-jit-control | 5,193 | — |
| candidate-jit-roots | 349 | 5,584 |
| candidate-roots | 345 | 5,520 |

All six ordinary windows had zero compiled entries for the required operation. The positive-control kernel recorded seven compiled entries and 35 taken compiled backedges within its window. Each ordinary fork ran after 500 ms of warmup. These are diagnostic invocation counts. Attribution of elapsed time to allocation, interpreter dispatch and collection requires a separate measurement.

A separately built release-profile executable exercised allocation and roots with the JIT enabled. Its counters cover a longer interval including warmup and reporting. [Timed-window reports and build identities](windows.jsonl.gz) · [Release-profile results](release-profile.json.gz) · [Reproduction commands](../../../../EXECUTION_MEASUREMENT.md) · [Uninstrumented comparison against G1 and ZGC](../unified-heap-roots/README.md).

A next throughput experiment can isolate the field buffer allocated by each two-field node. The baseline uses a 120-byte field buffer descriptor on native and 60 bytes on WASI. A prototype layout combining the dirty range with the buffer or two inline values measured 144 bytes on native and 72 on WASI, equal to the current buffer and range together. The probe compiled against the native and WASI libraries. [Probe source](field_layout.rs) · [Native byte sizes](layout-native.json) · [WASI byte sizes](layout-wasm32-wasip2.json). An actual JVM performance comparison must also check full Java descriptor size, tracing, heap accounting and the cost of dispatching on the new representation.
