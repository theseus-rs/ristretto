import java.io.ByteArrayOutputStream;
import java.io.PrintStream;
import java.lang.reflect.InvocationTargetException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

public final class ScalaScript {
    private static void compile(String source) throws Exception {
        boolean scala2 = System.getProperty("ristretto.scala.version").equals("2.13");
        Path workspace = Path.of(System.getProperty("user.dir"));
        Path output = workspace.resolve("classes");
        Files.createDirectories(output);
        // Both wrappers keep user code as members, preserving top-level definitions/imports.
        String prefix = scala2 ? "object Main extends App {\n" : "class MainScript(val args: Array[String]) {\n";
        String suffix = scala2 ? "\n}\n" : "\n}\nobject Main { def main(args: Array[String]): Unit = { new MainScript(args); () } }\n";
        Path file = workspace.resolve("Main.scala");
        Files.writeString(file, prefix + source + suffix, StandardCharsets.UTF_8);
        String[] arguments = {"-classpath", System.getProperty("java.class.path"), "-d", output.toString(), "-color:never", file.toString()};
        if (scala2) arguments = new String[] {"-classpath", System.getProperty("java.class.path"), "-d", output.toString(), file.toString()};
        Class<?> compiler = Class.forName(scala2 ? "scala.tools.nsc.Main" : "dotty.tools.dotc.Main");
        Object result;
        PrintStream original = System.err;
        ByteArrayOutputStream diagnostics = new ByteArrayOutputStream();
        try {
            System.setErr(new PrintStream(diagnostics, true, StandardCharsets.UTF_8));
            result = compiler.getMethod("process", String[].class).invoke(null, (Object) arguments);
        } catch (InvocationTargetException exception) {
            throw new RuntimeException(exception.getCause());
        } finally {
            System.setErr(original);
            String text = diagnostics.toString(StandardCharsets.UTF_8);
            original.print(java.util.regex.Pattern.compile("Main\\.scala:(\\d+)").matcher(text)
                .replaceAll(match -> "Main.sc:" + Math.max(1, Integer.parseInt(match.group(1)) - 1)));
        }
        boolean failed = scala2 ? !((Boolean) result) : (Boolean) result.getClass().getMethod("hasErrors").invoke(result);
        if (failed) throw new IllegalArgumentException("Scala script contains compilation errors");
    }

    public static void check(String source) throws Exception {
        compile(source);
    }

    public static void run(String source) throws Exception {
        compile(source);
        try {
            Class.forName("Main").getMethod("main", String[].class).invoke(null, (Object) new String[0]);
        } catch (InvocationTargetException exception) {
            remapStack(exception.getCause());
            throw new RuntimeException(exception.getCause());
        }
    }

    private static void remapStack(Throwable throwable) {
        if (throwable == null) return;
        StackTraceElement[] trace = throwable.getStackTrace();
        for (int i = 0; i < trace.length; i++) {
            StackTraceElement frame = trace[i];
            if ("Main.scala".equals(frame.getFileName()))
                trace[i] = new StackTraceElement(frame.getClassName(), frame.getMethodName(), "Main.sc", Math.max(1, frame.getLineNumber() - 1));
        }
        throwable.setStackTrace(trace);
        if (throwable.getCause() != throwable) remapStack(throwable.getCause());
    }
}
