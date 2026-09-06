export const JAVA_VERSIONS = [8, 11, 17, 21, 25] as const;
export type JavaVersion = (typeof JAVA_VERSIONS)[number];
export const isJavaVersion = (value: unknown): value is JavaVersion =>
  JAVA_VERSIONS.some((version) => version === value);

export type Request = {
  javaVersion: JavaVersion;
  id: number;
  action: 'compile' | 'run';
  className: string;
  source: string;
};
export type Event =
  | { id: number; type: 'phase'; phase: 'loading' | 'compiling' | 'running' }
  | { id: number; type: 'progress'; loaded: number; total: number }
  | { id: number; type: 'compiled'; classes: number }
  | { id: number; type: 'output'; stream: 'stdout' | 'stderr'; text: string }
  | { id: number; type: 'done'; exitCode?: number }
  | { id: number; type: 'error'; message: string };

export const LOAD_TIMEOUT = 120_000;
export const COMPILE_TIMEOUT = 600_000;
export const RUN_TIMEOUT = 30_000;
export const OUTPUT_LIMIT = 1024 * 1024;

/** Decode the runner's NDJSON and independently preserve split UTF-8 on each Java stream. */
export class EventDecoder {
  private decoder = new TextDecoder();
  private streams = { stdout: new TextDecoder(), stderr: new TextDecoder() };
  private pending = '';
  constructor(private readonly emit: (event: Event) => void) {}

  push(bytes: Uint8Array) {
    this.pending += this.decoder.decode(bytes, { stream: true });
    let end: number;
    while ((end = this.pending.indexOf('\n')) >= 0) {
      const line = this.pending.slice(0, end);
      this.pending = this.pending.slice(end + 1);
      if (!line) continue;
      const event = JSON.parse(line);
      if (event.type === 'output') {
        if (event.stream !== 'stdout' && event.stream !== 'stderr')
          throw new Error('Invalid output stream');
        const stream = event.stream as 'stdout' | 'stderr';
        const text = this.streams[stream].decode(new Uint8Array(event.bytes), { stream: true });
        if (text) this.emit({ id: event.id, type: 'output', stream, text });
      } else {
        if (event.type === 'done' || event.type === 'error') this.flush(event.id);
        this.emit(event);
      }
    }
    if (this.pending.length > 8 * OUTPUT_LIMIT)
      throw new Error('Runner output record is too large');
  }

  flush(id: number) {
    for (const stream of ['stdout', 'stderr'] as const) {
      const text = this.streams[stream].decode();
      if (text) this.emit({ id, type: 'output', stream, text });
    }
  }
}
