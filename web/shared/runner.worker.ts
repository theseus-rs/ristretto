/// <reference lib="webworker" />
import { createEngine } from './engine';
import type { Request } from './protocol';

const scope = self as unknown as DedicatedWorkerGlobalScope;
let engine: Awaited<ReturnType<typeof createEngine>> | undefined;
let busy = false;
scope.onmessage = async ({
  data: { request, assets },
}: MessageEvent<{
  request: Request;
  assets?: [string, Uint8Array<ArrayBuffer>][];
}>) => {
  if (busy) return;
  busy = true;
  try {
    if (!engine) {
      if (!assets) throw new Error('Missing Java runtime assets. Restart the session.');
      scope.postMessage({ id: request.id, type: 'phase', phase: 'loading' });
      engine = await createEngine(assets, (event) => scope.postMessage(event));
    }
    engine.execute(request);
  } catch (error) {
    engine = undefined;
    scope.postMessage({
      id: request.id,
      type: 'error',
      message: error instanceof Error ? error.message : String(error),
    });
  } finally {
    busy = false;
  }
};
