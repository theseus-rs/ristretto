// Local browser qualification fixture. No dependencies or external requests.
const parameters = new URLSearchParams(location.search);
const options = JSON.parse(parameters.get("options"));
const token = parameters.get("token");
const FAILED = 0xffffffff;

class Histogram {
  constructor() { this.bins = new Uint32Array(4096); this.count = 0; this.max_ns = 0; }
  record(ms) {
    if (!Number.isFinite(ms) || ms < 0) throw new Error("invalid elapsed time");
    const us = Math.ceil(ms * 1000);
    let index = us;
    if (us >= 1024) {
      const exponent = Math.floor(Math.log2(us));
      index = 1024 + (exponent - 10) * 128 + Math.floor((us / 2 ** exponent - 1) * 128);
    }
    if (index >= this.bins.length || this.bins[index] === FAILED) throw new Error("histogram exhausted");
    this.bins[index]++;
    this.count++;
    this.max_ns = Math.max(this.max_ns, Math.ceil(ms * 1e6));
  }
  json() {
    const buckets_us = [];
    for (let index = 0; index < this.bins.length; index++) {
      if (!this.bins[index]) continue;
      const exponent = 10 + Math.floor((index - 1024) / 128);
      const upper = index < 1024 ? index : Math.ceil(2 ** exponent * (1 + ((index - 1024) % 128 + 1) / 128));
      buckets_us.push([upper, this.bins[index]]);
    }
    return {count: this.count, max_ns: this.max_ns, buckets_us};
  }
}

const channel = new MessageChannel();
let nextTurn;
channel.port1.onmessage = () => { const resolve = nextTurn; nextTurn = null; resolve(); };
function yieldToHost() {
  return new Promise(resolve => { nextTurn = resolve; channel.port2.postMessage(0); });
}
function checked(value) {
  if ((value >>> 0) === FAILED) throw new Error("collector workload failed");
  return value;
}
function counters(wasm) {
  const names = ["completed", "polls", "steps", "managed_bytes", "live_objects", "retired_bytes",
    "root_pages", "pending_barriers", "finalizations", "errors", "checksum", "pending_request", "finished"];
  return Object.fromEntries(names.map((name, i) => [name, Number(wasm.gc_browser_metric(i))]));
}

let activePhase = null;
const heartbeat = setInterval(() => {
  if (activePhase) {
    const now = performance.now();
    activePhase.heartbeat.record(now - activePhase.lastHeartbeat);
    activePhase.lastHeartbeat = now;
  }
}, 1);
let animation;
function frame(now) {
  if (activePhase) {
    if (activePhase.lastFrame !== null) activePhase.frames.record(now - activePhase.lastFrame);
    activePhase.lastFrame = now;
  }
  animation = requestAnimationFrame(frame);
}
animation = requestAnimationFrame(frame);
let observer = null;
function recordLongTasks(entries) {
  if (!activePhase) return;
  for (const entry of entries) {
    if (entry.startTime >= activePhase.start && entry.startTime < activePhase.deadline)
      activePhase.longTasks.record(entry.duration);
  }
}
if (PerformanceObserver.supportedEntryTypes.includes("longtask")) {
  observer = new PerformanceObserver(list => recordLongTasks(list.getEntries()));
  observer.observe({type: "longtask", buffered: true});
}

async function phase(wasm, seconds, drain = false) {
  const startSequence = counters(wasm).completed;
  const offered = Math.floor(seconds * options.rate);
  const start = performance.now();
  const deadline = start + seconds * 1000;
  const sample = {start, deadline, lastHeartbeat: start, lastFrame: null,
    heartbeat: new Histogram(), frames: new Histogram(), longTasks: new Histogram()};
  activePhase = sample;
  const arrival = new Histogram(), service = new Histogram(), poll = new Histogram();
  const turnWork = new Histogram(), requestTurnWork = new Histogram(), turnGap = new Histogram();
  const series = [], growth = [];
  let completed = 0, peakBacklog = 0, turns = 0, pendingStart = null;
  let previousTurn = null, previousMemory = wasm.memory.buffer.byteLength, nextSample = start;
  const stop = deadline + (drain ? 1000 : 0);
  while (performance.now() < deadline || (drain && completed < offered && performance.now() < stop)) {
    await yieldToHost();
    const turnStart = performance.now();
    if (turnStart >= stop) break;
    if (previousTurn !== null) turnGap.record(turnStart - previousTurn);
    previousTurn = turnStart;
    const due = Math.min(offered, 1 + Math.floor((turnStart - start) * options.rate / 1000));
    peakBacklog = Math.max(peakBacklog, due - completed);
    let requestTurn = false;
    for (let attempt = 0; attempt < options.poll_budget && completed < due; attempt++) {
      const before = performance.now();
      if (before >= stop) break;
      requestTurn = true;
      if (pendingStart === null) pendingStart = before;
      const ready = checked(wasm.gc_browser_poll(startSequence + completed));
      const after = performance.now();
      poll.record(after - before);
      if (!ready) break;
      service.record(after - pendingStart);
      arrival.record(Math.max(0, after - (start + completed * 1000 / options.rate)));
      completed++;
      pendingStart = null;
    }
    checked(wasm.gc_browser_step());
    const now = performance.now();
    const arrived = Math.min(offered, 1 + Math.floor((now - start) * options.rate / 1000));
    peakBacklog = Math.max(peakBacklog, arrived - completed);
    const memory = wasm.memory.buffer.byteLength;
    if (memory !== previousMemory) {
      growth.push({t_ms: now - start, before_bytes: previousMemory, after_bytes: memory});
      previousMemory = memory;
    }
    if (now >= nextSample) {
      series.push({t_ms: now - start, offered: arrived, completed, backlog: arrived - completed,
        wasm_bytes: memory, counters: counters(wasm)});
      nextSample = now + Math.max(100, seconds * 1000 / 10000);
    }
    const turnElapsed = performance.now() - turnStart;
    turnWork.record(turnElapsed);
    // Idle posted turns are frequent. Keep request-bearing turns separate so
    // their cost cannot disappear into the idle-turn percentile denominator.
    // This includes the GC step, but excludes maintenance-only GC turns.
    if (requestTurn) requestTurnWork.record(turnElapsed);
    turns++;
  }
  recordLongTasks(observer?.takeRecords() ?? []);
  const elapsed_ms = performance.now() - start;
  // Include the last partial heartbeat interval so an end-of-trial stall is not lost.
  sample.heartbeat.record(performance.now() - sample.lastHeartbeat);
  activePhase = null;
  peakBacklog = Math.max(peakBacklog, offered - completed);
  return {offered, completed, unfinished: offered - completed, peak_backlog: peakBacklog,
    duration_ms: seconds * 1000, elapsed_ms, turns, start_sequence: startSequence,
    histograms: {arrival: arrival.json(), service: service.json(), poll: poll.json(),
      turn_work: turnWork.json(), request_turn_work: requestTurnWork.json(),
      turn_gap: turnGap.json(), heartbeat_gap: sample.heartbeat.json(),
      frame_gap: sample.frames.json(), long_task: sample.longTasks.json()}, series, growth};
}

function clockResolution() {
  let previous = performance.now(), zero = 0;
  const positive = [];
  for (let i = 0; i < 10000; i++) {
    const now = performance.now(), difference = now - previous;
    if (difference > 0) positive.push(difference); else zero++;
    previous = now;
  }
  positive.sort((a, b) => a - b);
  return {reads: 10000, zero_differences: zero,
    minimum_positive_ms: positive[0] ?? null,
    median_positive_ms: positive[Math.floor(positive.length / 2)] ?? null};
}

async function run() {
  const initializationStart = performance.now();
  const {instance} = await WebAssembly.instantiateStreaming(fetch("/workload.wasm"), {});
  const wasm = instance.exports;
  checked(wasm.gc_browser_init({baseline: 0, candidate: 1, control: 2}[options.policy], options.heap_mib));
  const initial_memory_bytes = wasm.memory.buffer.byteLength;
  const initialization_ms = performance.now() - initializationStart;
  const clock = clockResolution();
  const warmup = await phase(wasm, options.warmup, true);
  if (warmup.unfinished) throw Object.assign(new Error("warmup left unfinished requests"), {warmup});
  const measurement = await phase(wasm, options.seconds);
  const before_teardown = counters(wasm);
  const teardownStart = performance.now();
  const teardownPoll = new Histogram();
  let teardown_polls = 0;
  while (true) {
    await yieldToHost();
    const before = performance.now();
    const done = checked(wasm.gc_browser_finish());
    teardownPoll.record(performance.now() - before);
    teardown_polls++;
    if (done) break;
    if (performance.now() - teardownStart > 30000) throw new Error("cooperative teardown timeout");
  }
  return {schema: 1, options, user_agent: navigator.userAgent,
    cross_origin_isolated: crossOriginIsolated, long_task_observer: observer !== null,
    initialization_ms, clock, initial_memory_bytes, final_memory_bytes: wasm.memory.buffer.byteLength,
    warmup, measurement, before_teardown, after_teardown: counters(wasm),
    teardown_ms: performance.now() - teardownStart, teardown_polls,
    teardown_poll: teardownPoll.json(), qualification: false};
}

let result;
try { result = {status: "ok", result: await run()}; }
catch (error) { result = {status: "failed", error: String(error.stack ?? error), warmup: error.warmup ?? null}; }
finally {
  clearInterval(heartbeat); cancelAnimationFrame(animation); observer?.disconnect();
  channel.port1.close(); channel.port2.close();
}
document.getElementById("status").textContent = result.status;
await fetch(`/result?token=${encodeURIComponent(token)}`, {method: "POST",
  headers: {"Content-Type": "application/json"}, body: JSON.stringify(result)});
