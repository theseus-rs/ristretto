import java.nio.file.*;

/** Test basic NIO Path operations. */
public class Test {
    public static void main(String[] args) {
        // Test Path creation and basic operations
        Path p = Paths.get("test_file.txt");
        System.out.println("Filename: " + p.getFileName());
        System.out.println("Is absolute: " + p.isAbsolute());

        // Test path resolution
        Path base = Paths.get("/tmp");
        Path resolved = base.resolve("subdir").resolve("file.txt");
        System.out.println("Resolved: " + resolved);

        // Test relativize
        Path p1 = Paths.get("/a/b/c");
        Path p2 = Paths.get("/a/d");
        System.out.println("Relative: " + p1.relativize(p2));

        // Test normalize
        Path messy = Paths.get("/a/b/../c/./d");
        System.out.println("Normalized: " + messy.normalize());

        // Test getParent and getRoot
        Path full = Paths.get("/usr/local/bin/java");
        System.out.println("Parent: " + full.getParent());
        System.out.println("Root: " + full.getRoot());
        System.out.println("Name count: " + full.getNameCount());

        // Test toAbsolutePath
        Path rel = Paths.get("relative");
        Path abs = rel.toAbsolutePath();
        System.out.println("Absolute starts with /: " + abs.toString().startsWith("/"));

        System.out.println("Done");
    }
}
