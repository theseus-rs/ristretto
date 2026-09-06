// world root:component/root
import type * as WasiCliEnvironment from './interfaces/wasi-cli-environment.js'; // wasi:cli/environment@0.2.12
import type * as WasiCliExit from './interfaces/wasi-cli-exit.js'; // wasi:cli/exit@0.2.12
import type * as WasiCliStderr from './interfaces/wasi-cli-stderr.js'; // wasi:cli/stderr@0.2.12
import type * as WasiCliStdin from './interfaces/wasi-cli-stdin.js'; // wasi:cli/stdin@0.2.12
import type * as WasiCliStdout from './interfaces/wasi-cli-stdout.js'; // wasi:cli/stdout@0.2.12
import type * as WasiCliTerminalInput from './interfaces/wasi-cli-terminal-input.js'; // wasi:cli/terminal-input@0.2.12
import type * as WasiCliTerminalOutput from './interfaces/wasi-cli-terminal-output.js'; // wasi:cli/terminal-output@0.2.12
import type * as WasiCliTerminalStderr from './interfaces/wasi-cli-terminal-stderr.js'; // wasi:cli/terminal-stderr@0.2.12
import type * as WasiCliTerminalStdin from './interfaces/wasi-cli-terminal-stdin.js'; // wasi:cli/terminal-stdin@0.2.12
import type * as WasiCliTerminalStdout from './interfaces/wasi-cli-terminal-stdout.js'; // wasi:cli/terminal-stdout@0.2.12
import type * as WasiClocksMonotonicClock from './interfaces/wasi-clocks-monotonic-clock.js'; // wasi:clocks/monotonic-clock@0.2.12
import type * as WasiClocksWallClock from './interfaces/wasi-clocks-wall-clock.js'; // wasi:clocks/wall-clock@0.2.12
import type * as WasiFilesystemPreopens from './interfaces/wasi-filesystem-preopens.js'; // wasi:filesystem/preopens@0.2.12
import type * as WasiFilesystemTypes from './interfaces/wasi-filesystem-types.js'; // wasi:filesystem/types@0.2.12
import type * as WasiHttpOutgoingHandler from './interfaces/wasi-http-outgoing-handler.js'; // wasi:http/outgoing-handler@0.2.12
import type * as WasiHttpTypes from './interfaces/wasi-http-types.js'; // wasi:http/types@0.2.12
import type * as WasiIoError from './interfaces/wasi-io-error.js'; // wasi:io/error@0.2.12
import type * as WasiIoPoll from './interfaces/wasi-io-poll.js'; // wasi:io/poll@0.2.12
import type * as WasiIoStreams from './interfaces/wasi-io-streams.js'; // wasi:io/streams@0.2.12
import type * as WasiRandomInsecureSeed from './interfaces/wasi-random-insecure-seed.js'; // wasi:random/insecure-seed@0.2.12
import type * as WasiRandomRandom from './interfaces/wasi-random-random.js'; // wasi:random/random@0.2.12
import type * as WasiCliRun from './interfaces/wasi-cli-run.js'; // wasi:cli/run@0.2.0
export interface ImportObject {
  'wasi:cli/environment@0.2.12': typeof WasiCliEnvironment,
  'wasi:cli/exit@0.2.12': typeof WasiCliExit,
  'wasi:cli/stderr@0.2.12': typeof WasiCliStderr,
  'wasi:cli/stdin@0.2.12': typeof WasiCliStdin,
  'wasi:cli/stdout@0.2.12': typeof WasiCliStdout,
  'wasi:cli/terminal-input@0.2.12': typeof WasiCliTerminalInput,
  'wasi:cli/terminal-output@0.2.12': typeof WasiCliTerminalOutput,
  'wasi:cli/terminal-stderr@0.2.12': typeof WasiCliTerminalStderr,
  'wasi:cli/terminal-stdin@0.2.12': typeof WasiCliTerminalStdin,
  'wasi:cli/terminal-stdout@0.2.12': typeof WasiCliTerminalStdout,
  'wasi:clocks/monotonic-clock@0.2.12': typeof WasiClocksMonotonicClock,
  'wasi:clocks/wall-clock@0.2.12': typeof WasiClocksWallClock,
  'wasi:filesystem/preopens@0.2.12': typeof WasiFilesystemPreopens,
  'wasi:filesystem/types@0.2.12': typeof WasiFilesystemTypes,
  'wasi:http/outgoing-handler@0.2.12': typeof WasiHttpOutgoingHandler,
  'wasi:http/types@0.2.12': typeof WasiHttpTypes,
  'wasi:io/error@0.2.12': typeof WasiIoError,
  'wasi:io/poll@0.2.12': typeof WasiIoPoll,
  'wasi:io/streams@0.2.12': typeof WasiIoStreams,
  'wasi:random/insecure-seed@0.2.12': typeof WasiRandomInsecureSeed,
  'wasi:random/random@0.2.12': typeof WasiRandomRandom,
}
export interface Root {
  'wasi:cli/run@0.2.0': typeof WasiCliRun,
  run: typeof WasiCliRun,
}

/**
* Instantiates this component with the provided imports and
* returns a map of all the exports of the component.
*
* This function is intended to be similar to the
* `WebAssembly.Instantiate` constructor. The second `imports`
* argument is the "import object" for wasm, except here it
* uses component-model-layer types instead of core wasm
* integers/numbers/etc.
*
* The first argument to this function, `getCoreModule`, is
* used to compile core wasm modules within the component.
* Components are composed of core wasm modules and this callback
* will be invoked per core wasm module. The caller of this
* function is responsible for reading the core wasm module
* identified by `path` and returning its compiled
* `WebAssembly.Module` object. This would use the
* `WebAssembly.Module` constructor on the web, for example.
*/
export function instantiate(
getCoreModule: (path: string) => WebAssembly.Module,
imports: ImportObject,
instantiateCore?: (module: WebAssembly.Module, imports: Record<string, any>) => WebAssembly.Instance
): Root;
export function instantiate(
getCoreModule: (path: string) => WebAssembly.Module | Promise<WebAssembly.Module>,
imports: ImportObject,
instantiateCore?: (module: WebAssembly.Module, imports: Record<string, any>) => WebAssembly.Instance | Promise<WebAssembly.Instance>
): Root | Promise<Root>;

