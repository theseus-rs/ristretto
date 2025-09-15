import java.io.*;
import java.util.Arrays;

/**
 * Tests for java.io.FileInputStream and FileOutputStream classes
 */
public class Test {
    private static final String TEST_DIR = "test_streams";
    private static final String INPUT_FILE = "input.txt";
    private static final String OUTPUT_FILE = "output.txt";
    private static final String BINARY_FILE = "binary.dat";

    public static void main(String[] args) {
        System.out.println("=== FileInputStream and FileOutputStream Tests ===");

        try {
            // Setup
            File testDir = new File(TEST_DIR);
            cleanup(testDir);
            testDir.mkdir();

            testBasicFileOperations();
            testBinaryOperations();
            testBufferedOperations();
            testAppendMode();
            testFileStreamProperties();
            testStreamExceptions();
            testLargeFileOperations();
        } catch (Exception e) {
            System.out.println("ERROR: " + e.getMessage());
            e.printStackTrace();
        } finally {
            cleanup(new File(TEST_DIR));
        }

        System.out.println("=== FileStream Tests Complete ===");
    }

    private static void testBasicFileOperations() throws IOException {
        System.out.println("--- Basic File Operations Tests ---");

        File inputFile = new File(TEST_DIR, INPUT_FILE);
        File outputFile = new File(TEST_DIR, OUTPUT_FILE);

        // Create test data
        String testData = "Hello, World!\nThis is a test file.\nLine 3\n";

        // Write with FileOutputStream
        try (FileOutputStream fos = new FileOutputStream(inputFile)) {
            byte[] data = testData.getBytes();
            fos.write(data);
            System.out.println("Wrote " + data.length + " bytes to input file");
        }

        // Read with FileInputStream
        try (FileInputStream fis = new FileInputStream(inputFile)) {
            System.out.println("Available bytes: " + fis.available());

            // Read byte by byte
            int byteCount = 0;
            int b;
            while ((b = fis.read()) != -1) {
                byteCount++;
            }
            System.out.println("Read " + byteCount + " bytes");
        }

        // Copy file using streams
        try (FileInputStream fis = new FileInputStream(inputFile);
             FileOutputStream fos = new FileOutputStream(outputFile)) {

            byte[] buffer = new byte[1024];
            int bytesRead;
            int totalBytes = 0;

            while ((bytesRead = fis.read(buffer)) != -1) {
                fos.write(buffer, 0, bytesRead);
                totalBytes += bytesRead;
            }
            System.out.println("Copied " + totalBytes + " bytes");
        }

        // Verify copy
        System.out.println("Original file size: " + inputFile.length());
        System.out.println("Copied file size: " + outputFile.length());
        System.out.println("Files equal: " + filesEqual(inputFile, outputFile));
    }

    private static void testBinaryOperations() throws IOException {
        System.out.println("--- Binary Operations Tests ---");

        File binaryFile = new File(TEST_DIR, BINARY_FILE);

        // Write binary data
        try (FileOutputStream fos = new FileOutputStream(binaryFile)) {
            // Write various data types as bytes
            fos.write(0xFF);
            fos.write(0x00);
            fos.write(0x7F);

            // Write byte array
            byte[] data = {0x01, 0x02, 0x03, 0x04, 0x05};
            fos.write(data);

            // Write partial array
            byte[] moreData = {0x10, 0x20, 0x30, 0x40, 0x50};
            fos.write(moreData, 1, 3); // Write bytes 0x20, 0x30, 0x40

            System.out.println("Written binary data");
        }

        // Read binary data
        try (FileInputStream fis = new FileInputStream(binaryFile)) {
            System.out.println("Binary file size: " + binaryFile.length());

            // Read individual bytes
            System.out.print("Bytes: ");
            int b;
            while ((b = fis.read()) != -1) {
                System.out.printf("0x%02X ", b);
            }
            System.out.println();
        }

        // Read with buffer
        try (FileInputStream fis = new FileInputStream(binaryFile)) {
            byte[] buffer = new byte[20];
            int bytesRead = fis.read(buffer);
            System.out.println("Read " + bytesRead + " bytes into buffer");
            System.out.print("Buffer contents: ");
            for (int i = 0; i < bytesRead; i++) {
                System.out.printf("0x%02X ", buffer[i]);
            }
            System.out.println();
        }
    }

    private static void testBufferedOperations() throws IOException {
        System.out.println("--- Buffered Operations Tests ---");

        File testFile = new File(TEST_DIR, "buffered_test.txt");
        String data = "This is test data for buffered operations.\n";

        // Write multiple times
        try (FileOutputStream fos = new FileOutputStream(testFile)) {
            for (int i = 0; i < 5; i++) {
                String line = i + ": " + data;
                fos.write(line.getBytes());
            }
            System.out.println("Written multiple lines");
        }

        // Read in chunks
        try (FileInputStream fis = new FileInputStream(testFile)) {
            byte[] buffer = new byte[10];
            int bytesRead;
            int chunkCount = 0;

            while ((bytesRead = fis.read(buffer)) != -1) {
                chunkCount++;
                System.out.println("Chunk " + chunkCount + ": read " + bytesRead + " bytes");
                // Show first few characters
                String chunk = new String(buffer, 0, Math.min(bytesRead, 5));
                System.out.println("  Content starts with: '" + chunk + "'");
            }
        }
    }

    private static void testAppendMode() throws IOException {
        System.out.println("--- Append Mode Tests ---");

        File appendFile = new File(TEST_DIR, "append_test.txt");

        // Write initial content
        try (FileOutputStream fos = new FileOutputStream(appendFile)) {
            fos.write("Initial content\n".getBytes());
        }
        System.out.println("Initial file size: " + appendFile.length());

        // Append content
        try (FileOutputStream fos = new FileOutputStream(appendFile, true)) {
            fos.write("Appended content\n".getBytes());
        }
        System.out.println("File size after append: " + appendFile.length());

        // Verify content
        try (FileInputStream fis = new FileInputStream(appendFile)) {
            byte[] buffer = new byte[(int) appendFile.length()];
            int bytesRead = fis.read(buffer);
            String content = new String(buffer, 0, bytesRead);
            System.out.println("Final content:");
            String[] lines = content.split("\n");
            for (int i = 0; i < lines.length; i++) {
                System.out.println("  Line " + (i+1) + ": " + lines[i]);
            }
        }
    }

    private static void testFileStreamProperties() throws IOException {
        System.out.println("--- FileStream Properties Tests ---");

        File testFile = new File(TEST_DIR, "properties_test.txt");

        // Create test file
        try (FileOutputStream fos = new FileOutputStream(testFile)) {
            fos.write("Test data for properties".getBytes());
        }

        try (FileInputStream fis = new FileInputStream(testFile)) {
            System.out.println("Available at start: " + fis.available());

            // Read some bytes and check available
            fis.read();
            fis.read();
            System.out.println("Available after reading 2 bytes: " + fis.available());

            // Skip bytes
            long skipped = fis.skip(5);
            System.out.println("Skipped " + skipped + " bytes");
            System.out.println("Available after skip: " + fis.available());

            // Test mark support
            System.out.println("Mark supported: " + fis.markSupported());

            // Read remaining
            int remaining = 0;
            while (fis.read() != -1) {
                remaining++;
            }
            System.out.println("Remaining bytes read: " + remaining);
            System.out.println("Available at end: " + fis.available());
        }
    }

    private static void testStreamExceptions() {
        System.out.println("--- Stream Exceptions Tests ---");

        // Test FileNotFoundException
        try {
            new FileInputStream("nonexistent_file.txt");
            System.out.println("ERROR: Should have thrown FileNotFoundException");
        } catch (FileNotFoundException e) {
            System.out.println("Correctly caught FileNotFoundException: " + e.getMessage());
        }

        // Test writing to directory
        try {
            File dir = new File(TEST_DIR);
            new FileOutputStream(dir);
            System.out.println("ERROR: Should have thrown exception for directory");
        } catch (IOException e) {
            System.out.println("Correctly caught IOException for directory: " + e.getClass().getSimpleName());
        }

        // Test operations on closed stream
        File testFile = new File(TEST_DIR, "exception_test.txt");
        try {
            // Create file
            try (FileOutputStream fos = new FileOutputStream(testFile)) {
                fos.write("test".getBytes());
            }

            FileInputStream fis = new FileInputStream(testFile);
            fis.close();

            // Try to read from closed stream
            try {
                fis.read();
                System.out.println("ERROR: Should have thrown exception for closed stream");
            } catch (IOException e) {
                System.out.println("Correctly caught IOException for closed stream: " + e.getClass().getSimpleName());
            }

        } catch (IOException e) {
            System.out.println("Setup error: " + e.getMessage());
        }
    }

    private static void testLargeFileOperations() throws IOException {
        System.out.println("--- Large File Operations Tests ---");

        File largeFile = new File(TEST_DIR, "large_test.txt");

        // Write larger amount of data
        int dataSize = 10000;
        try (FileOutputStream fos = new FileOutputStream(largeFile)) {
            for (int i = 0; i < dataSize; i++) {
                String line = String.format("Line %04d: This is test data\n", i);
                fos.write(line.getBytes());
            }
        }

        System.out.println("Large file size: " + largeFile.length());

        // Read and count lines
        try (FileInputStream fis = new FileInputStream(largeFile)) {
            int lineCount = 0;
            int b;
            while ((b = fis.read()) != -1) {
                if (b == '\n') {
                    lineCount++;
                }
            }
            System.out.println("Lines counted: " + lineCount);
        }

        // Test skip on large file
        try (FileInputStream fis = new FileInputStream(largeFile)) {
            long skipAmount = largeFile.length() / 2;
            long actualSkipped = fis.skip(skipAmount);
            System.out.println("Requested skip: " + skipAmount + ", actual: " + actualSkipped);
            System.out.println("Available after skip: " + fis.available());
        }
    }

    private static boolean filesEqual(File file1, File file2) throws IOException {
        if (file1.length() != file2.length()) {
            return false;
        }

        try (FileInputStream fis1 = new FileInputStream(file1);
             FileInputStream fis2 = new FileInputStream(file2)) {

            int b1, b2;
            while ((b1 = fis1.read()) != -1) {
                b2 = fis2.read();
                if (b1 != b2) {
                    return false;
                }
            }
            return fis2.read() == -1;
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
            file.delete();
        }
    }
}
