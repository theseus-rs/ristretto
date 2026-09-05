import { EditorView, basicSetup } from 'codemirror';
import { keymap } from '@codemirror/view';
import { Compartment } from '@codemirror/state';
import { java } from '@codemirror/lang-java';
import { editorTheme, type Theme } from './editor-theme';
import logo from '../../images/logo.svg';
import { examples } from './examples';
import {
  COMPILE_TIMEOUT,
  LOAD_TIMEOUT,
  RUN_TIMEOUT,
  JAVA_VERSIONS,
  isJavaVersion,
  type JavaVersion,
  type Event,
  type Request,
} from './protocol';
import './style.css';
import workerScript from './runner.worker?worker&url';
import { loadRuntime } from './runtime';

// Keep the bundled script in memory so every run can use a fresh worker even offline.
let workerUrl: Promise<string> | undefined;
function loadWorker() {
  // Vite's development worker imports other modules from its server.
  if (import.meta.env.DEV) return Promise.resolve(workerScript);
  return (workerUrl ??= fetch(workerScript)
    .then(async (response) => {
      if (!response.ok)
        throw new Error(`Could not load Java worker (${response.status}). Try again.`);
      return URL.createObjectURL(new Blob([await response.text()], { type: 'text/javascript' }));
    })
    .catch((error) => {
      workerUrl = undefined;
      throw error;
    }));
}

const runIcon =
  '<svg viewBox="0 0 20 20" aria-hidden="true"><path d="m6 3 11 7-11 7Z" fill="currentColor"/></svg>';
const app = document.querySelector<HTMLDivElement>('#app')!;
app.innerHTML = `
  <header class="site-header">
    <a class="brand" href="../"><img src="${logo}" alt="" /><span>ristretto<span class="brand-divider">/</span><span class="brand-product">playground</span></span></a>
    <div class="header-actions"><label class="theme-picker" for="theme"><span>Theme</span><select id="theme" aria-label="Color theme"><option value="system">System</option><option value="light">Light</option><option value="dark">Dark</option></select></label><a class="github-link" href="https://github.com/theseus-rs/ristretto" target="_blank" rel="noreferrer">View on GitHub <span aria-hidden="true">↗</span></a></div>
  </header>
  <main>
    <section class="intro">
      <div><h1>A place to play with Java.</h1><p>Write a little code. Try an idea. See what happens.</p></div>
      <div class="local-note"><span class="local-icon" aria-hidden="true">⌘</span><div>Right in your browser.<br /><span>Your code stays on your device.</span></div></div>
    </section>
    <section class="workbench" aria-label="Java playground">
      <div class="toolbar">
        <div class="pickers"><div class="version-picker"><label for="java-version">Java version</label><select id="java-version">${JAVA_VERSIONS.map((version) => `<option value="${version}" ${version === 25 ? 'selected' : ''}>${version}</option>`).join('')}</select></div><div class="example-picker"><label for="example">Start with</label><select id="example">${Object.entries(
          examples,
        )
          .map(([key, example]) => `<option value="${key}">${example.title}</option>`)
          .join('')}</select></div></div>
        <div class="actions"><button id="compile" class="button secondary">Compile</button><button id="stop" class="button stop" disabled><span aria-hidden="true">■</span> Stop</button><button id="run" class="button primary">${runIcon} Run<span class="shortcut" aria-hidden="true">⌘ ↵</span></button></div>
      </div>
      <div class="panes">
        <section class="source-pane" aria-label="Java source">
          <div class="pane-header source-header"><span class="file-label"><span class="java-icon" aria-hidden="true">☕</span><span id="filename">Main.java</span></span><span id="language-label" class="language-label">JAVA 25</span></div>
          <div id="editor"></div>
          <div class="editor-footer"><label for="class-name">Main class</label><input id="class-name" value="Main" spellcheck="false" autocomplete="off" aria-describedby="main-help" /><span id="cursor">Ln 1, Col 1</span></div>
        </section>
        <section class="output-pane" aria-label="Program output">
          <div class="pane-header"><span class="console-label"><span aria-hidden="true">›_</span> Console</span><button id="clear" class="text-button">Clear output</button></div>
          <div class="console-body"><div id="empty-output"><span class="empty-icon" aria-hidden="true">›_</span><strong>Your next idea starts here.</strong><p>Hit <b>Run</b> to compile your Java<br />and see the output.</p><kbd>⌘ / Ctrl + Enter</kbd></div><pre id="output" aria-label="Console output" tabindex="0"></pre></div>
          <div class="console-footer"><span id="status" role="status" aria-live="polite" data-state="ready"><span class="status-dot"></span><span id="status-text">Ready when you are</span></span><span id="elapsed"></span></div>
        </section>
      </div>
      <div id="loading" hidden><progress id="progress" max="1" value="0" aria-label="Java runtime download"></progress><span id="loading-text">Loading Java…</span></div>
    </section>
    <footer class="workspace-footer"><span><span class="status-dot"></span> Powered by Ristretto <span class="footer-divider">·</span> <span id="runtime-label">Java 25</span></span><details><summary>Good to know <span aria-hidden="true">＋</span></summary><div class="help-card"><p id="main-help">Use a <code>public static void main(String[] args)</code> entry point. Set Main class to its fully qualified name when using a package.</p><p>Compile checks your source. Run compiles it again and starts a fresh program. Standard input is closed and arguments are empty.</p><p>Core Java libraries, collections, streams, and records are included. External dependencies, GUI, networking, and process execution are unavailable. Ristretto is an evolving JVM; some Java APIs may be unsupported.</p><p>Compilation may take several minutes in Firefox and stops after 10 minutes; execution after 30 seconds or 1 MiB of output. Stop interrupts either phase.</p><p>The first run of each version downloads its Java runtime. Switching versions preserves your code. Later runs reuse locally cached assets when browser storage is available.</p><a href="./notices.html" target="_blank" rel="noreferrer">Third-party notices ↗</a></div></details></footer>
  </main>`;

const element = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
const runButton = element<HTMLButtonElement>('run');
const compileButton = element<HTMLButtonElement>('compile');
const stopButton = element<HTMLButtonElement>('stop');
const className = element<HTMLInputElement>('class-name');
const examplePicker = element<HTMLSelectElement>('example');
const versionPicker = element<HTMLSelectElement>('java-version');
const output = element<HTMLPreElement>('output');
const emptyOutput = element<HTMLDivElement>('empty-output');
const status = element<HTMLSpanElement>('status');
const statusText = element<HTMLSpanElement>('status-text');
const loading = element<HTMLDivElement>('loading');
const progress = element<HTMLProgressElement>('progress');
const elapsed = element<HTMLSpanElement>('elapsed');
const themePicker = element<HTMLSelectElement>('theme');
const systemTheme = matchMedia('(prefers-color-scheme: dark)');
const themeKey = 'ristretto-playground-theme';
let themePreference = 'system';
try {
  const saved = localStorage.getItem(themeKey);
  if (saved === 'light' || saved === 'dark') themePreference = saved;
} catch {
  /* Theme controls also work when browser storage is unavailable. */
}
themePicker.value = themePreference;
const selectedTheme = (): Theme =>
  themePreference === 'system'
    ? systemTheme.matches
      ? 'dark'
      : 'light'
    : (themePreference as Theme);
const themeCompartment = new Compartment();
document.documentElement.dataset.theme = selectedTheme();
const storageKey = 'ristretto-playground-source-v1';
let initialSource: string = examples.hello.source;
try {
  const saved = JSON.parse(localStorage.getItem(storageKey) ?? 'null');
  if (isJavaVersion(saved?.javaVersion)) versionPicker.value = String(saved.javaVersion);
  if (typeof saved?.source === 'string' && typeof saved?.className === 'string') {
    initialSource = saved.source;
    className.value = saved.className;
  }
} catch {
  /* Editing and execution work without browser storage. */
}

let saveTimer: ReturnType<typeof setTimeout>;
const save = () => {
  clearTimeout(saveTimer);
  saveTimer = setTimeout(() => {
    try {
      localStorage.setItem(
        storageKey,
        JSON.stringify({
          source: editor.state.doc.toString(),
          className: className.value,
          javaVersion: Number(versionPicker.value),
        }),
      );
    } catch {
      /* Storage is optional. */
    }
  }, 200);
};
const editor = new EditorView({
  doc: initialSource,
  extensions: [
    basicSetup,
    java(),
    themeCompartment.of(editorTheme(selectedTheme())),
    keymap.of([
      {
        key: 'Mod-Enter',
        run: () => {
          start('run');
          return true;
        },
      },
    ]),
    EditorView.contentAttributes.of({ 'aria-label': 'Java source code', spellcheck: 'false' }),
    EditorView.theme({
      '&': { height: '100%', fontSize: '14px' },
      '.cm-scroller': {
        fontFamily: '"SFMono-Regular", Consolas, "Liberation Mono", monospace',
        lineHeight: '1.85',
      },
      '.cm-content': { padding: '22px 0' },
      '.cm-lineNumbers .cm-gutterElement': { padding: '0 15px 0 18px' },
      '.cm-line': { padding: '0 22px 0 8px' },
    }),
    EditorView.updateListener.of((update) => {
      if (update.docChanged) save();
      const position = update.state.selection.main.head;
      const line = update.state.doc.lineAt(position);
      element('cursor').textContent = `Ln ${line.number}, Col ${position - line.from + 1}`;
    }),
  ],
  parent: element('editor'),
});

function applyTheme() {
  const theme = selectedTheme();
  document.documentElement.dataset.theme = theme;
  document
    .querySelector('meta[name="theme-color"]')
    ?.setAttribute('content', theme === 'dark' ? '#1e1f22' : '#f5f7f4');
  editor.dispatch({ effects: themeCompartment.reconfigure(editorTheme(theme)) });
}
themePicker.addEventListener('change', () => {
  themePreference = themePicker.value;
  try {
    if (themePreference === 'system') localStorage.removeItem(themeKey);
    else localStorage.setItem(themeKey, themePreference);
  } catch {
    /* Storage is optional. */
  }
  applyTheme();
});
systemTheme.addEventListener('change', () => {
  if (themePreference === 'system') applyTheme();
});
window.addEventListener('storage', (event) => {
  if (event.key !== themeKey && event.key !== null) return;
  themePreference =
    event.newValue === 'light' || event.newValue === 'dark' ? event.newValue : 'system';
  themePicker.value = themePreference;
  applyTheme();
});
applyTheme();

let worker: Worker | undefined;
let active = false;
let requestId = 0;
let watchdog: ReturnType<typeof setTimeout>;
let clock: ReturnType<typeof setInterval>;
let started = 0;
let pending: { text: string; stream: string }[] = [];
let renderFrame = 0;

function flushOutput() {
  cancelAnimationFrame(renderFrame);
  renderFrame = 0;
  const nearBottom = output.scrollHeight - output.scrollTop - output.clientHeight < 80;
  for (const item of pending) {
    if (!item.text) continue;
    emptyOutput.hidden = true;
    const last = output.lastElementChild;
    if (last?.className === item.stream) last.lastChild!.textContent += item.text;
    else {
      const span = document.createElement('span');
      span.className = item.stream;
      span.append(document.createTextNode(item.text));
      output.append(span);
    }
  }
  pending = [];
  if (nearBottom) output.scrollTop = output.scrollHeight;
}
function append(text: string, stream = 'stdout') {
  const last = pending.at(-1);
  if (last?.stream === stream) last.text += text;
  else pending.push({ text, stream });
  if (!renderFrame) renderFrame = requestAnimationFrame(flushOutput);
}
function clearOutput() {
  pending = [];
  output.replaceChildren();
  emptyOutput.hidden = false;
}
function setStatus(text: string, state = 'ready') {
  statusText.textContent = text;
  status.dataset.state = state;
}
function finish(text: string, state = 'ready') {
  worker?.terminate();
  worker = undefined;
  active = false;
  clearTimeout(watchdog);
  clearInterval(clock);
  loading.hidden = true;
  runButton.disabled = compileButton.disabled = false;
  stopButton.disabled = true;
  versionPicker.disabled = false;
  flushOutput();
  setStatus(text, state);
}
function timeout(ms: number, phase: string) {
  clearTimeout(watchdog);
  watchdog = setTimeout(() => {
    append(
      `\n${phase} exceeded ${ms / 1000} seconds. You can edit the code and try again.\n`,
      'stderr',
    );
    finish('Time limit reached', 'error');
  }, ms);
}
async function start(action: Request['action']) {
  if (active) return;
  const name = className.value.trim();
  if (!name || !name.split('.').every((part) => /^[\p{L}_$][\p{L}\p{N}_$]*$/u.test(part))) {
    className.setCustomValidity('Enter a Java class name, such as Main or example.Main.');
    className.reportValidity();
    return;
  }
  className.setCustomValidity('');
  clearOutput();
  const id = ++requestId;
  const source = editor.state.doc.toString();
  const javaVersion = Number(versionPicker.value) as JavaVersion;
  versionPicker.disabled = true;
  active = true;
  started = performance.now();
  elapsed.textContent = '';
  runButton.disabled = compileButton.disabled = true;
  stopButton.disabled = false;
  setStatus('Loading Java…', 'busy');
  loading.hidden = false;
  progress.value = 0;
  timeout(LOAD_TIMEOUT, 'Runtime loading');
  clock = setInterval(() => {
    elapsed.textContent = `${((performance.now() - started) / 1000).toFixed(1)}s`;
  }, 100);
  try {
    const [url, assets] = await Promise.all([
      loadWorker(),
      loadRuntime(javaVersion, ({ loaded, total }) => {
        if (!active || requestId !== id) return;
        progress.value = loaded / total;
        element('loading-text').textContent =
          `Loading Java · ${(loaded / 1024 / 1024).toFixed(1)} / ${(total / 1024 / 1024).toFixed(1)} MiB`;
      }),
    ]);
    if (!active || requestId !== id) return;
    worker = new Worker(url, { type: 'module' });
    worker.onmessage = ({ data: event }: MessageEvent<Event>) => {
      if (event.id !== id || id !== requestId || !worker) return;
      switch (event.type) {
        case 'phase':
          if (event.phase !== 'loading') {
            loading.hidden = true;
            const compiling = event.phase === 'compiling';
            setStatus(compiling ? 'Compiling…' : 'Running…', 'busy');
            timeout(
              compiling ? COMPILE_TIMEOUT : RUN_TIMEOUT,
              compiling ? 'Compilation' : 'Execution',
            );
          }
          break;
        case 'progress':
          progress.value = event.loaded / event.total;
          element('loading-text').textContent =
            `Loading Java · ${(event.loaded / 1024 / 1024).toFixed(1)} / ${(event.total / 1024 / 1024).toFixed(1)} MiB`;
          break;
        case 'output':
          append(event.text, event.stream);
          break;
        case 'compiled':
          if (action === 'compile')
            append(
              `Compilation successful · ${event.classes} class${event.classes === 1 ? '' : 'es'} generated.\n`,
              'notice',
            );
          break;
        case 'done':
          if (!output.textContent && pending.length === 0)
            append('Program finished without output.\n', 'notice');
          finish(
            action === 'compile'
              ? 'Compiled successfully'
              : event.exitCode === undefined
                ? 'Finished successfully'
                : `Exited with code ${event.exitCode}`,
          );
          break;
        case 'error':
          append(`\n${event.message}\n`, 'stderr');
          finish('Could not complete · try again', 'error');
          break;
      }
    };
    worker.onerror = (event) => {
      if (id !== requestId || !worker) return;
      append(
        `\n${event.message || 'The Java runtime stopped unexpectedly. Try again.'}\n`,
        'stderr',
      );
      finish('Runtime error · try again', 'error');
    };
    worker.postMessage({
      request: { id, action, className: name, source, javaVersion } satisfies Request,
      assets,
    });
  } catch (error) {
    if (!active || requestId !== id) return;
    append(
      `${error instanceof Error ? error.message : 'WebAssembly and Web Workers are required.'}\n`,
      'stderr',
    );
    finish('Runtime unavailable', 'error');
  }
}

function updateVersion() {
  const version = Number(versionPicker.value);
  element('language-label').textContent = `JAVA ${version}`;
  element('runtime-label').textContent = `Java ${version}`;
  for (const option of examplePicker.options) {
    const example = examples[option.value as keyof typeof examples];
    option.disabled = example.minimumVersion > version;
    option.textContent =
      example.title + (option.disabled ? ` (Java ${example.minimumVersion}+)` : '');
  }
}
updateVersion();
versionPicker.onchange = () => {
  updateVersion();
  save();
};
runButton.onclick = () => start('run');
compileButton.onclick = () => start('compile');
stopButton.onclick = () => {
  append('\nStopped.\n', 'notice');
  finish('Stopped');
};
element('clear').onclick = clearOutput;
className.oninput = () => {
  className.setCustomValidity('');
  element('filename').textContent = `${className.value.split('.').at(-1) || 'Main'}.java`;
  save();
};
className.oninput(new InputEvent('input'));
examplePicker.onchange = () => {
  const example = examples[examplePicker.value as keyof typeof examples];
  editor.dispatch({ changes: { from: 0, to: editor.state.doc.length, insert: example.source } });
  className.value = 'Main';
  element('filename').textContent = 'Main.java';
  save();
  editor.focus();
};
window.addEventListener('beforeunload', () => {
  try {
    localStorage.setItem(
      storageKey,
      JSON.stringify({
        source: editor.state.doc.toString(),
        className: className.value,
        javaVersion: Number(versionPicker.value),
      }),
    );
  } catch {
    /* Optional storage. */
  }
});
if (!navigator.userAgent.includes('Mac'))
  document.querySelector('.shortcut')!.textContent = 'Ctrl ↵';
