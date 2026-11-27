# Archived catalog and pool implementation

The alpha uses `ristretto_gc::GarbageCollector` for Java objects, arrays, typed
roots and conditional references on native and WebAssembly. Changes to these
operations belong in that collector. The catalog, pool and restricted Java
runtime were retired in commit `fe7d8c39`.

The original ownership protocols, bounded-poll experiments, test commands and
results are available with the source they exercise.
[Archived port notes](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/docs/CANONICAL_CATALOG_PORT.md)
· [Current JVM heap and migration guide](../../GC_PREVIEW.md)
