import java.io.*;
import java.nio.file.*;

/** Test NIO file copy and move operations. */
public class Test {
    public static void main(String[] args) throws IOException {
        Path src = Paths.get("nio_copy_src.txt");
        Path dst = Paths.get("nio_copy_dst.txt");
        Path moved = Paths.get("nio_copy_moved.txt");

        try {
            // Write source file
            Files.write(src, "copy test content".getBytes());
            System.out.println("Source exists: " + Files.exists(src));

            // Copy file
            Files.copy(src, dst);
            System.out.println("Dest exists: " + Files.exists(dst));
            System.out.println("Dest size: " + Files.size(dst));

            // Verify content matches
            byte[] srcBytes = Files.readAllBytes(src);
            byte[] dstBytes = Files.readAllBytes(dst);
            System.out.println("Content matches: " + java.util.Arrays.equals(srcBytes, dstBytes));

            // Copy with REPLACE_EXISTING
            Files.write(src, "updated content".getBytes());
            Files.copy(src, dst, StandardCopyOption.REPLACE_EXISTING);
            System.out.println("Updated size: " + Files.size(dst));

            // Move file
            Files.move(dst, moved);
            System.out.println("Moved exists: " + Files.exists(moved));
            System.out.println("Original gone: " + !Files.exists(dst));

            // Read moved file
            String content = new String(Files.readAllBytes(moved));
            System.out.println("Moved content: " + content);

        } finally {
            Files.deleteIfExists(src);
            Files.deleteIfExists(dst);
            Files.deleteIfExists(moved);
            System.out.println("Cleanup done");
        }
    }
}
