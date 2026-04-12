import java.io.*;
import java.nio.file.*;
import java.nio.file.attribute.*;

/** Test NIO file attribute operations. */
public class Test {
    public static void main(String[] args) throws IOException {
        Path file = Paths.get("nio_attr_test.txt");

        try {
            Files.write(file, "attribute test".getBytes());

            // Basic attributes
            BasicFileAttributes attrs = Files.readAttributes(file, BasicFileAttributes.class);
            System.out.println("Is regular: " + attrs.isRegularFile());
            System.out.println("Is directory: " + attrs.isDirectory());
            System.out.println("Is symlink: " + attrs.isSymbolicLink());
            System.out.println("Size: " + attrs.size());
            System.out.println("Creation time not null: " + (attrs.creationTime() != null));
            System.out.println("Modified time not null: " + (attrs.lastModifiedTime() != null));

            // Test isReadable, isWritable, isExecutable
            System.out.println("Is readable: " + Files.isReadable(file));
            System.out.println("Is writable: " + Files.isWritable(file));

            // Test hidden file
            System.out.println("Is hidden: " + Files.isHidden(file));

            // Test modify last modified time
            FileTime newTime = FileTime.fromMillis(1000000000L);
            Files.setLastModifiedTime(file, newTime);
            FileTime readBack = Files.getLastModifiedTime(file);
            System.out.println("Modified time set: " + (readBack.toMillis() == 1000000000L));

        } finally {
            Files.deleteIfExists(file);
        }

        System.out.println("Done");
    }
}
