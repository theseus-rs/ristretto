import java.nio.file.*;

/** Minimal NIO test to debug failures. */
public class Test {
    public static void main(String[] args) {
        try {
            Path p = Paths.get("simple_test.txt");
            System.out.println("Path: " + p);
            System.out.println("Absolute: " + p.toAbsolutePath());
            System.out.println("Exists before: " + Files.exists(p));

            Files.write(p, "hello".getBytes());
            System.out.println("Write succeeded");

            Files.deleteIfExists(p);
            System.out.println("Done");
        } catch (Exception e) {
            System.out.println("Error: " + e.getClass().getName() + ": " + e.getMessage());
            e.printStackTrace(System.out);
        }
    }
}
