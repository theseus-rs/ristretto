import { EditorView, keymap } from '@codemirror/view';
import { Compartment } from '@codemirror/state';
import { java } from '@codemirror/lang-java';
import { editorTheme } from '../shared/editor-theme';
import { selectedTheme, initializeAppearance } from '../shared/appearance';
import { siteHeader, initializeSiteHeader } from '../shared/site-header';
import { loadWorker } from '../shared/worker-client';
import { loadRuntime } from '../shared/runtime';
import {
  COMPILE_TIMEOUT,
  LOAD_TIMEOUT,
  JAVA_VERSIONS,
  isJavaVersion,
  type Event,
  type JavaVersion,
  type Request,
} from '../shared/protocol';
import '../shared/style.css';
import './style.css';

const app = document.getElementById('app')!;
app.innerHTML = `${siteHeader('jshell')}
  <main class="shell-main">
    <section class="workbench shell-workbench" aria-label="JShell terminal">
      <div class="toolbar"><div class="version-picker"><label for="java-version">Java version</label><select id="java-version">${JAVA_VERSIONS.filter(
        (v) => v >= 11,
      )
        .map((v) => `<option value="${v}" ${v === 25 ? 'selected' : ''}>${v}</option>`)
        .join(
          '',
        )}</select><span class="session-label">Interactive session</span></div><div class="actions"><button id="shell-open" class="button secondary">Open script</button><button id="shell-reset" class="button secondary">Reset session</button><button id="shell-interrupt" class="button stop" disabled>■ Interrupt</button></div></div>
      <div class="shell-console-header"><span class="console-label"><span aria-hidden="true">›_</span> JShell</span><div><button id="shell-help" class="text-button">Help</button><button id="shell-clear" class="text-button">Clear screen</button></div></div>
      <div id="shell-scroll" class="shell-scroll"><pre id="shell-output" role="log" aria-label="JShell transcript" aria-live="polite" aria-relevant="additions text" tabindex="0"></pre></div>
      <div id="shell-completions" role="listbox" aria-label="Completions" hidden></div>
      <div class="shell-prompt-row"><span id="shell-prompt" aria-hidden="true">jshell&gt;</span><div id="shell-input"></div><button id="shell-submit" class="button primary" aria-label="Submit input" title="Submit input (Enter)">Enter <span aria-hidden="true">↵</span></button></div>
      <div id="shell-loading"><progress id="shell-progress" max="1" value="0" aria-label="Java runtime download"></progress><span id="shell-loading-text">Loading Java…</span></div>
      <div class="console-footer"><span id="shell-status" role="status" aria-live="polite" data-state="busy"><span class="status-dot"></span><span id="shell-status-text">Starting JShell…</span></span><span id="shell-elapsed"></span></div>
    </section>
    <footer class="workspace-footer"><span><span class="status-dot"></span> Powered by Ristretto <span class="footer-divider">·</span> <span id="runtime-label">Java 25</span></span><span class="shell-shortcuts"><kbd>↑ ↓</kbd> history <span>·</span> <kbd>Tab</kbd> complete <span>·</span> <kbd>Ctrl+C</kbd> cancel</span><details><summary>Good to know <span aria-hidden="true">＋</span></summary><div class="help-card"><p>Variables, objects, imports, and methods stay in this session until you reset, exit, switch Java versions, or reload the page. Previous snippets are not rerun when you submit new input.</p><p>Enter submits a line. JShell shows <code>...&gt;</code> when it needs more input. Shift+Enter inserts a line break; you can also paste multiline code. Up and Down recall your input. Tab completes code and commands.</p><p>Use <code>/help</code>, <code>/vars</code>, <code>/methods</code>, <code>/types</code>, <code>/imports</code>, <code>/list</code>, <code>/drop</code>, <code>/reset</code>, and <code>/exit</code>. Open script and <code>/open</code> use a file picker; <code>/save</code> downloads a script.</p><p>Ctrl+C cancels incomplete input without losing state. Interrupting running code, a time limit, or 1 MiB of output ends the current session. Evaluation is limited to 10 minutes per submission. Standard input is closed; external dependencies, GUI, networking, and subprocesses are unavailable. Some Java APIs may be unsupported.</p><p>Input history and your Java version are saved on this device when storage is available. Reloading starts a fresh session and keeps your history available for recall.</p><a href="${import.meta.env.BASE_URL}notices.html" target="_blank" rel="noreferrer">Third-party notices ↗</a></div></details></footer>
    <input id="shell-file" type="file" accept=".jsh,.java,text/plain" hidden />
  </main>`;

const element = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
const version = element<HTMLSelectElement>('java-version');
const output = element<HTMLPreElement>('shell-output');
const scroll = element<HTMLDivElement>('shell-scroll');
const prompt = element<HTMLSpanElement>('shell-prompt');
const status = element<HTMLSpanElement>('shell-status');
const submitButton = element<HTMLButtonElement>('shell-submit');
const resetButton = element<HTMLButtonElement>('shell-reset');
const interruptButton = element<HTMLButtonElement>('shell-interrupt');
const openButton = element<HTMLButtonElement>('shell-open');
const helpButton = element<HTMLButtonElement>('shell-help');
const loading = element<HTMLDivElement>('shell-loading');
const progress = element<HTMLProgressElement>('shell-progress');
const completionList = element<HTMLDivElement>('shell-completions');
const fileInput = element<HTMLInputElement>('shell-file');
const storageKey = 'ristretto-jshell-history-v1';
let history: string[] = [];
let historyIndex = 0;
let draft = '';
try {
  const saved = JSON.parse(localStorage.getItem(storageKey) ?? 'null');
  if (isJavaVersion(saved?.javaVersion) && saved.javaVersion >= 11)
    version.value = String(saved.javaVersion);
  if (Array.isArray(saved?.history))
    history = saved.history.filter((line: unknown) => typeof line === 'string').slice(-200);
  if (typeof saved?.draft === 'string') draft = saved.draft;
} catch {
  /* History is optional. */
}
historyIndex = history.length;

let worker: Worker | undefined;
let requestId = 0;
let generation = 0;
let busy = false;
let continuation = false;
let closed = false;
let watchdog: ReturnType<typeof setTimeout>;
let started = 0;
let activeOperation: Request['operation'];
let completionSource = '';
let completionCursor = 0;
let candidates: string[] = [];
let candidateIndex = -1;
let completionAnchor = 0;
let focusOnReady = false;
const theme = new Compartment();
const editable = new Compartment();

const editor = new EditorView({
  doc: draft,
  extensions: [
    java(),
    theme.of(editorTheme(selectedTheme())),
    editable.of(EditorView.editable.of(true)),
    EditorView.lineWrapping,
    EditorView.contentAttributes.of({
      'aria-label': 'JShell input',
      spellcheck: 'false',
      autocapitalize: 'off',
      autocomplete: 'off',
    }),
    keymap.of([
      {
        key: 'Enter',
        run: () => {
          if (candidateIndex >= 0) chooseCompletion(candidateIndex);
          else void submit();
          return true;
        },
      },
      {
        key: 'Shift-Enter',
        run: (view) => {
          view.dispatch(view.state.replaceSelection('\n'));
          return true;
        },
      },
      { key: 'ArrowUp', run: () => recall(-1) },
      { key: 'ArrowDown', run: () => recall(1) },
      {
        key: 'Tab',
        run: () => {
          void complete();
          return true;
        },
      },
      {
        key: 'Escape',
        run: () => {
          hideCompletions();
          return true;
        },
      },
      {
        key: 'Ctrl-d',
        run: () => {
          if (editor.state.doc.length) return false;
          void submit('/exit');
          return true;
        },
      },
    ]),
    EditorView.updateListener.of((update) => {
      if (update.docChanged) {
        hideCompletions();
        save();
      }
    }),
    EditorView.theme({
      '&': { fontSize: '14px' },
      '.cm-scroller': {
        fontFamily: '"SFMono-Regular", Consolas, "Liberation Mono", monospace',
        lineHeight: '1.7',
      },
      '.cm-content': { padding: '0' },
      '.cm-line': { padding: '0' },
    }),
  ],
  parent: element('shell-input'),
});
initializeSiteHeader();
initializeAppearance((value) =>
  editor.dispatch({ effects: theme.reconfigure(editorTheme(value)) }),
);

function save() {
  try {
    localStorage.setItem(
      storageKey,
      JSON.stringify({
        history,
        javaVersion: Number(version.value),
        draft: editor.state.doc.toString(),
      }),
    );
  } catch {
    /* Sessions work without storage. */
  }
}
function setInput(text: string) {
  editor.dispatch({
    changes: { from: 0, to: editor.state.doc.length, insert: text },
    selection: { anchor: text.length },
  });
}
function append(text: string, stream = 'stdout') {
  const nearBottom = scroll.scrollHeight - scroll.scrollTop - scroll.clientHeight < 80;
  const last = output.lastElementChild;
  if (last?.className === stream) last.lastChild!.textContent += text;
  else {
    const span = document.createElement('span');
    span.className = stream;
    span.textContent = text;
    output.append(span);
  }
  // Bound retained scrollback across an arbitrarily long session.
  while (output.textContent!.length > 2 * 1024 * 1024 && output.childNodes.length > 1)
    output.firstChild!.remove();
  if (nearBottom) scroll.scrollTop = scroll.scrollHeight;
}
function echo(text: string) {
  append(
    `${continuation ? '   ...>' : 'jshell>'} ${text.replaceAll('\n', '\n   ...> ')}\n`,
    'shell-command',
  );
  scroll.scrollTop = scroll.scrollHeight;
}
function setStatus(text: string, state = 'ready') {
  element('shell-status-text').textContent = text;
  status.dataset.state = state;
}
function controls() {
  submitButton.disabled = busy || closed;
  resetButton.disabled = busy;
  resetButton.textContent = closed ? 'Start session' : 'Reset session';
  openButton.disabled = helpButton.disabled = busy || closed || continuation;
  interruptButton.disabled = !busy && !continuation;
  version.disabled = busy;
  editor.dispatch({ effects: editable.reconfigure(EditorView.editable.of(!closed && !busy)) });
  prompt.textContent = closed ? 'exited' : continuation ? '   ...>' : 'jshell>';
}
function finish() {
  clearTimeout(watchdog);
  busy = false;
  loading.hidden = true;
  if (activeOperation === 'input' && started)
    element('shell-elapsed').textContent = `${((performance.now() - started) / 1000).toFixed(2)}s`;
  controls();
  if (focusOnReady && !closed) editor.focus();
  focusOnReady = false;
}
function abandon(message: string, state = 'error') {
  generation++;
  requestId++;
  worker?.terminate();
  worker = undefined;
  continuation = false;
  closed = true;
  hideCompletions();
  append(`|  ${message}\n|  Use Start session to continue with a fresh state.\n`, 'stderr');
  finish();
  setStatus('Session ended', state);
}
function timeout(milliseconds: number) {
  clearTimeout(watchdog);
  watchdog = setTimeout(
    () => abandon(`Time limit reached (${milliseconds / 1000} seconds).`),
    milliseconds,
  );
}
function receive(event: Event) {
  if (!worker || event.id !== requestId) return;
  switch (event.type) {
    case 'phase':
      if (event.phase === 'evaluating') {
        loading.hidden = true;
        setStatus(activeOperation === 'complete' ? 'Completing…' : 'Evaluating…', 'busy');
        timeout(COMPILE_TIMEOUT);
      }
      break;
    case 'output':
      append(event.text, event.stream);
      break;
    case 'ready':
      continuation = event.continuation;
      closed = event.closed;
      if (closed) {
        worker.terminate();
        worker = undefined;
      }
      finish();
      setStatus(closed ? 'Session ended' : continuation ? 'Continue your snippet…' : 'Ready');
      if (event.edit !== undefined) {
        setInput(event.edit.trimEnd());
        editor.focus();
      }
      if (event.download !== undefined) download(event.download, event.filename ?? 'session.jsh');
      break;
    case 'completions':
      finish();
      setStatus(continuation ? 'Continue your snippet…' : 'Ready');
      showCompletions(event.anchor, event.suggestions);
      break;
    case 'done':
      abandon(
        `Java session ended${event.exitCode === undefined ? '' : ` (exit ${event.exitCode})`}.`,
        'ready',
      );
      break;
    case 'error':
      abandon(event.message);
      break;
  }
}
async function request(source: string, operation: Request['operation'] = 'input') {
  if (busy || closed) return;
  busy = true;
  activeOperation = operation;
  const id = ++requestId;
  const currentGeneration = generation;
  started = performance.now();
  focusOnReady = true;
  hideCompletions();
  controls();
  const payload: Request = {
    id,
    javaVersion: Number(version.value) as JavaVersion,
    action: 'jshell',
    className: 'BrowserJShell',
    source,
    operation,
    cursor: completionCursor,
  };
  if (worker) {
    setStatus(operation === 'complete' ? 'Completing…' : 'Evaluating…', 'busy');
    timeout(COMPILE_TIMEOUT);
    worker.postMessage({ request: payload });
    return;
  }
  setStatus('Starting JShell…', 'busy');
  loading.hidden = false;
  progress.value = 0;
  timeout(LOAD_TIMEOUT);
  try {
    const [url, assets] = await Promise.all([
      loadWorker(),
      loadRuntime(payload.javaVersion, ({ loaded, total }) => {
        if (generation !== currentGeneration || requestId !== id) return;
        progress.value = loaded / total;
        element('shell-loading-text').textContent =
          `Loading Java · ${(loaded / 1024 / 1024).toFixed(1)} / ${(total / 1024 / 1024).toFixed(1)} MiB`;
      }),
    ]);
    if (generation !== currentGeneration || requestId !== id) return;
    worker = new Worker(url, { type: 'module' });
    worker.onmessage = ({ data }: MessageEvent<Event>) => receive(data);
    worker.onerror = (event) => {
      if (generation === currentGeneration)
        abandon(event.message || 'The Java runtime stopped unexpectedly.');
    };
    worker.postMessage({ request: payload, assets });
  } catch (error) {
    if (generation === currentGeneration && requestId === id)
      abandon(error instanceof Error ? error.message : String(error));
  }
}
async function submit(text = editor.state.doc.toString()) {
  if (busy || closed) return;
  if (!text.trim() && !continuation) return;
  hideCompletions();
  if (text.trim()) {
    if (history.at(-1) !== text) history.push(text);
    history = history.slice(-200);
    historyIndex = history.length;
    draft = '';
  }
  echo(text);
  setInput('');
  save();
  if (!continuation && /^\/open(?:\s|$)/.test(text.trim())) {
    fileInput.click();
    return;
  }
  if (!continuation && text.trim() === '/history') {
    append(history.join('\n') + '\n');
    editor.focus();
    return;
  }
  await request(text);
}
function recall(direction: number) {
  if (busy) return true;
  if (candidates.length) {
    candidateIndex = (candidateIndex + direction + candidates.length) % candidates.length;
    renderCompletions();
    return true;
  }
  const selection = editor.state.selection.main;
  const line = editor.state.doc.lineAt(selection.head);
  if (
    (direction < 0 && line.number !== 1) ||
    (direction > 0 && line.number !== editor.state.doc.lines)
  )
    return false;
  if (historyIndex === history.length) draft = editor.state.doc.toString();
  historyIndex = Math.max(0, Math.min(history.length, historyIndex + direction));
  setInput(historyIndex === history.length ? draft : history[historyIndex]);
  return true;
}
function hideCompletions() {
  candidates = [];
  candidateIndex = -1;
  completionList.hidden = true;
  completionList.replaceChildren();
}
async function complete() {
  if (busy || closed) return;
  if (candidates.length) {
    candidateIndex = (candidateIndex + 1) % candidates.length;
    renderCompletions();
    return;
  }
  completionSource = editor.state.doc.toString();
  completionCursor = editor.state.selection.main.head;
  await request(completionSource, 'complete');
}
function showCompletions(anchor: number, suggestions: string[]) {
  if (editor.state.doc.toString() !== completionSource || !suggestions.length) return;
  completionAnchor = anchor;
  candidates = suggestions;
  if (candidates.length === 1) {
    chooseCompletion(0);
    return;
  }
  let common = candidates[0];
  for (const candidate of candidates)
    while (!candidate.startsWith(common)) common = common.slice(0, -1);
  if (common.length > completionCursor - anchor) {
    const retained = candidates;
    editor.dispatch({
      changes: { from: anchor, to: completionCursor, insert: common },
      selection: { anchor: anchor + common.length },
    });
    completionCursor = anchor + common.length;
    completionSource = editor.state.doc.toString();
    candidates = retained;
  }
  renderCompletions();
}
function renderCompletions() {
  completionList.replaceChildren();
  completionList.hidden = false;
  candidates.forEach((candidate, index) => {
    const button = document.createElement('button');
    button.type = 'button';
    button.setAttribute('role', 'option');
    button.setAttribute('aria-selected', String(index === candidateIndex));
    button.textContent = candidate;
    button.onclick = () => chooseCompletion(index);
    completionList.append(button);
  });
}
function chooseCompletion(index: number) {
  const candidate = candidates[index];
  if (candidate === undefined) return;
  editor.dispatch({
    changes: { from: completionAnchor, to: completionCursor, insert: candidate },
    selection: { anchor: completionAnchor + candidate.length },
  });
  hideCompletions();
  editor.focus();
}
function interrupt() {
  if (busy) {
    abandon('Execution interrupted.');
    return;
  }
  if (closed) return;
  const text = editor.state.doc.toString();
  if (text) echo(text);
  append('^C\n', 'notice');
  setInput('');
  void request('', 'cancel');
}
function clearScreen() {
  output.replaceChildren();
  editor.focus();
}
function restart() {
  generation++;
  requestId++;
  worker?.terminate();
  worker = undefined;
  clearTimeout(watchdog);
  busy = continuation = closed = false;
  setInput('');
  element('runtime-label').textContent = `Java ${version.value}`;
  void request('');
}
function download(source: string, filename: string) {
  const url = URL.createObjectURL(new Blob([source], { type: 'text/plain;charset=utf-8' }));
  const link = document.createElement('a');
  link.href = url;
  link.download = filename.split(/[\\/]/).at(-1) || 'session.jsh';
  link.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}
submitButton.onclick = () => void submit();
resetButton.onclick = () => {
  if (closed) restart();
  else {
    echo('/reset');
    restart();
    append('|  Resetting state.\n', 'notice');
  }
};
interruptButton.onclick = interrupt;
helpButton.onclick = () => void submit('/help');
element('shell-clear').onclick = clearScreen;
openButton.onclick = () => fileInput.click();
fileInput.onchange = async () => {
  const file = fileInput.files?.[0];
  fileInput.value = '';
  if (!file || busy || closed) return;
  if (file.size > 1024 * 1024) {
    append('|  Script exceeds 1 MiB.\n', 'stderr');
    return;
  }
  try {
    await submit(await file.text());
  } catch (error) {
    append(`|  Could not read script: ${String(error)}\n`, 'stderr');
  }
};
version.onchange = () => {
  append(`|  Starting a new Java ${version.value} session.\n`, 'notice');
  restart();
  save();
};
document.addEventListener('keydown', (event) => {
  if (event.ctrlKey && event.key.toLowerCase() === 'c' && !window.getSelection()?.toString()) {
    event.preventDefault();
    interrupt();
  }
  if (event.ctrlKey && event.key.toLowerCase() === 'l') {
    event.preventDefault();
    clearScreen();
  }
});
window.addEventListener('beforeunload', save);
element('runtime-label').textContent = `Java ${version.value}`;
void request('');
