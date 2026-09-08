import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { registerHooks } from 'node:module';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import definitions from '../languages.json' with { type: 'json' };

// A parent process enforces a deadline even if synchronous WASM execution blocks Node.
if (!process.argv.includes('--child')) {
  for (const target of process.argv.slice(2).length
    ? process.argv.slice(2)
    : Object.keys(definitions)) {
    assert.ok(target in definitions, `Unknown target: ${target}`);
    execFileSync(
      process.execPath,
      ['--import', 'tsx', fileURLToPath(import.meta.url), target, '--child'],
      { stdio: 'inherit', timeout: 755_000 },
    );
  }
} else {
  registerHooks({
    resolve(specifier, context, next) {
      return next(
        specifier,
        specifier.startsWith('@bytecodealliance/preview2-shim')
          ? {
              ...context,
              conditions: context.conditions.filter((condition) => condition !== 'node'),
            }
          : context,
      );
    },
  });
  const { createEngine } = await import('../shared/engine.ts');
  const { examples } = await import('../playground/examples.ts');
  const { default: manifest } = await import('../generated/runtime-manifest.json', {
    with: { type: 'json' },
  });
  const target = process.argv[2];
  const entries = {
    ...manifest.files,
    'jdk.zip': manifest.jdks[25],
    'language.zip': manifest.languages[target],
  };
  const assets = Object.entries(entries).map(([name, entry]) => [
    name,
    new Uint8Array(readFileSync(new URL(`../public/runtime/${entry.file}`, import.meta.url))),
  ]);
  for (const action of ['check', 'run']) {
    const events = [];
    const started = performance.now();
    const engine = await createEngine(assets, (event) => {
      events.push(event);
      if (event.type === 'phase' || event.type === 'error' || event.type === 'output')
        console.log(target, event);
    });
    engine.execute({
      id: 1,
      action,
      language: target.startsWith('scala') ? 'scala' : target,
      scalaVersion: target === 'scala2' ? '2.13' : '3',
      javaVersion: 25,
      className: 'Main',
      source: examples[target].hello.source,
    });
    assert.equal(events.at(-1)?.type, 'done', JSON.stringify(events));
    assert.deepEqual(
      events.filter((event) => event.type === 'phase').map((event) => event.phase),
      [action === 'check' ? 'checking' : 'running'],
    );
    assert.equal(
      events.filter((event) => event.type === 'checked').length,
      action === 'check' ? 1 : 0,
    );
    const output = events
      .filter((event) => event.type === 'output' && event.stream === 'stdout')
      .map((event) => event.text)
      .join('');
    if (action === 'check') assert.equal(output, '');
    else assert.match(output, /Hello, world! ☕/);
    console.log(
      `${target} ${action} passed in ${((performance.now() - started) / 1000).toFixed(2)}s`,
    );
  }
}
