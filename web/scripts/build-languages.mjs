import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { homedir } from 'node:os';
import { delimiter, join } from 'node:path';
import { unzipSync, zipSync } from 'fflate';
import languages from '../languages.json' with { type: 'json' };

/** Build from a fully resolved lock; browsers never resolve Maven dependencies. */
export async function buildLanguages(root, javaHome, asset, jdkImage, jdkModules) {
  const cache = process.env.PLAYGROUND_LANGUAGE_CACHE ?? join(homedir(), '.ristretto', 'languages');
  mkdirSync(cache, { recursive: true });
  const manifest = {};
  const notices = [];
  for (const [target, definition] of Object.entries(languages)) {
    const directory = join(root, 'target', `playground-language-${target}`);
    rmSync(directory, { recursive: true, force: true });
    mkdirSync(directory, { recursive: true });
    const jars = [];
    const files = {};
    for (const entry of definition.artifacts) {
      const cached = join(cache, `${entry.sha256}.jar`);
      let bytes = existsSync(cached) ? readFileSync(cached) : undefined;
      const hash = (data) => createHash('sha256').update(data).digest('hex');
      if (!bytes || hash(bytes) !== entry.sha256) {
        const response = await fetch(entry.url);
        if (!response.ok) throw new Error(`Could not download ${entry.file}: ${response.status}`);
        bytes = Buffer.from(await response.arrayBuffer());
        if (hash(bytes) !== entry.sha256) throw new Error(`Checksum mismatch: ${entry.file}`);
        writeFileSync(cached, bytes);
      }
      const path = join(directory, entry.file);
      writeFileSync(path, bytes);
      jars.push(path);
      files[entry.file] = [new Uint8Array(bytes), { mtime: new Date('2026-01-01T00:00:00Z') }];
      notices.push(`\n=== ${entry.file} ===\nSource: ${entry.url}`);
      for (const [name, data] of Object.entries(unzipSync(bytes))) {
        if (/(^|\/)(license|notice|copying|epl)([._-]|$)/i.test(name) && data.length)
          notices.push(`--- ${name} ---\n${Buffer.from(data).toString('utf8')}`);
      }
    }
    if (target === 'kotlin') {
      // Give the embedded compiler a regular classpath view of the exact linked JDK.
      // The compiler can index ordinary JAR entries in the browser's virtual filesystem.
      const release = readFileSync(join(jdkImage, 'release'), 'utf8');
      const modules = release.match(/^MODULES="([^"]+)"/m)?.[1].split(' ');
      if (!modules) throw new Error('Linked JDK module list is missing');
      const api = {};
      for (const module of modules) {
        const bytes = readFileSync(join(jdkModules, `${module}.jmod`));
        for (const [name, data] of Object.entries(unzipSync(bytes.subarray(4)))) {
          if (
            name.startsWith('classes/') &&
            name.endsWith('.class') &&
            name !== 'classes/module-info.class'
          )
            api[name.slice('classes/'.length)] = [
              data,
              { mtime: new Date('2026-01-01T00:00:00Z') },
            ];
        }
      }
      const bytes = zipSync(api);
      const path = join(directory, 'jdk-api.jar');
      writeFileSync(path, bytes);
      jars.push(path);
      files['jdk-api.jar'] = [bytes, { mtime: new Date('2026-01-01T00:00:00Z') }];
    }
    const classes = join(directory, 'bridge');
    mkdirSync(classes);
    const bridge = join(root, 'web', 'runner', 'java', 'scripts');
    const classpath = jars.join(delimiter);
    const run = (command, args) =>
      execFileSync(join(javaHome, 'bin', command), args, { stdio: 'inherit' });
    if (target === 'kotlin') {
      run('java', [
        '-cp',
        classpath,
        'org.jetbrains.kotlin.cli.jvm.K2JVMCompiler',
        '-no-stdlib',
        '-no-reflect',
        '-jvm-target',
        '17',
        '-classpath',
        classpath,
        '-d',
        classes,
        join(bridge, 'KotlinScript.kt'),
      ]);
    } else {
      const name = target.startsWith('scala')
        ? 'ScalaScript'
        : target === 'groovy'
          ? 'GroovyScript'
          : 'ClojureScript';
      run('javac', [
        '--release',
        '17',
        '-cp',
        classpath,
        '-d',
        classes,
        join(bridge, `${name}.java`),
      ]);
    }
    const bridgeFiles = {};
    function collect(path, prefix = '') {
      for (const entry of readdirSync(path, { withFileTypes: true })) {
        const name = prefix + entry.name;
        if (entry.isDirectory()) collect(join(path, entry.name), name + '/');
        else
          bridgeFiles[name] = [
            new Uint8Array(readFileSync(join(path, entry.name))),
            { mtime: new Date('2026-01-01T00:00:00Z') },
          ];
      }
    }
    collect(classes);
    files['browser-script.jar'] = [
      zipSync(bridgeFiles),
      { mtime: new Date('2026-01-01T00:00:00Z') },
    ];
    writeFileSync(join(directory, 'browser-script.jar'), files['browser-script.jar'][0]);
    manifest[target] = {
      version: definition.version,
      javaVersion: definition.javaVersion,
      ...asset(`${target}.zip`, zipSync(files, { level: 6 })),
    };
  }
  return { manifest, notices };
}
