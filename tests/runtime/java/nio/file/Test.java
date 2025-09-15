import java.nio.file.*;
import java.nio.file.attribute.*;
import java.nio.*;
import java.io.*;
import java.util.*;
import java.util.stream.Stream;

/**
 * Tests for java.nio.file
 */
public class Test {
    private static final String TEST_DIR = "test_nio_files";
    private static final String TEST_FILE = "test_file.txt";
    private static final String TEST_CONTENT = "Hello, NIO File API!";

    public static void main(String[] args) {
        System.out.println("=== Java NIO File Tests ===");

        try {
            setupTestEnvironment();

            testPath();
            testFiles();
            testFileAttributes();
            testFileOperations();
            testDirectoryOperations();
            testFileVisitor();
            testWatchService();
            testFileSystem();

        } catch (Exception e) {
            System.out.println("ERROR in test setup: " + e.getMessage());
            e.printStackTrace();
        } finally {
            cleanup();
        }

        System.out.println("=== File Tests Complete ===");
    }

    private static void setupTestEnvironment() throws IOException {
        Path testDir = Paths.get(TEST_DIR);
        if (!Files.exists(testDir)) {
            Files.createDirectory(testDir);
        }
        System.out.println("Test directory created: " + testDir);
    }

    private static void testPath() {
        System.out.println("--- Path Tests ---");

        // Test path creation
        Path path = Paths.get(TEST_DIR, TEST_FILE);
        System.out.println("Created path: " + path);
        System.out.println("Path contains test dir: " + path.toString().contains(TEST_DIR));
        System.out.println("Path contains test file: " + path.toString().contains(TEST_FILE));

        // Test path components
        System.out.println("Path filename: " + path.getFileName());
        System.out.println("Path parent: " + path.getParent());
        if (path.getParent() != null) {
            System.out.println("Parent filename: " + path.getParent().getFileName());
        }

        // Test absolute path
        Path absolutePath = path.toAbsolutePath();
        System.out.println("Is absolute: " + absolutePath.isAbsolute());
        System.out.println("Absolute path length > relative: " + (absolutePath.toString().length() > path.toString().length()));

        // Test path operations
        System.out.println("Path name count: " + path.getNameCount());
        if (path.getNameCount() > 0) {
            System.out.println("Last name: " + path.getName(path.getNameCount() - 1));
        }

        // Test path resolution
        Path subPath = Paths.get(TEST_DIR);
        Path resolved = subPath.resolve(TEST_FILE);
        System.out.println("Original path: " + path);
        System.out.println("Resolved path: " + resolved);
        System.out.println("Resolution matches: " + resolved.toString().equals(path.toString()));

        // Test relative path
        Path relative = absolutePath.relativize(absolutePath.getParent().resolve("other.txt"));
        System.out.println("Relative path is absolute: " + relative.isAbsolute());

        // Test normalize
        Path unnormalized = Paths.get(TEST_DIR, "..", TEST_DIR, TEST_FILE);
        Path normalized = unnormalized.normalize();
        System.out.println("Unnormalized: " + unnormalized);
        System.out.println("Normalized: " + normalized);
        System.out.println("Normalization works: " + normalized.toString().equals(path.toString()));

        // Test startsWith/endsWith
        System.out.println("Path starts with test dir: " + path.startsWith(TEST_DIR));
        System.out.println("Path ends with test file: " + path.endsWith(TEST_FILE));

        // Test path comparison
        Path samePath = Paths.get(TEST_DIR, TEST_FILE);
        System.out.println("Paths equal: " + path.equals(samePath));
        System.out.println("Paths compare: " + path.compareTo(samePath));
    }

    private static void testFiles() throws IOException {
        System.out.println("--- Files Tests ---");

        Path testFile = Paths.get(TEST_DIR, TEST_FILE);

        // Test file creation and existence
        if (!Files.exists(testFile)) {
            Files.createFile(testFile);
        }
        System.out.println("File exists: " + Files.exists(testFile));
        System.out.println("Is regular file: " + Files.isRegularFile(testFile));
        System.out.println("Is directory: " + Files.isDirectory(testFile));

        // Test writing and reading
        Files.write(testFile, TEST_CONTENT.getBytes());
        System.out.println("File size after write: " + Files.size(testFile));

        byte[] readBytes = Files.readAllBytes(testFile);
        String readContent = new String(readBytes);
        System.out.println("Written content: '" + TEST_CONTENT + "'");
        System.out.println("Read content: '" + readContent + "'");
        System.out.println("Content matches: " + TEST_CONTENT.equals(readContent));

        // Test reading as lines
        List<String> lines = Files.readAllLines(testFile);
        System.out.println("Lines read: " + lines.size());
        if (!lines.isEmpty()) {
            System.out.println("First line: '" + lines.get(0) + "'");
            System.out.println("First line matches: " + TEST_CONTENT.equals(lines.get(0)));
        }

        // Test writing lines
        List<String> writeLines = Arrays.asList("Line 1", "Line 2", "Line 3");
        Files.write(testFile, writeLines);
        List<String> readLines = Files.readAllLines(testFile);
        System.out.println("Written lines: " + writeLines.size());
        System.out.println("Read lines: " + readLines.size());
        System.out.println("Lines match: " + writeLines.equals(readLines));

        // Test file copy
        Path copyFile = Paths.get(TEST_DIR, "copy_" + TEST_FILE);
        Files.copy(testFile, copyFile);
        System.out.println("Copy exists: " + Files.exists(copyFile));
        System.out.println("Copy size: " + Files.size(copyFile));
        System.out.println("Original size: " + Files.size(testFile));
        System.out.println("Sizes match: " + (Files.size(copyFile) == Files.size(testFile)));

        // Test file move
        Path moveFile = Paths.get(TEST_DIR, "moved_" + TEST_FILE);
        Files.move(copyFile, moveFile);
        System.out.println("Moved file exists: " + Files.exists(moveFile));
        System.out.println("Original copy exists: " + Files.exists(copyFile));

        // Test file deletion
        Files.delete(moveFile);
        System.out.println("File deleted: " + !Files.exists(moveFile));

        // Test deleteIfExists
        boolean deleted = Files.deleteIfExists(moveFile);
        System.out.println("Delete non-existent returned: " + deleted);
    }

    private static void testFileAttributes() throws IOException {
        System.out.println("--- File Attributes Tests ---");

        Path testFile = Paths.get(TEST_DIR, TEST_FILE);

        // Test basic attributes
        BasicFileAttributes attrs = Files.readAttributes(testFile, BasicFileAttributes.class);
        System.out.println("Is regular file: " + attrs.isRegularFile());
        System.out.println("Is directory: " + attrs.isDirectory());
        System.out.println("Is symbolic link: " + attrs.isSymbolicLink());
        System.out.println("File size: " + attrs.size());
        System.out.println("Creation time: " + attrs.creationTime());
        System.out.println("Last modified: " + attrs.lastModifiedTime());
        System.out.println("Last access: " + attrs.lastAccessTime());

        // Test file time
        FileTime originalTime = attrs.lastModifiedTime();
        FileTime newTime = FileTime.fromMillis(System.currentTimeMillis() + 10000);
        Files.setLastModifiedTime(testFile, newTime);

        FileTime updatedTime = Files.getLastModifiedTime(testFile);
        System.out.println("Original time: " + originalTime);
        System.out.println("Updated time: " + updatedTime);
        System.out.println("Time changed: " + !originalTime.equals(updatedTime));

        // Test file size
        long size = Files.size(testFile);
        System.out.println("File size: " + size);

        // Test readable/writable/executable
        System.out.println("File is readable: " + Files.isReadable(testFile));
        System.out.println("File is writable: " + Files.isWritable(testFile));

        // Test hidden file (platform dependent)
        boolean isHidden = Files.isHidden(testFile);
        System.out.println("File is hidden: " + isHidden);

        // Test same file
        Path samePath = Paths.get(TEST_DIR).resolve(TEST_FILE);
        System.out.println("Same file check: " + Files.isSameFile(testFile, samePath));
    }

    private static void testFileOperations() throws IOException {
        System.out.println("--- File Operations Tests ---");

        Path sourceFile = Paths.get(TEST_DIR, "source.txt");
        Path targetFile = Paths.get(TEST_DIR, "target.txt");

        // Create source file
        String sourceContent = "Source file content for operations test";
        Files.write(sourceFile, sourceContent.getBytes());
        System.out.println("Source file created with content length: " + sourceContent.length());

        // Test copy with options
        Files.copy(sourceFile, targetFile, StandardCopyOption.REPLACE_EXISTING);
        System.out.println("Target file exists after copy: " + Files.exists(targetFile));

        String targetContent = new String(Files.readAllBytes(targetFile));
        System.out.println("Copy preserves content: " + sourceContent.equals(targetContent));

        // Test copy attributes
        Files.copy(sourceFile, targetFile,
                   StandardCopyOption.REPLACE_EXISTING,
                   StandardCopyOption.COPY_ATTRIBUTES);
        System.out.println("Copy with attributes completed");

        // Test move with options
        Path movedFile = Paths.get(TEST_DIR, "moved.txt");
        Files.move(targetFile, movedFile, StandardCopyOption.REPLACE_EXISTING);
        System.out.println("Move completed, moved file exists: " + Files.exists(movedFile));
        System.out.println("Original target exists: " + Files.exists(targetFile));

        // Test newInputStream/newOutputStream
        try (OutputStream os = Files.newOutputStream(sourceFile, StandardOpenOption.APPEND)) {
            os.write(" - appended".getBytes());
        }

        try (InputStream is = Files.newInputStream(sourceFile)) {
            byte[] buffer = new byte[1024];
            int bytesRead = is.read(buffer);
            String content = new String(buffer, 0, bytesRead);
            System.out.println("Content after append contains 'appended': " + content.contains("appended"));
        }

        // Test newBufferedReader/newBufferedWriter
        try (BufferedWriter writer = Files.newBufferedWriter(movedFile)) {
            writer.write("Buffered writer test");
            writer.newLine();
            writer.write("Second line");
        }

        try (BufferedReader reader = Files.newBufferedReader(movedFile)) {
            String line1 = reader.readLine();
            String line2 = reader.readLine();
            System.out.println("Buffered line 1: '" + line1 + "'");
            System.out.println("Buffered line 2: '" + line2 + "'");
        }

        // Cleanup
        Files.deleteIfExists(sourceFile);
        Files.deleteIfExists(movedFile);
    }

    private static void testDirectoryOperations() throws IOException {
        System.out.println("--- Directory Operations Tests ---");

        Path testDir = Paths.get(TEST_DIR, "subdir");
        Path nestedDir = testDir.resolve("nested");

        // Test directory creation
        Files.createDirectory(testDir);
        System.out.println("Directory created: " + Files.exists(testDir));
        System.out.println("Created path is directory: " + Files.isDirectory(testDir));

        // Test createDirectories (recursive)
        Files.createDirectories(nestedDir);
        System.out.println("Nested directory created: " + Files.exists(nestedDir));
        System.out.println("Nested path is directory: " + Files.isDirectory(nestedDir));

        // Create some files in directory
        Path file1 = testDir.resolve("file1.txt");
        Path file2 = testDir.resolve("file2.txt");
        Files.write(file1, "File 1 content".getBytes());
        Files.write(file2, "File 2 content".getBytes());

        // Test directory listing
        try (Stream<Path> stream = Files.list(testDir)) {
            List<Path> entries = stream.sorted().collect(java.util.stream.Collectors.toList());
            System.out.println("Directory entries count: " + entries.size());

            boolean hasFile1 = entries.stream().anyMatch(p -> "file1.txt".equals(p.getFileName().toString()));
            boolean hasFile2 = entries.stream().anyMatch(p -> "file2.txt".equals(p.getFileName().toString()));
            boolean hasNested = entries.stream().anyMatch(p -> "nested".equals(p.getFileName().toString()));

            System.out.println("Contains file1.txt: " + hasFile1);
            System.out.println("Contains file2.txt: " + hasFile2);
            System.out.println("Contains nested dir: " + hasNested);
        }

        // Test directory walk
        try (Stream<Path> walkStream = Files.walk(testDir)) {
            long count = walkStream.count();
            System.out.println("Walk found entries: " + count);
        }

        // Test directory walk with depth
        try (Stream<Path> walkStream = Files.walk(testDir, 1)) {
            long count = walkStream.count();
            System.out.println("Walk with depth 1 found: " + count);
        }

        // Test find
        try (Stream<Path> findStream = Files.find(testDir, 2,
                (path, attrs) -> attrs.isRegularFile() && path.toString().endsWith(".txt"))) {
            long txtCount = findStream.count();
            System.out.println("Found .txt files: " + txtCount);
        }

        // Test directory is not empty
        System.out.println("Directory is empty: " + isEmpty(testDir));

        // Cleanup files first
        Files.delete(file1);
        Files.delete(file2);
        Files.delete(nestedDir);
        Files.delete(testDir);
        System.out.println("Directory cleanup completed");
    }

    private static void testFileVisitor() throws IOException {
        System.out.println("--- FileVisitor Tests ---");

        Path visitDir = Paths.get(TEST_DIR, "visit_test");
        Files.createDirectories(visitDir);

        // Create test structure
        Path subDir1 = visitDir.resolve("sub1");
        Path subDir2 = visitDir.resolve("sub2");
        Files.createDirectory(subDir1);
        Files.createDirectory(subDir2);

        Files.write(visitDir.resolve("root.txt"), "root file".getBytes());
        Files.write(subDir1.resolve("sub1.txt"), "sub1 file".getBytes());
        Files.write(subDir2.resolve("sub2.txt"), "sub2 file".getBytes());

        // Custom file visitor
        final int[] dirCount = {0};
        final int[] fileCount = {0};

        Files.walkFileTree(visitDir, new SimpleFileVisitor<Path>() {
            @Override
            public FileVisitResult visitFile(Path file, BasicFileAttributes attrs) {
                fileCount[0]++;
                return FileVisitResult.CONTINUE;
            }

            @Override
            public FileVisitResult preVisitDirectory(Path dir, BasicFileAttributes attrs) {
                dirCount[0]++;
                return FileVisitResult.CONTINUE;
            }
        });

        System.out.println("Directories visited: " + dirCount[0]);
        System.out.println("Files visited: " + fileCount[0]);

        // Test visitor with termination
        final boolean[] foundTarget = {false};

        Files.walkFileTree(visitDir, new SimpleFileVisitor<Path>() {
            @Override
            public FileVisitResult visitFile(Path file, BasicFileAttributes attrs) {
                if (file.getFileName().toString().equals("sub1.txt")) {
                    foundTarget[0] = true;
                    return FileVisitResult.TERMINATE;
                }
                return FileVisitResult.CONTINUE;
            }
        });

        System.out.println("Found target file: " + foundTarget[0]);

        // Cleanup
        deleteRecursively(visitDir);
    }

    private static void testWatchService() {
        System.out.println("--- WatchService Tests ---");

        try {
            Path watchDir = Paths.get(TEST_DIR, "watch_test");
            Files.createDirectories(watchDir);

            WatchService watchService = FileSystems.getDefault().newWatchService();
            System.out.println("WatchService created");

            WatchKey key = watchDir.register(watchService,
                StandardWatchEventKinds.ENTRY_CREATE,
                StandardWatchEventKinds.ENTRY_MODIFY,
                StandardWatchEventKinds.ENTRY_DELETE);
            System.out.println("Directory registered for watching");
            System.out.println("WatchKey is valid: " + key.isValid());

            // Create a file to trigger event
            Path testFile = watchDir.resolve("watched_file.txt");
            Files.write(testFile, "watch test".getBytes());

            // Poll for events (with timeout to avoid hanging)
            WatchKey polledKey = watchService.poll(java.util.concurrent.TimeUnit.SECONDS.toNanos(2),
                                                   java.util.concurrent.TimeUnit.NANOSECONDS);

            if (polledKey != null) {
                System.out.println("Watch event detected");

                List<WatchEvent<?>> events = polledKey.pollEvents();
                System.out.println("Events detected: " + events.size());

                boolean createEventFound = events.stream()
                    .anyMatch(event -> event.kind() == StandardWatchEventKinds.ENTRY_CREATE);
                System.out.println("Create event found: " + createEventFound);

                polledKey.reset();
            } else {
                System.out.println("Watch service timeout (may be platform dependent)");
            }

            key.cancel();
            watchService.close();

            deleteRecursively(watchDir);

        } catch (Exception e) {
            System.out.println("WatchService error (may be platform dependent): " + e.getMessage());
        }
    }

    private static void testFileSystem() throws IOException {
        System.out.println("--- FileSystem Tests ---");

        FileSystem defaultFs = FileSystems.getDefault();
        System.out.println("Default filesystem is open: " + defaultFs.isOpen());
        System.out.println("Default filesystem is read-only: " + defaultFs.isReadOnly());

        // Test path separator
        String separator = defaultFs.getSeparator();
        System.out.println("Path separator: '" + separator + "'");
        System.out.println("Separator length: " + separator.length());

        // Test root directories
        Iterable<Path> roots = defaultFs.getRootDirectories();
        boolean hasRoots = roots.iterator().hasNext();
        System.out.println("Has root directories: " + hasRoots);

        // Test file stores
        Iterable<FileStore> stores = defaultFs.getFileStores();
        System.out.println("File stores available: " + (stores != null));

        // Test supported file attribute views
        Set<String> views = defaultFs.supportedFileAttributeViews();
        System.out.println("Supported views count: " + views.size());
        System.out.println("Basic view supported: " + views.contains("basic"));

        // Test path matcher
        PathMatcher txtMatcher = defaultFs.getPathMatcher("glob:*.txt");
        System.out.println("PathMatcher created for *.txt");
        System.out.println("Matches test.txt: " + txtMatcher.matches(Paths.get("test.txt")));
        System.out.println("Matches test.java: " + txtMatcher.matches(Paths.get("test.java")));

        PathMatcher regexMatcher = defaultFs.getPathMatcher("regex:.*\\.java");
        System.out.println("Regex PathMatcher created");
        System.out.println("Regex matches Test.java: " + regexMatcher.matches(Paths.get("Test.java")));
        System.out.println("Regex matches test.txt: " + regexMatcher.matches(Paths.get("test.txt")));
    }

    private static boolean isEmpty(Path directory) throws IOException {
        try (Stream<Path> stream = Files.list(directory)) {
            return !stream.findAny().isPresent();
        }
    }

    private static void deleteRecursively(Path path) throws IOException {
        if (Files.isDirectory(path)) {
            try (Stream<Path> stream = Files.list(path)) {
                stream.forEach(child -> {
                    try {
                        deleteRecursively(child);
                    } catch (IOException e) {
                        System.err.println("Error deleting " + child + ": " + e.getMessage());
                    }
                });
            }
        }
        Files.deleteIfExists(path);
    }

    private static void cleanup() {
        try {
            Path testDir = Paths.get(TEST_DIR);
            if (Files.exists(testDir)) {
                deleteRecursively(testDir);
                System.out.println("Cleanup completed");
            }
        } catch (Exception e) {
            System.err.println("Cleanup error: " + e.getMessage());
        }
    }
}
