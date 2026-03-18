/**
 * Test java.lang.ProcessHandle API methods.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        testCurrent();
        testPid();
        testIsAlive();
        testParent();
        testOf();
        testInfo();
        testRunCommand();
    }

    private static void testCurrent() {
        System.out.println("=== ProcessHandle.current() ===");
        ProcessHandle current = ProcessHandle.current();
        System.out.println("not null: " + (current != null));
    }

    private static void testPid() {
        System.out.println("=== ProcessHandle.pid() ===");
        long pid = ProcessHandle.current().pid();
        System.out.println("positive: " + (pid > 0));
    }

    private static void testIsAlive() {
        System.out.println("=== ProcessHandle.isAlive() ===");
        System.out.println("alive: " + ProcessHandle.current().isAlive());
    }

    private static void testParent() {
        System.out.println("=== ProcessHandle.parent() ===");
        System.out.println("present: " + ProcessHandle.current().parent().isPresent());
    }

    private static void testOf() {
        System.out.println("=== ProcessHandle.of() ===");
        long pid = ProcessHandle.current().pid();
        System.out.println("present: " + ProcessHandle.of(pid).isPresent());
    }

    private static void testInfo() {
        System.out.println("=== ProcessHandle.info() ===");
        ProcessHandle.Info info = ProcessHandle.current().info();
        System.out.println("not null: " + (info != null));
        System.out.println("command present: " + info.command().isPresent());
    }

    private static void testRunCommand() throws Exception {
        System.out.println("=== Run Command ===");
        String os = System.getProperty("os.name").toLowerCase();
        ProcessBuilder pb;
        if (os.contains("win")) {
            pb = new ProcessBuilder("cmd", "/c", "echo", "hello");
        } else {
            pb = new ProcessBuilder("/bin/echo", "hello");
        }

        Process process = pb.start();
        ProcessHandle handle = process.toHandle();
        System.out.println("pid positive: " + (handle.pid() > 0));

        int exitCode = process.waitFor();
        System.out.println("exit code: " + exitCode);
        System.out.println("alive after exit: " + handle.isAlive());
    }
}
