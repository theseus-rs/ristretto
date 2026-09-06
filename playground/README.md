# Java playground

[Open the playground](https://theseus-rs.github.io/ristretto/playground/).

Compile and run a single Java source file entirely in your browser. Choose an example or write
your own code, then set **Main class** to the fully qualified class with
`public static void main(String[] args)`. Packages and nested classes are supported.

The selected version controls both the compiler and runtime. Your source and version are saved
locally; switching versions preserves your code. Examples that need a newer Java version are
disabled.

Each action runs Ristretto WebAssembly and a reduced Corretto JDK in an isolated Web Worker, with no
execution server. **Run** recompiles the source with the Java compiler and loads all generated
classes into a fresh VM. Annotation processing is disabled. **Stop** immediately terminates the
worker.

## Build and test

Requirements: the repository's Rust toolchain and Node.js 24 or newer. The JDK installer downloads
the Linux Corretto JDKs pinned with SHA-256 in `jdks.json`. On macOS, also install matching native
JDKs so `jlink` can process the Linux modules.

```sh
rustup target add wasm32-wasip2
cd playground
npm ci
npm run install:jdks
# Optional: PLAYGROUND_JDK_ROOT defaults to ~/.ristretto/linux-x64.
# On macOS, PLAYGROUND_JLINK_11 (and _17, _21, _25) can override each native jlink.
npm run build:runtime
npm run build
npx playwright install --with-deps chromium firefox webkit
npm run test:unit
npm test
npm run preview
```

Open `http://127.0.0.1:4173/ristretto/playground/`. For frontend development, use `npm run dev`
after building the runtime. Rebuild the runtime after Rust changes. Run `npm run format:check` to
check formatting.

Additional checks:

- Set `PLAYGROUND_BASE_URL` to run browser tests against an existing server or the deployed site.
- Run `node scripts/smoke-wasi.mjs` from `playground` to check all five JDKs with Wasmtime, without
  a browser.
- Run `cargo test -p ristretto_playground --test protocol` to test the runner's JSON protocol,
  diagnostics, exceptions, Unicode output, closed stdin, and output limit.

## Runtime and limits

- The first action downloads the selected JDK and a shared WebAssembly engine. Assets are verified
  with SHA-256 and cached in memory for later runs, including offline runs. Browser storage retains
  assets across visits when available. Reloading the site still requires a connection.
- Files exist only in memory under `/jdk`, `/workspace`, and `/tmp`. Programs cannot access host
  files or environment variables. Java allocations are retained until the VM shuts down; each action
  releases its worker and VMs on completion.
- Runtime loading, compilation, and execution have limits of 120, 600, and 30 seconds, respectively.
  Combined stdout and stderr are limited to 1 MiB. Time limits terminate the worker, even in loops.
- Compilation is interpreted and typically takes tens of seconds in Chromium and WebKit, or several
  minutes in Firefox.
- External dependencies, interactive input, GUI, networking, and subprocesses are unavailable.
  Standard input returns EOF, and program arguments are empty. Some Java APIs are not yet
  implemented.
- Nonzero `System.exit` values are reported as failures without preserving the exit code.
