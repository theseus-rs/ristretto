import { cli, filesystem, http } from '@bytecodealliance/preview2-shim';
import { WASIShim } from '@bytecodealliance/preview2-shim/instantiation';
import { unzipSync } from 'fflate';
import { instantiate, type ImportObject } from '../generated/runner.js';
import { EventDecoder, OUTPUT_LIMIT, type Event, type Request } from './protocol';

const encoder = new TextEncoder();
type Assets = [string, Uint8Array<ArrayBuffer>][];

const openAt = filesystem.types.Descriptor.prototype.openAt;
const statAt = filesystem.types.Descriptor.prototype.statAt;
// The shim reports ENOTDIR when an intermediate directory is missing. Java libraries
// rely on ENOENT to distinguish an absent configuration file from a broken path.
filesystem.types.Descriptor.prototype.openAt = function (...args) {
  try {
    return openAt.apply(this, args);
  } catch (error) {
    if (error === 'not-directory') {
      const [flags, path] = args;
      const segments = path.split('/');
      for (let count = 1; count < segments.length; count++) {
        try {
          statAt.call(this, flags, segments.slice(0, count).join('/') || '.');
        } catch (parentError) {
          if (parentError === 'no-entry') throw parentError;
          break;
        }
      }
    }
    throw error;
  }
};

/** Instantiate once per worker. Repeated calls retain JShell's VM and in-memory files. */
export async function createEngine(assets: Assets, emit: (event: Event) => void) {
  let request: Request;
  let executing = false;
  let terminal: Event | undefined;
  let stopped = false;
  let outputSize = 0;
  const send = (event: Event) => {
    if (terminal || event.id !== request?.id) return;
    if (event.type === 'output') {
      outputSize += encoder.encode(event.text).length;
      if (outputSize > OUTPUT_LIMIT) throw new Error('Output exceeded 1 MiB; session stopped.');
    }
    if (['error', 'done', 'ready', 'completions'].includes(event.type)) {
      terminal = event;
      return;
    }
    emit(event);
  };
  const decoder = new EventDecoder(send);
  const files = new Map(assets);
  type Tree = { dir?: Record<string, Tree>; source?: Uint8Array };
  function archive(bytes: Uint8Array): Tree {
    const tree: Tree = { dir: Object.create(null) };
    for (const [path, source] of Object.entries(unzipSync(bytes))) {
      const names = path.split('/').filter(Boolean);
      if (names.some((name) => name === '..' || name === '__proto__'))
        throw new Error('Invalid runtime archive path');
      let directory = tree;
      for (const name of names.slice(0, -1))
        directory = directory.dir![name] ??= { dir: Object.create(null) };
      if (!path.endsWith('/')) directory.dir![names.at(-1)!] = { source };
    }
    return tree;
  }
  const root: Tree = {
    dir: {
      jdk: archive(files.get('jdk.zip')!),
      languages: files.has('language.zip') ? archive(files.get('language.zip')!) : { dir: {} },
      workspace: { dir: {} },
      tmp: { dir: {} },
    },
  };
  filesystem._setFileData(root);
  filesystem._setPreopens({ '/': root });
  // The published shim returns an empty symlink target for regular files. WASI canonicalize
  // requires EINVAL instead. Our runtime archive dereferences every link at build time.
  filesystem.types.Descriptor.prototype.readlinkAt = () => {
    throw 'invalid';
  };
  cli._setCwd('/workspace');
  cli._setEnv({});
  cli._setArgs(['ristretto_playground']);
  cli._setStdin({
    blockingRead() {
      throw { tag: 'closed' };
    },
  });
  cli._setStdout({ write: (bytes) => decoder.push(bytes) });
  const stderr = new TextDecoder();
  cli._setStderr({
    write: (bytes) =>
      send({
        id: request.id,
        type: 'output',
        stream: 'stderr',
        text: stderr.decode(bytes, { stream: true }),
      }),
  });
  const shim = new WASIShim({
    sandbox: { enableNetwork: false },
    http: {
      ...http,
      outgoingHandler: {
        handle() {
          throw { tag: 'http-request-denied' };
        },
      },
    },
  });
  // Jco's JS uses unversioned keys; its declarations also include versioned aliases.
  const imports = {
    ...shim.getImportObject(),
    ...shim.getImportObject({ asVersion: '0.2.12' }),
  } as unknown as ImportObject;
  const component = await instantiate(async (name) => {
    const bytes = files.get(name);
    if (!bytes) throw new Error(`Missing runtime module: ${name}`);
    return WebAssembly.compile(bytes);
  }, imports);
  return {
    async execute(next: Request) {
      if (stopped) {
        emit({ id: next.id, type: 'error', message: 'Java session stopped. Restart the session.' });
        return false;
      }
      if (executing) {
        if (request.id !== next.id) {
          emit({ id: next.id, type: 'error', message: 'A Java request is already running.' });
        }
        return true;
      }
      executing = true;
      request = next;
      terminal = undefined;
      outputSize = 0;
      try {
        if (!component.startRequest(JSON.stringify(request))) {
          throw new Error('A Java request is already running.');
        }
        const turns = new MessageChannel();
        try {
          while (!component.pollRequest()) {
            await new Promise<void>((resume) => {
              turns.port1.onmessage = () => resume();
              turns.port2.postMessage(null);
            });
          }
        } finally {
          turns.port1.close();
          turns.port2.close();
        }
        decoder.flush(request.id);
        if (!terminal) throw new Error('Java exited without a result.');
      } catch (error) {
        // A host exit or trap bypasses the guest's request-slot cleanup.
        stopped = true;
        if (
          typeof error === 'object' &&
          error !== null &&
          'exitError' in error &&
          'code' in error
        ) {
          const exitCode = Number(error.code);
          if (exitCode === 0) {
            send({ id: request.id, type: 'done', exitCode });
          } else {
            send({
              id: request.id,
              type: 'error',
              message: `Java exited unsuccessfully (code ${exitCode}).`,
            });
          }
        } else {
          send({
            id: request.id,
            type: 'error',
            message: error instanceof Error ? error.message : String(error),
          });
        }
      } finally {
        executing = false;
      }
      // A caller may submit the next JShell input as soon as it receives ready.
      if (terminal) emit(terminal);
      return !stopped;
    },
  };
}
