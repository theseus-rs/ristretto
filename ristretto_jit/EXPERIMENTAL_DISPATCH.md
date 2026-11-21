# Experimental native activations

The `experimental-reference-dispatch` feature exposes `Compiler::compile_requests`.
The initial capability is scalar, static Java methods with conditional branches
and backedges. The feature name reserves the later owning-reference protocol;
this slice does not admit references, Java calls, allocation, fields, arrays,
handlers, synchronization, or integer division/remainder. The embedding must
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

Each activation owns its code, arguments and spilled scalar state. No native
frame or borrowed context pointer remains active between calls. Dropping one
cancels it without re-entering generated code. Activations from the same function
have independent state. Argument checks and native-stack rejection occur before
entry; a pre-entry error leaves the activation resumable.

This ABI provides no collector root guarantee yet. Reference admission and a
canonical frame owner are a separate dependent change. The fallback compiler
rejects request-only compilation on WASI and other unsupported targets.

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
state limits and unsupported capability rejection. They register no runtime
helper symbols.
