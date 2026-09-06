import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import jdks from '../jdks.json' with { type: 'json' };
import { unzipSync } from 'fflate';
import manifest from '../generated/runtime-manifest.json' with { type: 'json' };
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = fileURLToPath(new URL('../../', import.meta.url));
const workspace = resolve(root, 'target/playground-smoke');
for (const { major } of jdks) {
  const jdk = resolve(workspace, `jdk-${major}`);
  rmSync(jdk, { recursive: true, force: true });
  const archive = readFileSync(
    resolve(root, 'playground/public/runtime', manifest.jdks[major].file),
  );
  for (const [name, bytes] of Object.entries(unzipSync(archive))) {
    const destination = resolve(jdk, name);
    mkdirSync(dirname(destination), { recursive: true });
    writeFileSync(destination, bytes);
  }
  const temporary = resolve(workspace, 'tmp');
  mkdirSync(workspace, { recursive: true });
  mkdirSync(temporary, { recursive: true });
  writeFileSync(
    resolve(workspace, 'request.json'),
    JSON.stringify({
      id: 1,
      javaVersion: major,
      action: 'run',
      className: 'Main',
      source: `import java.nio.file.*;
public class Main {
  public static void main(String[] args) throws Exception {
    System.out.println("Java " + System.getProperty("java.version") + " ☕");
    if (!System.getProperty("java.version").startsWith("${major === 8 ? '1.8.' : major + '.'}")) throw new AssertionError("runtime version");
    if (args.length != 0 || System.in.read() != -1) throw new AssertionError("input");
    if ("😀".hashCode() != 1772899) throw new AssertionError("UTF-16 hash");
    java.util.concurrent.atomic.AtomicReference<Object> reference = new java.util.concurrent.atomic.AtomicReference<>(new Object());
    if (reference.compareAndSet(new Object(), null)) throw new AssertionError("reference identity");
    ${
      major >= 21
        ? `
    var weak = new java.lang.ref.WeakReference<>(reference.get());
    if (weak.refersTo(new Object()) || !weak.refersTo(reference.get())) throw new AssertionError("weak identity");
    weak.clear();
    if (!weak.refersTo(null)) throw new AssertionError("cleared reference");`
        : ''
    }
    Path lib = Paths.get("/jdk/lib/../lib").toRealPath();
    if (!lib.toString().equals("/jdk/lib") || !Files.isReadable(lib)) throw new AssertionError("realpath/access");
    try (DirectoryStream<Path> entries = Files.newDirectoryStream(lib)) {
      boolean found = false;
      for (Path entry : entries) if (entry.getFileName().toString().equals("${major === 8 ? 'tools.jar' : 'modules'}")) found = true;
      if (!found) throw new AssertionError("directory iteration");
    }
    try { Paths.get("/jdk/missing").toRealPath(); throw new AssertionError("missing path"); }
    catch (NoSuchFileException expected) { }
    System.out.println("Filesystem and input checks passed.");
  }
}`,
    }),
  );
  const output = execFileSync(
    'wasmtime',
    [
      'run',
      '-W',
      'max-wasm-stack=8388608',
      '-S',
      'http',
      `--dir=${workspace}::/workspace`,
      `--dir=${jdk}::/jdk`,
      `--dir=${temporary}::/tmp`,
      resolve(root, 'target/wasm32-wasip2/release/ristretto_playground.wasm'),
    ],
    { encoding: 'utf8', timeout: 120_000 },
  );
  const events = output
    .trim()
    .split('\n')
    .map((line) => JSON.parse(line));
  const text = Buffer.concat(
    events.filter((event) => event.type === 'output').map((event) => Buffer.from(event.bytes)),
  ).toString();
  console.log(text);
  if (
    !events.some((event) => event.type === 'done') ||
    !text.includes('Filesystem and input checks passed.')
  ) {
    throw new Error(JSON.stringify(events));
  }
  console.log('WASI compilation and execution passed.');
}
