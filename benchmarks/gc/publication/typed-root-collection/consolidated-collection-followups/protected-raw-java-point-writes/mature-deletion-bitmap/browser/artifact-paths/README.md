# Shipping the workspace collector in the playground

The playground builds its WASI engine with Cargo and packages the resulting component into the browser worker. The WASI command smoke runs a companion executable from the same build. Both tools now ask Cargo for its target directory from the workspace root. The regression test covers a relative `[build] target-dir` setting and an overriding `CARGO_TARGET_DIR`.

The release check put invalid files at both default artifact paths in an isolated checkout and selected an external Cargo target directory. The earlier packaging script tried to transpile the stale sentinel. The corrected standard `npm run build:runtime` and `npm run build` commands produced the site using the configured directory. Independently transpiling the chosen component reproduced the SHA-256 hashes of all three packaged Wasm modules. The smoke command ran the configured executable for Java 8, 11, 17, 21 and 25.

The combined revision `8313a0cd` passed in three fresh Chromium workers. Each completed 13 guest collections, reclaimed 10,473,075 managed bytes and returned checksum 799864. Every scheduled worker ping was answered. The same check passed on the packaging fix against the prior collector revision. Worker reply timing includes Java execution, scheduling and browser work. Global pause duration was unmeasured.
[Integrated artifact and worker hashes](integrated.json) · [Before the bitmap](before-bitmap.json) · [Stale-file control](parent-stale-component.log.gz) · [Five WASI runs](wasi-command.log.gz) · [Web unit tests](web-tests.log.gz)
