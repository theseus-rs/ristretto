# Java playground

[Open the playground](https://theseus-rs.github.io/ristretto/playground/).

Select Java **8, 11, 17, 21, or 25** to compile and run one source file entirely in your browser. The editor supports packages, nested classes, compiler diagnostics, separate stdout/stderr styling, examples, local autosave, and immediate cancellation. Set **Main class** to the fully qualified class containing `public static void main(String[] args)`.

There is no execution server. Each action creates an isolated Web Worker containing Ristretto compiled to WebAssembly and a reduced Corretto JDK. Compilation uses the real Java compiler with annotation processing disabled. Run recompiles the current source and loads every generated class into a fresh VM. Standard input returns EOF; program arguments are empty.

## Build and test

Requirements: the repository's Rust toolchain and Node.js 24 or newer. `npm run install:jdks` installs the five Linux Corretto JDKs pinned with SHA-256 in `jdks.json`. On macOS, also install matching native JDKs for Java 11/17/21/25 to run `jlink` against the Linux modules.

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

Open `http://127.0.0.1:4173/ristretto/playground/`. For frontend development, run `npm run dev` after building the runtime. Rust changes require `npm run build:runtime` again. Generated assets and `node_modules` are ignored by Git. `npm run format:check` checks frontend formatting.

The optional `node scripts/smoke-wasi.mjs` check uses Wasmtime with all five JDKs to exercise compilation, Unicode, reference identity, directory iteration, canonical paths, missing-file errors, and closed stdin without a browser. Set `PLAYGROUND_BASE_URL` to run browser tests against an existing server or the deployed site instead of starting a local preview.

`cargo test -p ristretto_playground --test protocol` exercises the runner executable's JSON protocol, compiler diagnostics, runtime exceptions, Unicode output, closed stdin, and output limit. These native process tests also contribute runner coverage to the existing Rust coverage job.

## Runtime and limits

- Java 8 bundles its runtime JARs and `tools.jar`. For Java 11 and newer, the pinned JDK roots are `java.base`, `java.compiler`, `jdk.compiler`, and `jdk.zipfs`. `jlink` resolves their dependencies. Its resource compression plugin is disabled because Ristretto reads uncompressed jimage entries; the downloadable archive is ZIP-compressed instead.
- The first action for each selected version downloads that JDK and shares the same WASM engine; the build reports its exact size. Every asset is named by its content hash and verified with SHA-256. Verified assets and the worker script remain in memory for subsequent runs, including offline runs. Cache Storage also retains assets across visits when available; unavailable or full storage falls back to fetching. Reloading the site still requires a connection.
- The filesystem contains only the bundled `/jdk`, the request in `/workspace`, and `/tmp`. These are in-memory files in a fresh worker. Guest HTTP and sockets are denied. No host files or environment variables are exposed.
- Runtime loading has a 120-second limit and compilation has a 600-second limit; execution has a 30-second limit. Combined program output is limited to 1 MiB. Stop and time limits terminate the worker, including infinite loops.
- External dependencies, interactive stdin, GUI, networking, and subprocesses are unavailable. Ristretto does not yet implement every Java API. Compilation is interpreted. It typically takes tens of seconds in Chromium and WebKit and can take several minutes in Firefox. The compilation limit accommodates that measured difference.
- Java allocations are retained until their VM shuts down because VM stacks and native Rust containers are not yet a complete garbage-collector root set. Each playground action releases its worker and VMs on completion. Native embedders that reuse a `Compiler` retain those allocations for the compiler VM’s lifetime.
- WASI represents process exit as success or failure; nonzero `System.exit` values are reported as unsuccessful exits without preserving the original number.
- Core Rust changes implement WASI canonical paths, directory iteration, access checks, needed by `javac`, defer module descriptors until system bootstrap to preserve Java 25 UTF-16 hashing, correct reference identity in atomic compare-and-set and `Reference.refersTo`, register generated method-handle classes, and decode compact UTF-16 strings in native byte order. The browser shim's regular-file `readlinkAt` behavior is adapted to return WASI `invalid`, as required by canonicalization.

The selected version controls both the compiler and runtime, and is autosaved with the source. Examples that require a newer Java version are disabled; switching versions preserves your code. The runner reads a single JSON request and writes NDJSON events with a request ID. Java writes are represented as byte arrays to preserve UTF-8 split across writes. The worker decodes the protocol; the UI only renders text. The UI ignores events from obsolete workers.

## GitHub Pages

`.github/workflows/web.yml` builds the runtime and frontend, tests the actual compiled Java examples in independent Chromium, Firefox, and WebKit jobs, builds the existing Oranda docs, and copies the playground into `public/playground/`. Only successful builds on `main` deploy to `playground-pages`. Pull requests run the same checks without deploying. The dedicated publication branch prevents older documentation workflows that still target `gh-pages` from replacing the playground during migration.

The Pages configuration must use **Deploy from a branch → playground-pages → / (root)**. The Vite base is `/ristretto/playground/`; update it and the Playwright base URL if the repository is renamed. All executable resources are static files served from the Pages origin. No custom headers, SharedArrayBuffer, server, or secret is required.

JDK legal notices are retained in the runtime ZIP. The deployed [notices page](public/notices.html) links to the upstream sources and generated license bundle.

The workspace uses nearly the full browser width. Appearance follows the system by default;
the Theme selector can override it with Light or Dark, stored only in this browser's local storage.
Choose System to resume automatic switching. Editor colors follow the current
[IntelliJ Light and Dark schemes](https://github.com/JetBrains/intellij-community/tree/04b6fcde0aa6b01a4ccb536c137e8594274ab356/platform/platform-resources/src/themes/expUI),
with CodeMirror syntax highlighting rather than IntelliJ semantic analysis.
