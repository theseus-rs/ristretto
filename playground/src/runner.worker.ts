/// <reference lib="webworker" />
import { cli, filesystem, http } from '@bytecodealliance/preview2-shim';
import { WASIShim } from '@bytecodealliance/preview2-shim/instantiation';
import { unzipSync } from 'fflate';
import { instantiate, type ImportObject } from '../generated/runner.js';
import { EventDecoder, OUTPUT_LIMIT, type Event, type Request } from './protocol';

const scope = self as unknown as DedicatedWorkerGlobalScope;
const encoder = new TextEncoder();

scope.onmessage = async ({
  data: { request, assets },
}: MessageEvent<{ request: Request; assets: [string, Uint8Array<ArrayBuffer>][] }>) => {
  let terminal = false;
  let outputSize = 0;
  const send = (event: Event) => {
    if (terminal || event.id !== request.id) return;
    if (event.type === 'output') {
      outputSize += encoder.encode(event.text).length;
      if (outputSize > OUTPUT_LIMIT) throw new Error('Output exceeded 1 MiB; execution stopped.');
    }
    if (event.type === 'error' || event.type === 'done') terminal = true;
    scope.postMessage(event);
  };
  const decoder = new EventDecoder(send);
  try {
    send({ id: request.id, type: 'phase', phase: 'loading' });
    const files = new Map(assets);
    type Tree = { dir?: Record<string, Tree>; source?: Uint8Array };
    const jdk: Tree = { dir: Object.create(null) };
    for (const [path, source] of Object.entries(unzipSync(files.get('jdk.zip')!))) {
      const names = path.split('/').filter(Boolean);
      if (names.some((name) => name === '..' || name === '__proto__'))
        throw new Error('Invalid runtime archive path');
      let directory = jdk;
      for (const name of names.slice(0, -1))
        directory = directory.dir![name] ??= { dir: Object.create(null) };
      if (!path.endsWith('/')) directory.dir![names.at(-1)!] = { source };
    }
    const root: Tree = {
      dir: {
        jdk,
        workspace: { dir: { 'request.json': { source: encoder.encode(JSON.stringify(request)) } } },
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
    component.run.run();
    decoder.flush(request.id);
    if (!terminal) throw new Error('Java exited without a result.');
  } catch (error) {
    decoder.flush(request.id);
    if (typeof error === 'object' && error !== null && 'exitError' in error && 'code' in error) {
      const code = Number(error.code);
      if (code === 0) send({ id: request.id, type: 'done', exitCode: code });
      else
        send({
          id: request.id,
          type: 'error',
          message:
            'Program exited unsuccessfully (WASI reports status 1 for any nonzero exit code).',
        });
    } else {
      send({
        id: request.id,
        type: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  }
};
