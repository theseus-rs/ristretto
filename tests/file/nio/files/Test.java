import java.io.*;
import java.nio.file.*;
import java.nio.charset.StandardCharsets;
import java.util.List;

/** Test NIO Files read/write operations. */
public class Test {
    public static void main(String[] args) throws IOException {
        Path file = Paths.get("nio_test_file.txt");

        try {
            // Write string to file
            String content = "Hello NIO!\nSecond line\nThird line";
            Files.write(file, content.getBytes(StandardCharsets.UTF_8));
            System.out.println("File exists: " + Files.exists(file));
            System.out.println("Is regular file: " + Files.isRegularFile(file));

            // Read all bytes
            byte[] bytes = Files.readAllBytes(file);
            System.out.println("Read bytes: " + bytes.length);

            // Read all lines
            List<String> lines = Files.readAllLines(file, StandardCharsets.UTF_8);
            System.out.println("Line count: " + lines.size());
            for (String line : lines) {
                System.out.println("  " + line);
            }

            // Test file size
            long size = Files.size(file);
            System.out.println("File size: " + size);

            // Write and read via writeString (Java 11+)
            Path file2 = Paths.get("nio_test_file2.txt");
            try {
                Files.writeString(file2, "writeString test");
                String read = Files.readString(file2);
                System.out.println("readString: " + read);
            } finally {
                Files.deleteIfExists(file2);
            }

        } finally {
            Files.deleteIfExists(file);
            System.out.println("Cleanup done: " + !Files.exists(file));
        }
    }
}
