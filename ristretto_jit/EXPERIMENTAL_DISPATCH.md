# Experimental native activations

The `experimental-reference-dispatch` feature exposes `Compiler::compile_requests`.
The capability includes static scalar methods and opaque reference identities,
with conditional branches and backedges. This slice does not admit Java calls,
allocation, fields, array operations, handlers, synchronization, or integer
division/remainder. The embedding must
verify bytecode before compilation. Unsupported descriptors/opcodes fail closed.

`Compiler::compile` and production `can_compile` keep their existing policy.
A request-only `Function` rejects synchronous `execute`. Begin it with
`function.activation(arguments)`, then call `activation.resume(budget)`:

- `ExecutionStep::Yielded` requests an embedding scheduling boundary. Every taken
  branch to the same or an earlier byte offset consumes one budget unit.
- `ExecutionStep::Returned(value)` completes the invocation. Resuming it again
  fails.

`function.cooperation_budget()` chooses a conservative positive budget from the
instruction count. The budget bounds bytecode work, not elapsed time. Saving and
restoring locals/operands adds bounded work; the compiler rejects excessive
state-transfer expansion. Branch destinations use Java byte offsets.

Each activation owns its code, argument bits and spilled state. No native
frame or borrowed context pointer remains active between calls. Dropping one
cancels it without re-entering generated code. Activations from the same function
have independent state. Argument checks and native-stack rejection occur before
entry; a pre-entry error leaves the activation resumable.

Reference-bearing code requires a non-null owning context and the registered
`jit_publish_frame_roots(context, tokens, count) -> i32` callback. Before each
bytecode and on suspension, generated code publishes live reference locals and
reference operands; a yield uses the destination's live set. The slice contains
opaque i64 tokens, never payload pointers. Publication is capped at 4096 cells.
The callback must validate and own the full replacement before dropping old
roots; zero reports failure and stops generated execution. The host must surface
its stored error and cancel that activation. The return token remains published
until the host has acquired an independent owner.

The context must outlive the activation, suspension and result handoff, and its
Drop must release roots even when the token table is borrowed. This JIT protocol
alone does not implement collector ownership: the canonical frame adapter is a
separate dependent change. `resume` without context rejects a reference-bearing
function before entry. The fallback compiler rejects request-only compilation
on WASI and other unsupported targets.

Run the executable proof and the ordinary regressions:

```sh
cargo test --locked -p ristretto_jit --features experimental-reference-dispatch
cargo test --locked -p ristretto_jit
cargo clippy --locked -p ristretto_jit --all-targets \
  --features experimental-reference-dispatch -- -D warnings
```

The focused `continuation::` tests cover exact yields/resumption, a nonempty
operand stack, long/double argument slots, signed zero/NaN bit preservation,
independent owners, cancellation of a zero-offset loop, stack-error recovery,
state limits and unsupported capability rejection. The scalar cases register no runtime
helper symbols. `frame_roots::` protocol tests register only the publication
callback; they exercise live locals/operands, dead-local release, reference return
handoff, failed publication, missing contexts and the spill-capacity limit.

Request-only static, special, virtual and interface invocations now return a `CallRequest`
with the exact bytecode offset and owner-local constant-pool index. The embedding resolves
access/dispatch/initialization and invokes the callee with no generated caller on the stack.
`call_arguments()` exposes one eight-byte cell per logical parameter, including a receiver;
wide Java parameters still occupy two local slots. Reference cells remain opaque tokens
owned by the published caller frame. No synchronous invocation helper is linked.

Complete a successful request once with `complete_call(bits)`, after the context owns any
reference result. On a Java/embedding error or cancellation, drop the activation; handlers
and exception propagation through native catch blocks are not admitted in this slice.
A pending request cannot resume before completion. `remaining_budget()` carries fuel across
immediately completed requests; a zero remainder requires a host scheduling boundary.
Instance entry is explicit through this request-only API. Ordinary eligibility is unchanged.
