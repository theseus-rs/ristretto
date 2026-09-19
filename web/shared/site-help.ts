const playgroundHelp = `
  <section class="help-section">
    <h3>Write, check, run</h3>
    <p id="main-help">Use a <code>public static void main(String[] args)</code> entry point. Set Main class to its fully qualified name when using a package.</p>
    <p>Compile checks Java source. Check validates scripts without running their bodies; Clojure Check validates reader syntax only. Run starts a fresh program, including any compilation, and checks execution errors.</p>
    <div class="help-shortcut"><span>Run your code</span><kbd>⌘ / Ctrl + Enter</kbd></div>
  </section>
  <section class="help-section">
    <h3>On your device</h3>
    <p>The first run of each version downloads its Java runtime. Later runs reuse locally cached assets when browser storage is available. Each language and Scala generation keeps its own draft when you switch.</p>
  </section>
  <section class="help-section">
    <h3>Runtime &amp; limits</h3>
    <p>Java and the selected language’s standard libraries are included. Standard input is closed and arguments are empty. External dependencies, GUI, networking, and subprocesses are unavailable. Some Java APIs may be unsupported.</p>
    <p>Compilation, script checking, and script execution have a 10-minute limit; Java execution has a 30-second limit. Output is limited to 1 MiB. Compilation may take several minutes in Firefox. Stop interrupts either phase.</p>
  </section>`;

const jshellHelp = `
  <section class="help-section">
    <h3>A session that remembers</h3>
    <p>Variables, objects, imports, and methods stay until you reset, exit, switch Java versions, or reload. Previous snippets are not rerun with new input.</p>
    <p>Input history and your Java version are saved on this device when storage is available. Reloading starts a fresh session and keeps your history.</p>
  </section>
  <section class="help-section">
    <h3>At your fingertips</h3>
    <div class="help-shortcut"><span>Submit a line</span><kbd>Enter</kbd></div>
    <div class="help-shortcut"><span>Insert a line break</span><kbd>Shift + Enter</kbd></div>
    <div class="help-shortcut"><span>Recall input</span><kbd>↑ / ↓</kbd></div>
    <div class="help-shortcut"><span>Complete code or commands</span><kbd>Tab</kbd></div>
    <div class="help-shortcut"><span>Cancel incomplete input</span><kbd>Ctrl + C</kbd></div>
    <p>Paste multiline code, or keep typing when the prompt shows <code>...&gt;</code>.</p>
  </section>
  <section class="help-section">
    <h3>Useful commands</h3>
    <p class="help-commands"><code>/help</code> <code>/vars</code> <code>/methods</code> <code>/types</code> <code>/imports</code> <code>/list</code> <code>/drop</code> <code>/reset</code> <code>/exit</code></p>
    <p>Open script and <code>/open</code> use a file picker. <code>/save</code> downloads a script.</p>
  </section>
  <section class="help-section">
    <h3>Runtime &amp; limits</h3>
    <p>Ctrl+C cancels incomplete input without losing state. Interrupting running code, reaching the 10-minute submission limit, or exceeding 1 MiB of output ends the session.</p>
    <p>Standard input is closed. External dependencies, GUI, networking, and subprocesses are unavailable. Some Java APIs may be unsupported.</p>
  </section>`;

export function siteHelp(product: 'playground' | 'jshell') {
  return `<details class="header-help">
    <summary aria-label="Good to know" title="Good to know" aria-controls="site-help">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="M9.5 9a2.5 2.5 0 1 1 4 2c-1 .7-1.5 1-1.5 2.5" stroke-linecap="round"/><circle cx="12" cy="17" r=".9" fill="currentColor" stroke="none"/></svg>
      <span>Good to know</span>
    </summary>
    <section id="site-help" class="help-card" aria-labelledby="help-title">
      <div class="help-heading"><div><span class="help-eyebrow">${product === 'playground' ? 'Playground' : 'JShell'} guide</span><h2 id="help-title">Good to know</h2></div><button id="close-help" class="help-close" type="button" aria-label="Close guide"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" aria-hidden="true"><path d="m6 6 12 12M18 6 6 12"/></svg></button></div>
      <div class="help-content">${product === 'playground' ? playgroundHelp : jshellHelp}</div>
      <a class="help-notices" href="${import.meta.env.BASE_URL}notices.html" target="_blank" rel="noreferrer">Third-party notices <span aria-hidden="true">↗</span></a>
    </section>
  </details>`;
}
