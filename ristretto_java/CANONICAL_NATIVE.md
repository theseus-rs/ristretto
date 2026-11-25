# Explicit application native boundary

The opt-in `experimental-canonical-native` feature enables one launcher binding:

```text
--canonical-native CLASS.METHOD=TARGET.METHOD
--canonical-class BINARY_NAME
```

The bridge must be a declared application `static native int method(int)`.
The target must be an admitted `static int method(int)`. Repeat
`--canonical-class` for companion classes required by the target. Class names
may use dots or slashes; nested-class names retain `$`. Method descriptors are
fixed to `(I)I` and are not part of the option syntax. The VM validates the
actual declarations and target bytecode before invoking the ordinary main.

This is an explicit boundary into the restricted canonical interpreter. JVM
bootstrap, the main class, argument processing, stdout, ordinary class
initialization and other native resolution still use the normal VM. Canonical
object references do not cross the boundary; only an integer does. Adding the
feature does not bind any native automatically. One binding is supported per
VM; multiple bindings and orphan companion options are rejected.

Compile the ordinary caller and admitted target fixture using a JDK 25 home:

```sh
export RISTRETTO_JDK_HOME="$JAVA_HOME"
mkdir -p /tmp/canonical-native-classes
"$JAVA_HOME/bin/javac" --release 8 -Xlint:-options \
  -d /tmp/canonical-native-classes \
  ristretto_java/tests/fixtures/canonical_native/bridge/Caller.java \
  ristretto_java/tests/fixtures/canonical_native/bridge/Target.java \
  ristretto_java/tests/fixtures/canonical_native/bridge/StatefulTarget.java
cargo run --locked -p ristretto_java --no-default-features \
  --features experimental-canonical-native -- \
  -Xint -cp /tmp/canonical-native-classes \
  --canonical-native bridge.Caller.enter=bridge.Target.compute \
  --canonical-class 'bridge.Target$Node' \
  bridge.Caller hello --canonical-native=application-argument
```

Expected output is `ordinary-before:hello`,
`ordinary-caught:NegativeArraySizeException`, then `ordinary-after:25,37`. The caller's ordinary static initializer uses
`System.setProperty`; its normal main checks the property and its arguments,
then invokes the bridge twice successfully. Each successful target call constructs
a cycle and a primitive array and returns a known integer. A third call uses a
negative array length; ordinary Java catches the real runtime exception returned
across the boundary. The negative controls also reject a mutable-static target
during admission, before main runs. The final argument deliberately
looks like a launcher option and must reach main unchanged.

The same ordinary launcher and precompiled classpath can run on WASI:

```sh
cargo build --locked -p ristretto_java --no-default-features \
  --features experimental-canonical-native --target wasm32-wasip2
wasmtime run -W max-wasm-stack=8388608 -S http --dir "$JAVA_HOME::/jdk" \
  --dir /tmp/canonical-native-classes::/classes \
  --env RISTRETTO_JDK_HOME=/jdk --env JAVA_VERSION=25.0.5.0.10 \
  target/wasm32-wasip2/debug/java.wasm \
  -Xint -cp /classes \
  --canonical-native bridge.Caller.enter=bridge.Target.compute \
  --canonical-class 'bridge.Target$Node' \
  bridge.Caller hello --canonical-native=application-argument
```

Use a source-specific absolute `CARGO_TARGET_DIR` for isolated builds and adjust
the module path accordingly. This interface uses precompiled application
classes. It does not change unrestricted bytecode admission or provide native
compiled canonical execution. Collector progress, complete stall coverage,
observer overhead and sustained throughput/tail qualification remain separate
requirements; this finite fixture makes no performance claim.
