# Static requests and initialization handoff

The request-only compiler already emits `GetStatic` (internal tag 10, no outgoing
cells) and `PutStatic` (tag 11, one value cell). No additional initialization tag
or synchronous runtime helper is needed. An `i32` value is sign-extended into its
eight-byte request cell; references are opaque tokens owned by the embedding's
published frame and temporary result snapshots. Request-only code links only
`jit_publish_frame_roots`; host faults cancel the parked activation without a
synchronous pending-exception callback. Ordinary continuation code retains its
pending-exception and handler protocol.

This change validates static Fieldref/Class/NameAndType/UTF8 metadata and the
complete field descriptor before request-only native compilation. It does not
change production eligibility or the ordinary experimental policy. All previously
valid generic field representations remain admitted, including scalar types and
array descriptors that a particular embedding may reject.

The canonical VM contract accepts static int and its closed reference layouts,
enforces static/instance opcode kind, member/class access, quotas, final-write
location and unsupported volatile/scalar/layout rejection before execution. The
generic compiler cannot establish those rules for external classes. The VM's
existing explicit rejection of static-bearing compiled programs remains necessary
until its driver implements the following handoff.

1. Resolve the requesting instruction against the owning method's linked metadata.
   `New`, static field requests and static calls trigger the resolved declaring
   class. Reference-array allocation and casts do not initialize component classes.
2. Keep the requesting activation, frame snapshot, outgoing arguments and any
   temporary result roots owned while the host prepares initialization. Never
   hold a frame-table/RefCell borrow across a suspension.
3. Drive the exact initializer identified by the registry-bound, single-use
   `PreparedInitialization` lease using another activation and the normal frame,
   request and native-slice quotas. The requesting activation remains parked;
   resumption before `complete_call` is rejected. Initializer frame publication
   must not replace the requesting frame's owners.
4. Consume the lease and initializer outcome through the VM's existing finish
   operation. Only after success may the host finish the waiting allocation,
   field access or call and publish/transfer its result before native resumption.
   Reference results must be published before a later bytecode can use them.
5. On failure or cancellation, use typed VM owner teardown and lease poisoning.
   Do not silently retry initialization, reinterpret a legacy RuntimeContext,
   or release the requesting roots while its pending operation can still use them.

The frozen VM foundation is invocation-owned and uses one local execution strand.
Its recursive `Initializing` rule does not establish shared-heap or cross-thread
initialization semantics. Persistent/shared heaps require separate owner/wait
identity rules. External direct `<clinit>` calls remain a VM admission restriction;
the compiler can prepare `<clinit>()V` for its internal initialization driver.

Tests cover signed static cells, exact CP/BCI/arity, outgoing references after
locals are cleared, nulls, context relocation, result publication failure, parked
callers with separate initializer activations, error/cancellation, malformed
metadata, and preserved generic capabilities. Exact javac source/class copies from
the frozen VM static fixture are under `tests/fixtures/canonical_static`; their
`contract.json` records hashes and origin. Every method is compiled with only the
two typed callbacks, and both real 9-slot and 2048-slot initializers are driven
through their allocation/static-store requests. These are compiler protocol proofs;
canonical heap ownership, Java initialization state and end-to-end execution remain
VM integration responsibilities, with no performance or GC qualification claim.
