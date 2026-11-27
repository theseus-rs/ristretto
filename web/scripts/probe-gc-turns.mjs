// Run against a locally served production build from npm run build:runtime && npm run build.
import { chromium } from '@playwright/test';
import { createHash } from 'node:crypto';
import { readFile, readdir, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const [webRoot, baseUrl, output, ...flags] = process.argv.slice(2);
if (
  !output ||
  flags.some((f) => !['--require-worker-progress', '--require-reclamation'].includes(f))
)
  throw new Error(
    'Usage: node scripts/probe-gc-turns.mjs WEB_ROOT PLAYGROUND_URL OUTPUT.json [--require-worker-progress] [--require-reclamation]',
  );
const timeoutMs = Number(process.env.RISTRETTO_GC_BROWSER_TIMEOUT_MS || 600_000);
if (!Number.isSafeInteger(timeoutMs) || timeoutMs <= 0)
  throw new Error('Expected a positive deadline');
const manifest = JSON.parse(await readFile(resolve(webRoot, 'generated/runtime-manifest.json')));
const workers = (await readdir(resolve(webRoot, 'dist/assets'))).filter((name) =>
  /^runner\.worker-.*\.js$/.test(name),
);
if (workers.length !== 1) throw new Error('Expected one production worker bundle');
const workerHash = createHash('sha256')
  .update(await readFile(resolve(webRoot, 'dist/assets', workers[0])))
  .digest('hex');
const source = await readFile(new URL('./fixtures/GcTurns.java', import.meta.url), 'utf8');
const browser = await chromium.launch();
try {
  const page = await browser.newPage();
  page.on('console', (message) => {
    if (message.text().startsWith('gc-turns ')) console.log(message.text());
  });
  await page.goto(baseUrl);
  const result = await page.evaluate(
    async ({ files, workerName, workerHash, source, timeoutMs, reclamation }) => {
      const assetUrl = (path) => new URL('../' + path, location.href).href;
      const digest = async (bytes) =>
        Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256', bytes)), (n) =>
          n.toString(16).padStart(2, '0'),
        ).join('');
      const assets = await Promise.all(
        Object.entries(files).map(async ([key, entry]) => {
          const bytes = await (await fetch(assetUrl('runtime/' + entry.file))).arrayBuffer();
          if (bytes.byteLength !== entry.size || (await digest(bytes)) !== entry.sha256)
            throw new Error('Asset mismatch: ' + key);
          return [key, new Uint8Array(bytes)];
        }),
      );
      const workerBytes = await (await fetch(assetUrl('assets/' + workerName))).arrayBuffer();
      if ((await digest(workerBytes)) !== workerHash) throw new Error('Worker bundle mismatch');
      const product = URL.createObjectURL(new Blob([workerBytes], { type: 'text/javascript' }));
      // The measurement port is independent of the product request handler.
      const wrapper = URL.createObjectURL(
        new Blob(
          [
            `
      const forward = self.postMessage.bind(self);
      self.postMessage = (event, ...args) => forward({...event, workerT: performance.now()}, ...args);
      const loaded = import(${JSON.stringify(product)});
      self.addEventListener('message', e => {
        if (!e.data.measurementPort) return;
        e.stopImmediatePropagation();
        const port = e.data.measurementPort;
        port.onmessage = ({data}) => port.postMessage({id:data, workerT:performance.now()});
        loaded.then(() => port.postMessage(-1));
      }, {capture:true});
    `,
          ],
          { type: 'text/javascript' },
        ),
      );
      const worker = new Worker(wrapper, { type: 'module' });
      const channel = new MessageChannel();
      const ticks = [],
        frames = [],
        pings = [],
        events = [];
      let frameId, timerId, terminal, finish;
      const done = new Promise((resolve) => {
        finish = resolve;
      });
      const fail = (message) => {
        terminal = { type: 'error', message };
        finish();
      };
      worker.onerror = (e) => fail(e.message);
      let primed;
      const ready = new Promise((resolve) => {
        primed = resolve;
      });
      channel.port1.onmessage = ({ data }) => {
        if (data === -1) {
          primed();
          return;
        }
        pings[data.id].ack = performance.now();
        pings[data.id].workerT = data.workerT;
      };
      worker.onmessage = ({ data }) => {
        events.push({ t: performance.now(), ...data });
        if (data.type === 'phase' || data.type === 'error' || data.type === 'output')
          console.log('gc-turns ' + JSON.stringify(data));
        if (data.type === 'error' || data.type === 'done') {
          terminal = data;
          finish();
        }
      };
      worker.postMessage({ measurementPort: channel.port2 }, [channel.port2]);
      const deadline = setTimeout(() => fail('Worker deadline'), timeoutMs);
      try {
        const startup = setTimeout(() => fail('Worker probe registration timed out'), 30_000);
        await Promise.race([ready, done]);
        clearTimeout(startup);
        console.log('gc-turns worker boot complete');
        if (terminal) throw new Error(terminal.message);
        const tick = () => {
          const t = performance.now();
          ticks.push(t);
          const id = pings.length;
          pings.push({ t, ack: null });
          channel.port1.postMessage(id);
        };
        const frame = (t) => {
          frames.push(t);
          frameId = requestAnimationFrame(frame);
        };
        frameId = requestAnimationFrame(frame);
        timerId = setInterval(tick, 16);
        await new Promise((resolve) => setTimeout(resolve, 200));
        if (!pings.some((p) => p.ack !== null))
          throw new Error('Idle worker probe never acknowledged');
        worker.postMessage(
          {
            request: {
              id: 1,
              action: 'run',
              javaVersion: 25,
              className: 'GcTurns',
              language: 'java',
              source,
              ...(reclamation ? { measureCollections: true } : {}),
            },
            assets,
          },
          assets.map(([, b]) => b.buffer),
        );
        await done;
        // Keep the worker alive to observe replies queued during Java execution.
        await new Promise((resolve) => setTimeout(resolve, 200));
      } catch (error) {
        fail(error instanceof Error ? error.message : String(error));
      } finally {
        clearInterval(timerId);
        clearTimeout(deadline);
        cancelAnimationFrame(frameId);
        worker.terminate();
        channel.port1.close();
        URL.revokeObjectURL(product);
        URL.revokeObjectURL(wrapper);
      }
      return { events, ticks, frames, pings, terminal, visibility: document.visibilityState };
    },
    {
      files: { ...manifest.files, 'jdk.zip': manifest.jdks['25'] },
      reclamation: flags.includes('--require-reclamation'),
      workerName: workers[0],
      timeoutMs,
      workerHash,
      source,
    },
  );
  const report = {
    browser: browser.version(),
    workerHash,
    runtime: manifest.files,
    jdk: manifest.jdks['25'],
    sourceSha256: createHash('sha256').update(source).digest('hex'),
    globalPauseMs: null,
    timeoutMs,
    ...result,
  };
  const begin = result.events.find((e) => e.type === 'output' && e.text.includes('GC_TURNS_BEGIN'));
  const end = result.events.find((e) => e.type === 'output' && e.text.includes('GC_TURNS_END'));
  if (begin && end) {
    const scheduled = result.pings.filter((p) => p.t >= begin.t && p.t < end.t);
    const responses = scheduled.filter(
      (p) => p.workerT >= begin.workerT && p.workerT < end.workerT,
    );
    const times = [
      begin.workerT,
      ...responses.map((p) => p.workerT).sort((a, b) => a - b),
      end.workerT,
    ];
    const gaps = times
      .slice(1)
      .map((t, i) => t - times[i])
      .sort((a, b) => a - b);
    report.measurement = {
      maxWorkerReplyGapMs: gaps.at(-1),
      p95WorkerReplyGapMs: gaps[Math.ceil(0.95 * gaps.length) - 1],
      p99WorkerReplyGapMs: gaps[Math.ceil(0.99 * gaps.length) - 1],
      javaWindowMs: end.workerT - begin.workerT,
      scheduledPings: scheduled.length,
      workerRepliesDuringJava: responses.length,
      unansweredPings: scheduled.filter((p) => p.ack === null).length,
    };
  }
  await writeFile(output, JSON.stringify(report) + '\n', { flag: 'wx' });
  if (result.terminal.type !== 'done') throw new Error(result.terminal.message);
  const stdout = result.events
    .filter((e) => e.type === 'output' && e.stream === 'stdout')
    .map((e) => e.text)
    .join('');
  if (
    !/GC_TURNS_BEGIN/.test(stdout) ||
    !/GC_TURNS_END cycles=[1-9]\d* checksum=799864/.test(stdout)
  )
    throw new Error('Missing collection or checksum evidence: ' + stdout);
  if (flags.includes('--require-reclamation')) {
    const gc = result.terminal.gc;
    if (
      !gc ||
      ![gc.completed, gc.objectsSwept, gc.bytesFreed].every((v) => Number.isSafeInteger(v) && v > 0)
    )
      throw new Error('Guest did not report completed collection, sweeping and byte refunds');
  }
  if (
    flags.includes('--require-worker-progress') &&
    !(report.measurement?.workerRepliesDuringJava > 0)
  )
    throw new Error('No worker reply during the Java measurement window.');
} finally {
  await browser.close();
}
