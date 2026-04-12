import java.nio.file.*;

/** Test NIO directory operations. */
public class Test {
    public static void main(String[] args) throws Exception {
        Path dir = Paths.get("nio_test_dir");

        // Clean up any leftover from previous runs
        if (Files.exists(dir)) {
            Files.delete(dir);
        }

        // Create and verify directory
        Files.createDirectory(dir);
        System.out.println("Dir created: " + Files.isDirectory(dir));

        // Create a file in the directory
        Path file = dir.resolve("test.txt");
        Files.write(file, "hello".getBytes());
        System.out.println("File exists: " + Files.exists(file));

        // Read it back
        String content = new String(Files.readAllBytes(file));
        System.out.println("Content: " + content);

        // Cleanup
        Files.delete(file);
        Files.delete(dir);
        System.out.println("Cleanup done: " + !Files.exists(dir));
    }
}
