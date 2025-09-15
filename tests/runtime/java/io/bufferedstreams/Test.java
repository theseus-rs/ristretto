import java.io.*;

/**
 * Tests for java.io.BufferedInputStream and BufferedOutputStream classes
 */
public class Test {
    private static final String TEST_DIR = "test_buffered_streams";
    private static final String TEST_FILE = "buffered_test.txt";
    private static final String BINARY_FILE = "buffered_binary.dat";

    public static void main(String[] args) {
        System.out.println("=== BufferedInputStream and BufferedOutputStream Tests ===");

        try {
            // Setup
            File testDir = new File(TEST_DIR);
            cleanup(testDir);
            testDir.mkdir();

            testBasicBufferedOperations();
            testBufferSizes();
            testMarkAndReset();
            testBufferedBinaryOperations();
            testBufferedExceptions();
            testMixedOperations();
        } catch (Exception e) {
            System.out.println("ERROR: " + e.getMessage());
            e.printStackTrace();
        } finally {
            cleanup(new File(TEST_DIR));
        }

        System.out.println("=== BufferedStream Tests Complete ===");
    }

    private static void testBasicBufferedOperations() throws IOException {
        System.out.println("--- Basic Buffered Operations Tests ---");

        File testFile = new File(TEST_DIR, TEST_FILE);
        String testData = "Hello, World!\nThis is line 2\nThis is line 3\n";

        // Write with BufferedOutputStream
        try (FileOutputStream fos = new FileOutputStream(testFile);
             BufferedOutputStream bos = new BufferedOutputStream(fos)) {

            byte[] data = testData.getBytes();
            bos.write(data);
            System.out.println("Written " + data.length + " bytes with BufferedOutputStream");

            // Test individual byte writing
            bos.write('\n');
            bos.write('E');
            bos.write('N');
            bos.write('D');

            // Explicit flush
            bos.flush();
            System.out.println("Flushed BufferedOutputStream");
        }

        System.out.println("File size after buffered write: " + testFile.length());

        // Read with BufferedInputStream
        try (FileInputStream fis = new FileInputStream(testFile);
             BufferedInputStream bis = new BufferedInputStream(fis)) {

            System.out.println("Available bytes: " + bis.available());

            // Read byte by byte
            int byteCount = 0;
            int b;
            while ((b = bis.read()) != -1 && byteCount < 10) {
                System.out.print((char) b);
                byteCount++;
            }
            System.out.println("\nRead first " + byteCount + " characters");

            // Read remaining into buffer
            byte[] buffer = new byte[1024];
            int bytesRead = bis.read(buffer);
            System.out.println("Read " + bytesRead + " bytes into buffer");

            String remaining = new String(buffer, 0, bytesRead);
            System.out.println("Remaining content: '" + remaining.replace("\n", "\\n") + "'");
        }
    }

    private static void testBufferSizes() throws IOException {
        System.out.println("--- Buffer Size Tests ---");

        File testFile = new File(TEST_DIR, "buffer_size_test.txt");
        String data = "0123456789";

        // Test different buffer sizes
        int[] bufferSizes = {1, 8, 16, 32, 8192};

        for (int bufferSize : bufferSizes) {
            System.out.println("Testing buffer size: " + bufferSize);

            // Write with specific buffer size
            try (FileOutputStream fos = new FileOutputStream(testFile);
                 BufferedOutputStream bos = new BufferedOutputStream(fos, bufferSize)) {

                // Write multiple times to test buffering
                for (int i = 0; i < 5; i++) {
                    bos.write(data.getBytes());
                }
                System.out.println("  Written with buffer size " + bufferSize);
            }

            // Read with specific buffer size
            try (FileInputStream fis = new FileInputStream(testFile);
                 BufferedInputStream bis = new BufferedInputStream(fis, bufferSize)) {

                int totalBytes = 0;
                int b;
                while ((b = bis.read()) != -1) {
                    totalBytes++;
                }
                System.out.println("  Read " + totalBytes + " bytes with buffer size " + bufferSize);
            }
        }
    }

    private static void testMarkAndReset() throws IOException {
        System.out.println("--- Mark and Reset Tests ---");

        File testFile = new File(TEST_DIR, "mark_reset_test.txt");

        // Create test data
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < 100; i++) {
            sb.append("Line ").append(i).append("\n");
        }

        try (FileOutputStream fos = new FileOutputStream(testFile);
             BufferedOutputStream bos = new BufferedOutputStream(fos)) {
            bos.write(sb.toString().getBytes());
        }

        // Test mark and reset
        try (FileInputStream fis = new FileInputStream(testFile);
             BufferedInputStream bis = new BufferedInputStream(fis)) {

            System.out.println("Mark supported: " + bis.markSupported());

            // Read some data
            byte[] initial = new byte[20];
            int initialRead = bis.read(initial);
            System.out.println("Initial read: " + initialRead + " bytes");
            System.out.println("Initial content: '" + new String(initial, 0, initialRead) + "'");

            // Mark current position
            bis.mark(100);
            System.out.println("Marked position after reading " + initialRead + " bytes");

            // Read more data
            byte[] middle = new byte[30];
            int middleRead = bis.read(middle);
            System.out.println("Middle read: " + middleRead + " bytes");
            System.out.println("Middle content: '" + new String(middle, 0, middleRead) + "'");

            // Reset to marked position
            bis.reset();
            System.out.println("Reset to marked position");

            // Read again from marked position
            byte[] afterReset = new byte[30];
            int afterResetRead = bis.read(afterReset);
            System.out.println("After reset read: " + afterResetRead + " bytes");
            System.out.println("After reset content: '" + new String(afterReset, 0, afterResetRead) + "'");

            // Compare middle and afterReset (should be the same)
            boolean same = middleRead == afterResetRead;
            if (same) {
                for (int i = 0; i < middleRead; i++) {
                    if (middle[i] != afterReset[i]) {
                        same = false;
                        break;
                    }
                }
            }
            System.out.println("Middle and after-reset content identical: " + same);
        }
    }

    private static void testBufferedBinaryOperations() throws IOException {
        System.out.println("--- Buffered Binary Operations Tests ---");

        File binaryFile = new File(TEST_DIR, BINARY_FILE);

        // Write binary data
        try (FileOutputStream fos = new FileOutputStream(binaryFile);
             BufferedOutputStream bos = new BufferedOutputStream(fos, 16)) {

            // Write various binary patterns
            for (int i = 0; i < 256; i++) {
                bos.write(i);
            }

            // Write byte arrays
            byte[] pattern1 = {(byte)0xDE, (byte)0xAD, (byte)0xBE, (byte)0xEF};
            byte[] pattern2 = {(byte)0xCA, (byte)0xFE, (byte)0xBA, (byte)0xBE};

            bos.write(pattern1);
            bos.write(pattern2, 1, 2); // Write FEBA

            System.out.println("Written binary data patterns");
        }

        System.out.println("Binary file size: " + binaryFile.length());

        // Read and verify binary data
        try (FileInputStream fis = new FileInputStream(binaryFile);
             BufferedInputStream bis = new BufferedInputStream(fis, 16)) {

            // Verify first 256 bytes
            boolean sequenceCorrect = true;
            for (int i = 0; i < 256; i++) {
                int b = bis.read();
                if (b != i) {
                    sequenceCorrect = false;
                    System.out.println("Sequence error at position " + i + ": expected " + i + ", got " + b);
                    break;
                }
            }
            System.out.println("Binary sequence 0-255 correct: " + sequenceCorrect);

            // Read patterns
            byte[] readPattern = new byte[6];
            int patternRead = bis.read(readPattern);
            System.out.println("Read " + patternRead + " pattern bytes");

            System.out.print("Pattern bytes: ");
            for (int i = 0; i < patternRead; i++) {
                System.out.printf("0x%02X ", readPattern[i] & 0xFF);
            }
            System.out.println();
        }
    }

    private static void testBufferedExceptions() throws IOException {
        System.out.println("--- Buffered Exceptions Tests ---");

        // Test invalid buffer size
        try {
            File testFile = new File(TEST_DIR, "exception_test.txt");
            try (FileOutputStream fos = new FileOutputStream(testFile)) {
                fos.write("test".getBytes());
            }

            try (FileInputStream fis = new FileInputStream(testFile)) {
                new BufferedInputStream(fis, 0);
                System.out.println("ERROR: Should have thrown exception for zero buffer size");
            } catch (IllegalArgumentException e) {
                System.out.println("Correctly caught IllegalArgumentException for zero buffer size");
            }

            try (FileInputStream fis = new FileInputStream(testFile)) {
                new BufferedInputStream(fis, -1);
                System.out.println("ERROR: Should have thrown exception for negative buffer size");
            } catch (IllegalArgumentException e) {
                System.out.println("Correctly caught IllegalArgumentException for negative buffer size");
            }

        } catch (Exception e) {
            System.out.println("Setup error: " + e.getMessage());
        }

        // Test mark with insufficient readlimit
        try {
            File testFile = new File(TEST_DIR, "mark_test.txt");
            try (FileOutputStream fos = new FileOutputStream(testFile)) {
                for (int i = 0; i < 1000; i++) {
                    fos.write(("Line " + i + "\n").getBytes());
                }
            }

            try (FileInputStream fis = new FileInputStream(testFile);
                 BufferedInputStream bis = new BufferedInputStream(fis, 100)) {

                bis.mark(10); // Small read limit

                // Read more than read limit
                byte[] buffer = new byte[50];
                bis.read(buffer);

                try {
                    bis.reset();
                    System.out.println("Reset successful after exceeding read limit");
                } catch (IOException e) {
                    System.out.println("Correctly caught IOException for reset after exceeding read limit");
                }
            }

        } catch (Exception e) {
            System.out.println("Mark test error: " + e.getMessage());
        }
    }

    private static void testMixedOperations() throws IOException {
        System.out.println("--- Mixed Operations Tests ---");

        File testFile = new File(TEST_DIR, "mixed_test.txt");

        // Write with mixed byte and array operations
        try (FileOutputStream fos = new FileOutputStream(testFile);
             BufferedOutputStream bos = new BufferedOutputStream(fos, 32)) {

            bos.write('H');
            bos.write('e');
            bos.write("llo".getBytes());
            bos.write(' ');
            bos.write("World!".getBytes());
            bos.write('\n');

            byte[] moreData = "More data here".getBytes();
            bos.write(moreData, 5, 4); // Write "data"

            System.out.println("Written mixed data");
        }

        // Read with mixed operations
        try (FileInputStream fis = new FileInputStream(testFile);
             BufferedInputStream bis = new BufferedInputStream(fis, 16)) {

            // Read first few bytes individually
            System.out.print("Individual bytes: ");
            for (int i = 0; i < 5; i++) {
                int b = bis.read();
                if (b != -1) {
                    System.out.print((char) b);
                }
            }
            System.out.println();

            // Skip some bytes
            long skipped = bis.skip(3);
            System.out.println("Skipped " + skipped + " bytes");

            // Read remaining with buffer
            byte[] buffer = new byte[20];
            int bytesRead = bis.read(buffer);
            System.out.println("Read " + bytesRead + " bytes into buffer");
            System.out.println("Buffer content: '" + new String(buffer, 0, bytesRead) + "'");

            System.out.println("Available at end: " + bis.available());
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
