# Restricted canonical native execution

With `experimental-canonical-jit`, an admitted `canonical::Program` can prepare a
`CompiledProgram`. Preparation checks every method against the request-only JIT
capability. The ordinary VM dispatch policy is unchanged. Native compilation is
unavailable on WASI; the interpreter and token-table compatibility checks remain.

This consumer slice supports scalar/reference control flow and admitted static or
constructor calls, object allocation and instance fields. A native caller returns before invoking its callee. An explicit,
bounded vector owns the Java activation frames; Java recursion does not recursively
poll Rust futures. Each frame's typed snapshot owns its canonical references, and
return handoff acquires the returned owner before retiring the callee snapshot.
Call, return and loop boundaries yield to the host. Dropping the invocation drops
all snapshots and resets the exclusive prepared-program run lease.

`CompiledLimits` separates native slices, frame depth, request polls and cleanup
polls from interpreted instruction counting. Reports count actual native entries,
resumes and single-unit backedge yields. `instruction_count_available` is false;
zero `instructions` is not a claim of zero Java work. Limits are work bounds, not
wall-clock pause bounds. `run` owns a dedicated heap and completes cleanup before
returning a successful report or runtime error.

The compiler protocol can be broader than this consumer. The effective capability
is the intersection of verifier, canonical admission and compiler support. No
legacy context, interpreter fallback or payload pointer is linked into generated
code. Object/field requests use the same typed Execution helpers as the interpreter.
Programs containing static storage or any class initializer are rejected before
code generation, including a fieldless scalar initializer. Later consumer commits
separately connect array and static helpers.

Runnable proof:

```sh
cargo test --locked -p ristretto_vm --no-default-features \
  --features experimental-canonical-jit --test canonical_compiled
```

The Java fixture is compiled with `javac --release 17 -g`. A separate ordinary Java
oracle uses `-Xverify:all` and emits ten rows in `compiled-calls-oracle.tsv`; its
main/string/output helpers are excluded from the admitted classes. The tests check
HotSpot and interpreter parity, real native entry/backedges, frame/slice errors,
cancellation and 1,025 live Java frames on the ordinary executor stack.
