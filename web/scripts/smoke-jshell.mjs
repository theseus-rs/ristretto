import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { registerHooks } from 'node:module';
// Exercise the browser's in-memory WASI adapters even when running this check in Node.
registerHooks({
  resolve(specifier, context, next) {
    return next(
      specifier,
      specifier.startsWith('@bytecodealliance/preview2-shim')
        ? { ...context, conditions: context.conditions.filter((condition) => condition !== 'node') }
        : context,
    );
  },
});
const { createEngine } = await import('../shared/engine.ts');
import manifest from '../generated/runtime-manifest.json' with { type: 'json' };

const selected = process.argv.slice(2).map(Number);
for (const javaVersion of selected.length ? selected : [11, 17, 21, 25]) {
  const entries = { ...manifest.files, 'jdk.zip': manifest.jdks[javaVersion] };
  const assets = Object.entries(entries).map(([name, asset]) => [
    name,
    new Uint8Array(readFileSync(new URL(`../public/runtime/${asset.file}`, import.meta.url))),
  ]);
  let events = [];
  const engine = await createEngine(assets, (event) => events.push(event));
  let id = 0;
  async function input(source, operation = 'input', cursor = source.length) {
    events = [];
    await engine.execute({
      id: ++id,
      javaVersion,
      action: 'jshell',
      className: 'BrowserJShell',
      source,
      operation,
      cursor,
    });
    const output = events
      .filter((event) => event.type === 'output')
      .map((event) => event.text)
      .join('');
    console.log(`Java ${javaVersion} > ${source}\n${output}`);
    assert.ok(events.every((event) => event.id === id));
    const result = events.at(-1);
    assert.ok(['ready', 'completions'].includes(result?.type), JSON.stringify(events));
    return { output, result };
  }
  assert.match((await input('')).output, /Welcome to JShell/);
  assert.equal((await input('// a Java comment')).result.continuation, false);
  assert.equal((await input('/* a multiline')).result.continuation, true);
  assert.equal((await input('comment */')).result.continuation, false);
  assert.match((await input('int counter = 0;')).output, /counter ==> 0/);
  await input('counter++;');
  assert.match((await input('/vars')).output, /int counter = 1/);
  // This identity and mutation survive repeated component entry; history is never replayed.
  await input('var values = new ArrayList<String>();');
  await input('values.add("coffee");');
  assert.match((await input('values.toString()')).output, /==> "\[coffee\]"/);
  assert.equal((await input('int twice(int value) {')).result.continuation, true);
  assert.equal((await input('return value * 2;')).result.continuation, true);
  assert.equal((await input('}')).result.continuation, false);
  assert.match((await input('twice(counter)')).output, /==> 2/);
  assert.match((await input('/methods')).output, /twice\(int\)int/);
  assert.ok((await input('coun', 'complete')).result.suggestions.includes('counter'));
  await input('int broken = "wrong";');
  assert.match((await input('counter')).output, /counter ==> 1/);
  assert.match((await input('/-99999999999999999999')).output, /Unknown command or snippet/);
  await input('int kept = 7;');
  await input('int removed = 9;');
  await input('/drop removed');
  await input('/reload');
  assert.match((await input('counter')).output, /counter ==> 1/);
  const variables = (await input('/vars')).output;
  assert.match(variables, /int kept = 7/);
  assert.doesNotMatch(variables, /int removed =/);
  assert.match((await input('values.toString()')).output, /==> "\[coffee\]"/);
  await input('throw new IllegalArgumentException("test failure");');
  assert.match((await input('counter')).output, /counter ==> 1/);
  // #940: an impossible wasm32 array must throw a catchable Java exception, and
  // both caught and uncaught allocation failures must preserve the session.
  for (const expression of [
    'new long[Integer.MAX_VALUE - 8]',
    'new Object[Integer.MAX_VALUE - 8]',
    'new long[1][Integer.MAX_VALUE - 8]',
    'java.lang.reflect.Array.newInstance(long.class, Integer.MAX_VALUE - 8)',
    'java.lang.reflect.Array.newInstance(Object.class, 1, Integer.MAX_VALUE - 8)',
  ]) {
    assert.match(
      (
        await input(
          `try { Object huge = ${expression}; } catch (OutOfMemoryError e) { System.out.println("caught array allocation failure"); }`,
        )
      ).output,
      /caught array allocation failure/,
    );
  }
  assert.match(
    (await input('new long[Integer.MAX_VALUE - 8]')).output,
    /java.lang.OutOfMemoryError/,
  );
  assert.match((await input('counter')).output, /counter ==> 1/);
  await input('int incomplete(');
  assert.equal((await input('', 'cancel')).result.continuation, false);
  assert.match((await input('counter')).output, /counter ==> 1/);
  await input('/drop counter');
  assert.match((await input('counter')).output, /cannot find symbol/);
  assert.equal((await input('/reset')).result.reset, true);
  assert.match((await input('/vars')).output, /Welcome to JShell/);
  assert.doesNotMatch((await input('/methods')).output, /twice/);
  assert.equal((await input('/exit')).result.closed, true);
  console.log(`Java ${javaVersion} persistent JShell checks passed.`);
}
