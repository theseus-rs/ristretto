import type { JavaVersion } from './protocol';
import manifest from '../generated/runtime-manifest.json';

const assetRoot = new URL(`${import.meta.env.BASE_URL}runtime/`, location.origin);
const cacheName = 'ristretto-playground-assets-v2';
type Progress = { loaded: number; total: number };
type Asset = { file: string; sha256: string; size: number };
const retained = new Map<string, Promise<Uint8Array<ArrayBuffer>>>();

/** Fetch only the selected JDK and share verified WASM assets across versions. */
export async function loadRuntime(version: JavaVersion, onProgress: (progress: Progress) => void) {
  const files: Record<string, Asset> = { ...manifest.files, 'jdk.zip': manifest.jdks[version] };
  const loaded = new Map<string, number>();
  const total = Object.values(files).reduce((sum, entry) => sum + entry.size, 0);
  return Promise.all(
    Object.entries(files).map(async ([name, entry]) => {
      const report = (size: number) => {
        loaded.set(name, size);
        onProgress({ loaded: [...loaded.values()].reduce((a, b) => a + b, 0), total });
      };
      let asset = retained.get(entry.sha256);
      if (!asset) {
        asset = fetchAsset(entry, report).catch((error) => {
          retained.delete(entry.sha256);
          throw error;
        });
        retained.set(entry.sha256, asset);
      }
      const bytes = await asset;
      report(bytes.length);
      return [name, bytes] as [string, Uint8Array<ArrayBuffer>];
    }),
  );
}
async function fetchAsset(entry: Asset, report: (size: number) => void) {
  let cache: Cache | undefined;
  try {
    cache = await caches.open(cacheName);
  } catch {
    /* Browser storage is optional. */
  }
  const url = new URL(entry.file, assetRoot).href;
  let response: Response | undefined;
  try {
    response = await cache?.match(url);
  } catch {
    /* Fetch when cache reads fail. */
  }
  const cached = !!response;
  response ??= await fetch(url);
  if (!response.ok) throw new Error(`Could not load Java runtime (${response.status}). Try again.`);
  const chunks: Uint8Array[] = [];
  const reader = response.body?.getReader();
  if (!reader) throw new Error('The browser could not read a runtime asset.');
  let size = 0;
  while (true) {
    const { done, value } = await reader.read();
    if (done) break;
    chunks.push(value);
    size += value.byteLength;
    report(size);
  }
  const bytes = new Uint8Array(size);
  let offset = 0;
  for (const chunk of chunks) {
    bytes.set(chunk, offset);
    offset += chunk.length;
  }
  const digest = await crypto.subtle.digest('SHA-256', bytes);
  const hash = [...new Uint8Array(digest)]
    .map((byte) => byte.toString(16).padStart(2, '0'))
    .join('');
  if (size !== entry.size || hash !== entry.sha256) {
    try {
      await cache?.delete(url);
    } catch {
      /* Retry can fetch a fresh asset. */
    }
    throw new Error('The downloaded Java runtime was incomplete. Try again.');
  }
  if (!cached) {
    try {
      await cache?.put(url, new Response(bytes));
    } catch {
      /* Continue without persistent cache. */
    }
  }
  return bytes;
}
