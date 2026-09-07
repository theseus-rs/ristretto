import java.lang.reflect.Array;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.IdentityHashMap;
import java.util.Iterator;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.function.BiFunction;
import java.util.function.Supplier;
import jdk.jshell.*;
import jdk.jshell.execution.DirectExecutionControl;
import jdk.jshell.spi.*;

/** One JShell instance lives for the lifetime of the browser worker's VM. */
public final class BrowserJShell {
    private static JShell shell;
    private static String pending = "";
    private static String feedback = "normal";
    private static int nextId = 1;
    private static int startupId;
    private static boolean starting;
    private static boolean closed;
    private static boolean reset;
    private static final List<String> history = new ArrayList<>();
    private static final List<ReplayEntry> replay = new ArrayList<>();
    private static final class ReplayEntry {
        final String source;
        final boolean drop;
        ReplayEntry(String source, boolean drop) { this.source = source; this.drop = drop; }
    }
    private static final String[] COMMANDS = {"/!", "/?", "/drop", "/edit", "/env", "/exit",
            "/help", "/history", "/imports", "/list", "/methods", "/open", "/reload",
            "/reset", "/save", "/set", "/types", "/vars"};

    private static void start(boolean welcome) {
        ExecutionControlProvider provider = new ExecutionControlProvider() {
            public String name() { return "ristretto"; }
            public ExecutionControl generate(ExecutionEnv env, Map<String, String> parameters) {
                return new DirectExecutionControl() {
                    protected String invoke(Method method) throws Exception {
                        // Older JDKs' stock value renderer uses primitive stream adapters that
                        // Ristretto does not implement yet. Keep rendering local and portable.
                        return renderValue(method.invoke(null), new IdentityHashMap<>());
                    }

                    public String varValue(String className, String name) throws InternalException {
                        try {
                            Field field = findClass(className).getDeclaredField(name);
                            field.setAccessible(true);
                            return renderValue(field.get(null), new IdentityHashMap<>());
                        } catch (Exception error) {
                            throw new InternalException(error.toString());
                        }
                    }

                    public String invoke(String className, String methodName)
                            throws RunException, InternalException, EngineTerminationException {
                        try {
                            return super.invoke(className, methodName);
                        } catch (InternalException error) {
                            // JShell otherwise logs engine failures only to its debug channel.
                            // Surface unsupported VM operations as evaluation diagnostics.
                            throw new UserException(error.getMessage(), error.getClass().getName(),
                                    error.getStackTrace());
                        }
                    }
                };
            }
        };
        nextId = 1;
        startupId = 0;
        starting = true;
        shell = JShell.builder().executionEngine(provider, null)
                // Snippets use modules and generated classes, not the host bridge archive.
                .compilerOptions("-classpath", "")
                // Anonymous classes also work on older JDKs without reflective lambda access.
                .idGenerator(new BiFunction<Snippet, Integer, String>() {
                    public String apply(Snippet snippet, Integer index) {
                        return starting ? "s" + (++startupId) : "" + nextId++;
                    }
                })
                .tempVariableNameGenerator(new Supplier<String>() {
                    public String get() { return "$" + nextId; }
                })
                .in(System.in).out(System.out).err(System.err).build();
        for (String name : new String[] {"java.io", "java.math", "java.net", "java.nio.file",
                "java.util", "java.util.concurrent", "java.util.function", "java.util.prefs",
                "java.util.regex", "java.util.stream"}) {
            shell.eval("import " + name + ".*;");
        }
        starting = false;
        if (welcome) {
            notice("Welcome to JShell -- Version " + System.getProperty("java.version"));
            notice("For an introduction type: /help intro");
        }
    }

    public static String input(String text) {
        if (shell == null) start(true);
        if (!text.isBlank()) history.add(text);
        reset = false;
        String extra = "";
        for (String line : text.split("\\r\\n|\\r|\\n", -1)) {
            String stripped = line.strip();
            if (pending.isBlank() && stripped.startsWith("/")
                    && !stripped.startsWith("//") && !stripped.startsWith("/*")) {
                extra = command(line.strip());
            } else {
                accept(line);
            }
            if (closed || reset || !extra.isEmpty()) break;
        }
        return ready(extra);
    }

    private static void accept(String line) {
        pending += line + "\n";
        while (!pending.isBlank()) {
            SourceCodeAnalysis.CompletionInfo completion = shell.sourceCodeAnalysis().analyzeCompletion(pending);
            if (completion.completeness() == SourceCodeAnalysis.Completeness.EMPTY) {
                pending = "";
                return;
            }
            if (!completion.completeness().isComplete()) return;
            String source = completion.source();
            pending = completion.remaining();
            evaluate(source);
        }
        pending = "";
    }

    private static void evaluate(String source) {
        for (SnippetEvent event : shell.eval(source)) {
            if (event.causeSnippet() != null) continue;
            Snippet snippet = event.snippet();
            for (Iterator<Diag> diagnostics = shell.diagnostics(snippet).iterator(); diagnostics.hasNext();) {
                Diag diagnostic = diagnostics.next();
                error(diagnostic.isError() ? "Error:" : "Warning:");
                error(diagnostic.getMessage(Locale.ROOT));
                long position = diagnostic.getPosition();
                if (position >= 0 && position <= source.length()) {
                    int offset = (int) position;
                    int start = source.lastIndexOf('\n', offset - 1) + 1;
                    int end = source.indexOf('\n', offset);
                    error(source.substring(start, end < 0 ? source.length() : end));
                    error(" ".repeat(offset - start) + "^");
                }
            }
            if (event.status().isActive()) replay.add(new ReplayEntry(source, false));
            if (event.exception() != null) {
                JShellException exception = event.exception();
                error("Exception " + (exception instanceof EvalException
                        ? ((EvalException) exception).getExceptionClassName() + ": " + exception.getMessage()
                        : exception.toString()));
                for (StackTraceElement frame : exception.getStackTrace()) {
                    if (frame.getFileName() != null && frame.getFileName().startsWith("#"))
                        error("    at " + frame);
                }
            } else if (!feedback.equals("silent") && event.status() != Snippet.Status.REJECTED) {
                if (event.value() != null && !event.value().isEmpty()) {
                    String name = snippet instanceof VarSnippet ? ((VarSnippet) snippet).name()
                            : snippet instanceof ExpressionSnippet ? ((ExpressionSnippet) snippet).name() : "";
                    System.out.println((name.isEmpty() ? "" : name + " ==> ") + event.value());
                } else if (snippet instanceof MethodSnippet && !feedback.equals("concise")) {
                    MethodSnippet method = (MethodSnippet) snippet;
                    notice("created method " + method.name() + "(" + method.parameterTypes() + ")");
                } else if (snippet instanceof TypeDeclSnippet && !feedback.equals("concise")) {
                    notice("created " + typeName(snippet) + " " + ((TypeDeclSnippet) snippet).name());
                }
                if (snippet instanceof DeclarationSnippet && event.status() != Snippet.Status.VALID) {
                    for (Iterator<String> missing = shell.unresolvedDependencies((DeclarationSnippet) snippet).iterator(); missing.hasNext();)
                        notice("cannot be used until " + missing.next() + " is declared");
                }
            }
        }
    }

    private static String command(String input) {
        String[] parts = input.split("\\s+", 2);
        String name = parts[0];
        String argument = parts.length == 2 ? parts[1].strip() : "";
        List<String> matches = new ArrayList<>();
        for (String candidate : COMMANDS) if (candidate.startsWith(name)) matches.add(candidate);
        if (matches.size() == 1) name = matches.get(0);
        switch (name) {
            case "/help": case "/?":
                notice("Enter Java expressions, declarations, statements, or imports.");
                notice("Enter submits a line; incomplete snippets continue at ...>.");
                notice("Tab completes code and commands. Up/Down recall input. Ctrl+L clears the screen.");
                notice("/list [name|id|-all|-start]  list snippets");
                notice("/vars /methods /types /imports  inspect the session");
                notice("/drop <name|id>  drop a declaration");
                notice("/edit [name|id]  edit snippets in the prompt");
                notice("/history  show input; /! or /<id> or /-<n>  rerun a snippet");
                notice("/reset  clear state; /reload  reset and replay successful snippets");
                notice("/open  choose a local script; /save [file]  download active snippets");
                notice("/set feedback normal|concise|silent|verbose  change feedback");
                notice("/exit  end this session");
                notice("Ctrl+C cancels incomplete input. Interrupting running code resets this browser session.");
                break;
            case "/list":
                for (Snippet snippet : selected(argument))
                    System.out.println("  " + snippet.id() + " : " + snippet.source().strip());
                break;
            case "/vars":
                for (Snippet snippet : selected(argument)) if (snippet instanceof VarSnippet) {
                    VarSnippet variable = (VarSnippet) snippet;
                    notice("  " + variable.typeName() + " " + variable.name() + " = "
                            + (shell.status(variable) == Snippet.Status.VALID ? shell.varValue(variable) : "(not active)"));
                }
                break;
            case "/methods":
                for (Snippet snippet : selected(argument)) if (snippet instanceof MethodSnippet) {
                    MethodSnippet method = (MethodSnippet) snippet;
                    notice("  " + method.name() + method.signature());
                }
                break;
            case "/types":
                for (Snippet snippet : selected(argument)) if (snippet instanceof TypeDeclSnippet)
                    notice("  " + typeName(snippet) + " " + ((TypeDeclSnippet) snippet).name());
                break;
            case "/imports":
                for (Iterator<ImportSnippet> imports = shell.imports().iterator(); imports.hasNext();)
                    notice("  " + imports.next().source());
                break;
            case "/drop":
                if (argument.isEmpty()) { error("Specify a snippet name or id: /drop <name|id>"); break; }
                for (Snippet snippet : selected(argument)) {
                    if (!(snippet instanceof PersistentSnippet) || !shell.status(snippet).isActive()) {
                        error("Snippet " + snippet.id() + " cannot be dropped.");
                        continue;
                    }
                    shell.drop(snippet);
                    replay.add(new ReplayEntry(snippet.source(), true));
                    notice("dropped " + ((PersistentSnippet) snippet).name());
                }
                break;
            case "/history":
                for (String entry : history) System.out.println(entry);
                break;
            case "/reset":
                if (!argument.isEmpty()) { error("JVM options are unavailable in the browser."); break; }
                shell.close();
                reset = true;
                pending = "";
                notice("Resetting state.");
                break;
            case "/reload":
                if (!argument.isEmpty()) { error("Use /reload without options in the browser."); break; }
                notice("Restarting and restoring state.");
                // The host recreates the VM so generated classes from the old session cannot
                // collide with new classes whose snippet IDs have changed during replay.
                StringBuilder restore = new StringBuilder(",\"reload\":{\"feedback\":")
                        .append(json(feedback)).append(",\"entries\":[");
                for (int i = 0; i < replay.size(); i++) {
                    if (i > 0) restore.append(',');
                    ReplayEntry entry = replay.get(i);
                    restore.append("{\"source\":").append(json(entry.source))
                            .append(",\"drop\":").append(entry.drop).append('}');
                }
                return restore.append("]}").toString();
            case "/exit":
                shell.close();
                closed = true;
                pending = "";
                notice("Goodbye");
                break;
            case "/edit":
                return ",\"edit\":" + json(sources(selected(argument)));
            case "/save":
                String filename = argument.isEmpty() || argument.startsWith("-") ? "session.jsh" : argument;
                String source = argument.equals("-history") ? String.join("\n", history)
                        : sources(selected(argument.startsWith("-") ? argument : ""));
                return ",\"download\":" + json(source) + ",\"filename\":" + json(filename);
            case "/open":
                error("Use Open script to choose a file from your device.");
                break;
            case "/set":
                if (argument.isEmpty()) { notice("/set feedback " + feedback); break; }
                if (argument.matches("feedback (normal|concise|silent|verbose)")) {
                    feedback = argument.substring(9);
                    notice("Feedback mode: " + feedback);
                } else error("Use /set feedback normal|concise|silent|verbose");
                break;
            case "/env":
                notice("Java " + System.getProperty("java.version") + " in your browser. External classpaths and JVM options are unavailable.");
                break;
            default:
                List<Snippet> snippets = selected("-all");
                for (Iterator<Snippet> iterator = snippets.iterator(); iterator.hasNext();)
                    if (iterator.next().id().startsWith("s")) iterator.remove();
                Snippet repeat = null;
                if (name.equals("/!") || name.matches("/-[1-9][0-9]*")) {
                    int distance;
                    try { distance = name.equals("/!") ? 1 : Integer.parseInt(name.substring(2)); }
                    catch (NumberFormatException error) { distance = Integer.MAX_VALUE; }
                    if (distance <= snippets.size()) repeat = snippets.get(snippets.size() - distance);
                } else if (name.matches("/[0-9]+")) {
                    for (Snippet snippet : snippets) if (snippet.id().equals(name.substring(1))) repeat = snippet;
                }
                if (repeat != null) {
                    System.out.println(repeat.source().strip());
                    evaluate(repeat.source());
                } else error((matches.size() > 1 ? "Ambiguous command: " : "Unknown command or snippet: ") + input + ". Type /help for help.");
        }
        return "";
    }

    private static List<Snippet> selected(String argument) {
        List<Snippet> result = new ArrayList<>();
        for (Iterator<Snippet> snippets = shell.snippets().iterator(); snippets.hasNext();) {
            Snippet snippet = snippets.next();
            boolean startup = snippet.id().startsWith("s");
            if (argument.equals("-start") ? startup
                    : argument.equals("-all") ? true
                    : argument.isEmpty() ? !startup && shell.status(snippet).isActive()
                    : argument.equals(snippet.id()) || snippet instanceof PersistentSnippet
                            && argument.equals(((PersistentSnippet) snippet).name())) result.add(snippet);
        }
        if (!argument.isEmpty() && !argument.startsWith("-") && result.isEmpty()) error("No such snippet: " + argument);
        if (argument.startsWith("-") && !argument.equals("-all") && !argument.equals("-start")) error("Unknown option: " + argument);
        return result;
    }

    private static String sources(List<Snippet> snippets) {
        StringBuilder result = new StringBuilder();
        for (Snippet snippet : snippets) result.append(snippet.source().strip()).append('\n');
        return result.toString();
    }

    private static String typeName(Snippet snippet) {
        if (snippet.source().strip().startsWith("record ")) return "record";
        return snippet.subKind().name().toLowerCase(Locale.ROOT).replace("_subkind", "");
    }

    public static String cancel() {
        pending = "";
        return ready("");
    }

    public static String beginReload(String mode) {
        start(false);
        feedback = mode;
        return ready("");
    }

    public static String dropSource(String source) {
        // Rejected snippets consume IDs but are not replayed. Identify the original source
        // so a changed ID cannot drop another declaration (including an overloaded method).
        for (Snippet snippet : selected("")) {
            if (snippet.source().equals(source)) {
                shell.drop(snippet);
                replay.add(new ReplayEntry(source, true));
            }
        }
        return ready("");
    }

    public static String complete(String text, String position) {
        if (shell == null) start(true);
        int cursor = Math.max(0, Integer.parseInt(position));
        LinkedHashSet<String> suggestions = new LinkedHashSet<>();
        int anchor = 0;
        if (pending.isEmpty() && text.startsWith("/") && !text.contains(" ")) {
            for (String command : COMMANDS) if (command.startsWith(text)) suggestions.add(command);
        } else {
            int[] start = new int[1];
            String source = pending + text;
            for (SourceCodeAnalysis.Suggestion suggestion : shell.sourceCodeAnalysis()
                    .completionSuggestions(source, pending.length() + Math.min(cursor, text.length()), start)) {
                suggestions.add(suggestion.continuation());
                if (suggestions.size() >= 100) break;
            }
            anchor = Math.max(0, start[0] - pending.length());
        }
        StringBuilder result = new StringBuilder("{\"type\":\"completions\",\"anchor\":").append(anchor).append(",\"suggestions\":[");
        boolean first = true;
        for (String suggestion : suggestions) {
            if (!first) result.append(',');
            first = false;
            result.append(json(suggestion));
        }
        return result.append("]}").toString();
    }

    private static String ready(String extra) {
        return "{\"type\":\"ready\",\"continuation\":" + !pending.isBlank()
                + ",\"closed\":" + closed + ",\"reset\":" + reset + extra + "}";
    }

    private static void notice(String text) { System.out.println("|  " + text.replace("\n", "\n|  ")); }
    private static void error(String text) { System.err.println("|  " + text.replace("\n", "\n|  ")); }
    private static String json(String text) {
        StringBuilder result = new StringBuilder("\"");
        for (int i = 0; i < text.length(); i++) {
            char c = text.charAt(i);
            if (c == '"' || c == '\\') result.append('\\').append(c);
            else if (c < 32) {
                String hex = Integer.toHexString(c);
                result.append("\\u").append("0000", 0, 4 - hex.length()).append(hex);
            } else result.append(c);
        }
        return result.append('"').toString();
    }

    private static String renderValue(Object value, IdentityHashMap<Object, Boolean> seen) {
        if (value == null) return "null";
        if (value instanceof String || value instanceof Character) {
            char quote = value instanceof String ? '"' : '\'';
            StringBuilder text = new StringBuilder().append(quote);
            String raw = value.toString();
            for (int i = 0; i < raw.length(); i++) {
                char character = raw.charAt(i);
                if (character == quote || character == '\\') text.append('\\').append(character);
                else if (character == '\n') text.append("\\n");
                else if (character == '\r') text.append("\\r");
                else if (character == '\t') text.append("\\t");
                else if (character == '\b') text.append("\\b");
                else if (character == '\f') text.append("\\f");
                else text.append(character);
            }
            return text.append(quote).toString();
        }
        if (!value.getClass().isArray()) return value.toString();
        if (seen.put(value, Boolean.TRUE) != null) return "[...]";
        StringBuilder text = new StringBuilder(value.getClass().getComponentType().getTypeName())
                .append('[').append(Array.getLength(value)).append("] { ");
        for (int i = 0; i < Array.getLength(value); i++) {
            if (i > 0) text.append(", ");
            text.append(renderValue(Array.get(value, i), seen));
        }
        seen.remove(value);
        return text.append(" }").toString();
    }
}
