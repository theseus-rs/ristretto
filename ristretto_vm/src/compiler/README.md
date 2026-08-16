# Embedded compiler bridge

`java/org/ristretto/compiler/CompilerBridge.java` is a Java 8-compatible bridge to the standard
`javax.tools` compiler API. Its generated class files are checked in under `classes/` and embedded
in `ristretto_vm` with `include_bytes!`.

Regenerate the assets from the repository root with a JDK 8 `javac`:

```shell
javac -source 8 -target 8 \
  -d ristretto_vm/src/compiler/classes \
  ristretto_vm/src/compiler/java/org/ristretto/compiler/CompilerBridge.java
```
