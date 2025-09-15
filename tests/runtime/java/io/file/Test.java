import java.io.*;
import java.nio.file.*;
import java.util.*;

/**
 * Tests for java.io.File class
 */
public class Test {
    private static final String TEST_DIR = "test_file_operations";
    private static final String TEST_FILE = "test.txt";
    private static final String TEST_SUBDIR = "subdir";

    public static void main(String[] args) {
        System.out.println("=== File Class Tests ===");

        try {
            // Setup test directory
            File testDir = new File(TEST_DIR);
            cleanup(testDir);

            testFileCreation();
            testFileProperties();
            testDirectoryOperations();
            testFilePermissions();
            testFileComparison();
            testFilePathOperations();
            testFileFiltering();
        } catch (Exception e) {
            System.out.println("ERROR: " + e.getMessage());
            e.printStackTrace();
        } finally {
            // Cleanup
            cleanup(new File(TEST_DIR));
        }

        System.out.println("=== File Tests Complete ===");
    }

    private static void testFileCreation() throws IOException {
        System.out.println("--- File Creation Tests ---");

        File testDir = new File(TEST_DIR);
        System.out.println("mkdir result: " + testDir.mkdir());
        System.out.println("testDir exists: " + testDir.exists());
        System.out.println("testDir isDirectory: " + testDir.isDirectory());

        File testFile = new File(testDir, TEST_FILE);
        System.out.println("createNewFile result: " + testFile.createNewFile());
        System.out.println("testFile exists: " + testFile.exists());
        System.out.println("testFile isFile: " + testFile.isFile());

        // Try creating again (should return false)
        System.out.println("createNewFile again: " + testFile.createNewFile());

        // Test with FileWriter
        try (FileWriter writer = new FileWriter(testFile)) {
            writer.write("Hello, World!");
        }
        System.out.println("File length after write: " + testFile.length());
    }

    private static void testFileProperties() throws IOException {
        System.out.println("--- File Properties Tests ---");

        File testFile = new File(TEST_DIR, TEST_FILE);

        System.out.println("getName: " + testFile.getName());
        System.out.println("getParent: " + testFile.getParent());
        System.out.println("getPath: " + testFile.getPath());
        System.out.println("getAbsolutePath: " + testFile.getAbsolutePath());
        System.out.println("getCanonicalPath: " + testFile.getCanonicalPath());

        System.out.println("canRead: " + testFile.canRead());
        System.out.println("canWrite: " + testFile.canWrite());
        System.out.println("canExecute: " + testFile.canExecute());

        System.out.println("isHidden: " + testFile.isHidden());
        System.out.println("lastModified: " + testFile.lastModified());

        // Test setters
        long newTime = System.currentTimeMillis() - 10000;
        System.out.println("setLastModified: " + testFile.setLastModified(newTime));
        System.out.println("lastModified after set: " + testFile.lastModified());

        System.out.println("setReadOnly: " + testFile.setReadOnly());
        System.out.println("canWrite after setReadOnly: " + testFile.canWrite());

        // Restore write permission for cleanup
        System.out.println("setWritable true: " + testFile.setWritable(true));
    }

    private static void testDirectoryOperations() throws IOException {
        System.out.println("--- Directory Operations Tests ---");

        File testDir = new File(TEST_DIR);
        File subDir = new File(testDir, TEST_SUBDIR);

        System.out.println("subDir mkdir: " + subDir.mkdir());
        System.out.println("subDir exists: " + subDir.exists());

        // Create multiple files
        for (int i = 0; i < 3; i++) {
            File file = new File(subDir, "file" + i + ".txt");
            try (FileWriter writer = new FileWriter(file)) {
                writer.write("Content " + i);
            }
        }

        // List directory contents
        String[] list = testDir.list();
        System.out.println("testDir list length: " + (list != null ? list.length : 0));
        if (list != null) {
            Arrays.sort(list);
            for (String name : list) {
                System.out.println("  " + name);
            }
        }

        File[] files = testDir.listFiles();
        System.out.println("testDir listFiles length: " + (files != null ? files.length : 0));
        if (files != null) {
            Arrays.sort(files, (a, b) -> a.getName().compareTo(b.getName()));
            for (File file : files) {
                System.out.println("  " + file.getName() + " (isDirectory: " + file.isDirectory() + ")");
            }
        }

        // Test mkdirs
        File deepDir = new File(testDir, "level1/level2/level3");
        System.out.println("deepDir mkdirs: " + deepDir.mkdirs());
        System.out.println("deepDir exists: " + deepDir.exists());
    }

    private static void testFilePermissions() {
        System.out.println("--- File Permissions Tests ---");

        File testFile = new File(TEST_DIR, TEST_FILE);

        System.out.println("Initial canRead: " + testFile.canRead());
        System.out.println("Initial canWrite: " + testFile.canWrite());
        System.out.println("Initial canExecute: " + testFile.canExecute());

        System.out.println("setReadable false: " + testFile.setReadable(false));
        System.out.println("canRead after false: " + testFile.canRead());

        System.out.println("setReadable true: " + testFile.setReadable(true));
        System.out.println("canRead after true: " + testFile.canRead());

        System.out.println("setExecutable true: " + testFile.setExecutable(true));
        System.out.println("canExecute after true: " + testFile.canExecute());

        System.out.println("setExecutable false: " + testFile.setExecutable(false));
        System.out.println("canExecute after false: " + testFile.canExecute());
    }

    private static void testFileComparison() {
        System.out.println("--- File Comparison Tests ---");

        File file1 = new File(TEST_DIR, TEST_FILE);
        File file2 = new File(TEST_DIR, TEST_FILE);
        File file3 = new File(TEST_DIR, "different.txt");

        System.out.println("file1.equals(file2): " + file1.equals(file2));
        System.out.println("file1.equals(file3): " + file1.equals(file3));
        System.out.println("file1.compareTo(file2): " + file1.compareTo(file2));
        System.out.println("file1.compareTo(file3): " + file1.compareTo(file3));
        System.out.println("file1.hashCode() == file2.hashCode(): " + (file1.hashCode() == file2.hashCode()));
    }

    private static void testFilePathOperations() {
        System.out.println("--- File Path Operations Tests ---");

        File file = new File(TEST_DIR, TEST_FILE);

        System.out.println("File.separator: '" + File.separator + "'");
        System.out.println("File.pathSeparator: '" + File.pathSeparator + "'");

        File parent = file.getParentFile();
        System.out.println("getParentFile: " + (parent != null ? parent.getName() : "null"));

        // Test with different path constructions
        File file1 = new File(TEST_DIR + File.separator + TEST_FILE);
        File file2 = new File(TEST_DIR, TEST_FILE);
        System.out.println("Different construction equals: " + file1.equals(file2));

        // Test relative vs absolute
        File relativeFile = new File(TEST_FILE);
        File absoluteFile = new File(new File(TEST_DIR).getAbsolutePath(), TEST_FILE);
        System.out.println("relative isAbsolute: " + relativeFile.isAbsolute());
        System.out.println("absolute isAbsolute: " + absoluteFile.isAbsolute());
    }

    private static void testFileFiltering() {
        System.out.println("--- File Filtering Tests ---");

        File testDir = new File(TEST_DIR);

        // Filter for .txt files
        String[] txtFiles = testDir.list(new FilenameFilter() {
            public boolean accept(File dir, String name) {
                return name.endsWith(".txt");
            }
        });

        System.out.println("txt files count: " + (txtFiles != null ? txtFiles.length : 0));
        if (txtFiles != null) {
            Arrays.sort(txtFiles);
            for (String name : txtFiles) {
                System.out.println("  " + name);
            }
        }

        // Filter for directories
        File[] directories = testDir.listFiles(new FileFilter() {
            public boolean accept(File pathname) {
                return pathname.isDirectory();
            }
        });

        System.out.println("directories count: " + (directories != null ? directories.length : 0));
        if (directories != null) {
            Arrays.sort(directories, (a, b) -> a.getName().compareTo(b.getName()));
            for (File dir : directories) {
                System.out.println("  " + dir.getName());
            }
        }
    }

    private static void cleanup(File file) {
        if (file.exists()) {
            if (file.isDirectory()) {
                File[] children = file.listFiles();
                if (children != null) {
                    for (File child : children) {
                        cleanup(child);
                    }
                }
            }
            file.setWritable(true);
            file.delete();
        }
    }
}
