import java.io.File;
import java.util.Arrays;
import java.util.List;
import java.util.Map;

/**
 * Test java.lang.ProcessBuilder API methods including running OS commands
 * and verifying their output.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        testConstructor();
        testCommand();
        testEnvironment();
        testDirectory();
        testRedirectErrorStream();
        testRunEchoCommand();
        testExitValue();
    }

    private static void testConstructor() {
        System.out.println("=== ProcessBuilder Constructor ===");
        ProcessBuilder pb1 = new ProcessBuilder("echo", "hello");
        System.out.println("list constructor size: " + pb1.command().size());

        List<String> cmd = Arrays.asList("echo", "world");
        ProcessBuilder pb2 = new ProcessBuilder(cmd);
        System.out.println("varargs constructor size: " + pb2.command().size());
    }

    private static void testCommand() {
        System.out.println("=== ProcessBuilder.command() ===");
        ProcessBuilder pb = new ProcessBuilder("echo", "hello");
        List<String> cmd = pb.command();
        System.out.println("command[0]: " + cmd.get(0));
        System.out.println("command[1]: " + cmd.get(1));

        pb.command("ls", "-la");
        List<String> cmd2 = pb.command();
        System.out.println("updated command[0]: " + cmd2.get(0));
        System.out.println("updated command[1]: " + cmd2.get(1));
    }

    private static void testEnvironment() {
        System.out.println("=== ProcessBuilder.environment() ===");
        ProcessBuilder pb = new ProcessBuilder("echo");
        Map<String, String> env = pb.environment();
        System.out.println("env not null: " + (env != null));
        System.out.println("env not empty: " + !env.isEmpty());

        // Test setting a custom environment variable
        env.put("TEST_VAR", "test_value");
        System.out.println("custom var set: " + env.containsKey("TEST_VAR"));
        System.out.println("custom var value: " + env.get("TEST_VAR"));
    }

    private static void testDirectory() {
        System.out.println("=== ProcessBuilder.directory() ===");
        ProcessBuilder pb = new ProcessBuilder("echo");
        System.out.println("default directory: " + pb.directory());

        File tempDir = new File(System.getProperty("java.io.tmpdir"));
        pb.directory(tempDir);
        System.out.println("directory set: " + (pb.directory() != null));
        System.out.println("directory is temp: " + pb.directory().equals(tempDir));
    }

    private static void testRedirectErrorStream() {
        System.out.println("=== ProcessBuilder.redirectErrorStream() ===");
        ProcessBuilder pb = new ProcessBuilder("echo");
        System.out.println("default redirectErrorStream: " + pb.redirectErrorStream());

        pb.redirectErrorStream(true);
        System.out.println("after set true: " + pb.redirectErrorStream());

        pb.redirectErrorStream(false);
        System.out.println("after set false: " + pb.redirectErrorStream());
    }

    private static void testRunEchoCommand() throws Exception {
        System.out.println("=== Run Echo Command ===");
        String os = System.getProperty("os.name").toLowerCase();
        ProcessBuilder pb;
        if (os.contains("win")) {
            pb = new ProcessBuilder("cmd", "/c", "echo", "hello world");
        } else {
            pb = new ProcessBuilder("/bin/echo", "hello world");
        }

        Process process = pb.start();
        int exitCode = process.waitFor();

        System.out.println("exit code: " + exitCode);
        System.out.println("pid positive: " + (process.pid() > 0));
    }

    private static void testExitValue() throws Exception {
        System.out.println("=== Exit Value ===");
        String os = System.getProperty("os.name").toLowerCase();
        ProcessBuilder pb;
        if (os.contains("win")) {
            pb = new ProcessBuilder("cmd", "/c", "exit", "0");
        } else {
            pb = new ProcessBuilder("/usr/bin/true");
        }

        Process process = pb.start();
        int exitCode = process.waitFor();
        System.out.println("true exit code: " + exitCode);
        System.out.println("process not null: " + (process != null));
    }
}
