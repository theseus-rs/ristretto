# Java in the browser

[Open the playground](https://theseus-rs.github.io/ristretto/playground/) or
[open JShell](https://theseus-rs.github.io/ristretto/jshell/). The dropdown beside **ristretto /**
switches between pages. Everything runs entirely on your device using Ristretto WebAssembly and
a reduced Corretto JDK, with no execution server.

## Build and test

Requirements: the repository's Rust toolchain and Node.js 24 or newer. The JDK installer downloads
the Linux Corretto JDKs pinned with SHA-256 in `jdks.json`. On macOS, also install matching native
JDKs so `jlink` can process the Linux modules.

```sh
rustup target add wasm32-wasip2
cd web
npm ci
npm run install:jdks
# Optional: PLAYGROUND_JDK_ROOT defaults to ~/.ristretto/linux-x64.
# On macOS, PLAYGROUND_JLINK_11 (and _17, _21, _25) can override each native jlink.
npm run build:runtime
npm run build
npx playwright install --with-deps chromium firefox webkit
npm run test:unit
npm run test:shell
npm test
npm run preview
```

Open `http://127.0.0.1:4173/ristretto/jshell/` or
`http://127.0.0.1:4173/ristretto/playground/`. For frontend development, use `npm run dev` after
building the runtime. Rebuild the runtime after Rust or Java bridge changes. Run
`npm run format:check` to check formatting.

Additional checks:

- Set `PLAYGROUND_BASE_URL` to the Java playground URL to test an existing server or deployed site.
- Run `node scripts/smoke-wasi.mjs` from `web` to check all five Java runtimes with Wasmtime.
- Run `npm run test:shell` to check persistent state, mutations, multiline input, commands,
  completion, and error recovery on all four supported JDKs using the browser's component host.
  Append `-- 11`, for example, to check just that runtime.
- Run `cargo test -p ristretto_playground --test protocol` to test the native runner's JSON
  protocol, diagnostics, exceptions, Unicode output, closed stdin, and output limit.

## Runtime and limits

- The first action downloads the selected JDK and a shared WebAssembly engine. Assets are verified
  with SHA-256 and cached in memory, including for offline use. Browser storage retains assets
  across visits when available. Reloading the site still requires a connection.
- Files exist only in memory under `/jdk`, `/workspace`, and `/tmp`. Programs cannot access host
  files or environment variables. Java allocations remain until the VM shuts down.
- Runtime loading is limited to 120 seconds. Java compilation and each JShell submission have
  a 10 minute limit; Java program execution has a 30 second limit. Combined stdout and stderr are
  limited to 1 MiB per request. Exceeding a limit ends the worker and its session.
- Compilation is interpreted and typically takes tens of seconds in Chromium and WebKit, or
  several minutes in Firefox.
- External dependencies, interactive standard input, GUI, networking, and subprocesses are
  unavailable. Standard input returns EOF; program arguments are empty. Some Java APIs are not
  yet implemented.
- Nonzero `System.exit` values are reported as failures without preserving the exit code.
