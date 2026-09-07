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
  function input(source, operation = 'input', cursor = source.length) {
    events = [];
    engine.execute({
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
  assert.match(input('').output, /Welcome to JShell/);
  assert.equal(input('// a Java comment').result.continuation, false);
  assert.equal(input('/* a multiline').result.continuation, true);
  assert.equal(input('comment */').result.continuation, false);
  assert.match(input('int counter = 0;').output, /counter ==> 0/);
  input('counter++;');
  assert.match(input('/vars').output, /int counter = 1/);
  // This identity and mutation survive repeated component entry; history is never replayed.
  input('var values = new ArrayList<String>();');
  input('values.add("coffee");');
  assert.match(input('values.toString()').output, /==> "\[coffee\]"/);
  assert.equal(input('int twice(int value) {').result.continuation, true);
  assert.equal(input('return value * 2;').result.continuation, true);
  assert.equal(input('}').result.continuation, false);
  assert.match(input('twice(counter)').output, /==> 2/);
  assert.match(input('/methods').output, /twice\(int\)int/);
  assert.ok(input('coun', 'complete').result.suggestions.includes('counter'));
  input('int broken = "wrong";');
  assert.match(input('counter').output, /counter ==> 1/);
  assert.match(input('/-99999999999999999999').output, /Unknown command or snippet/);
  input('int kept = 7;');
  input('int removed = 9;');
  input('/drop removed');
  input('/reload');
  assert.match(input('counter').output, /counter ==> 1/);
  const variables = input('/vars').output;
  assert.match(variables, /int kept = 7/);
  assert.doesNotMatch(variables, /int removed =/);
  assert.match(input('values.toString()').output, /==> "\[coffee\]"/);
  input('throw new IllegalArgumentException("test failure");');
  assert.match(input('counter').output, /counter ==> 1/);
  input('int incomplete(');
  assert.equal(input('', 'cancel').result.continuation, false);
  assert.match(input('counter').output, /counter ==> 1/);
  input('/drop counter');
  assert.match(input('counter').output, /cannot find symbol/);
  assert.equal(input('/reset').result.reset, true);
  assert.match(input('/vars').output, /Welcome to JShell/);
  assert.doesNotMatch(input('/methods').output, /twice/);
  assert.equal(input('/exit').result.closed, true);
  console.log(`Java ${javaVersion} persistent JShell checks passed.`);
}
