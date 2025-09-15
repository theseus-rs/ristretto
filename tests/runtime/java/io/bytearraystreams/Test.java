import java.io.*;

/**
 * Tests for java.io.ByteArrayInputStream and ByteArrayOutputStream classes
 */
public class Test {
    private static final String TEST_DIR = "test_byte_array_streams";

    public static void main(String[] args) {
        System.out.println("=== ByteArrayInputStream and ByteArrayOutputStream Tests ===");

        try {
            // Setup
            File testDir = new File(TEST_DIR);
            cleanup(testDir);
            testDir.mkdir();

            testBasicByteArrayOperations();
            testByteArrayStreamProperties();
            testBinaryDataOperations();
            testStreamCopying();
            testMarkAndReset();
            testExceptionHandling();
            testLargeDataOperations();
            testMemoryEfficiency();
        } catch (Exception e) {
            System.out.println("ERROR: " + e.getMessage());
            e.printStackTrace();
        } finally {
            cleanup(new File(TEST_DIR));
        }

        System.out.println("=== ByteArrayStream Tests Complete ===");
    }

    private static void testBasicByteArrayOperations() throws IOException {
        System.out.println("--- Basic ByteArray Operations Tests ---");

        // Test ByteArrayOutputStream
        ByteArrayOutputStream baos = new ByteArrayOutputStream();

        // Write various data
        baos.write(65); // 'A'
        baos.write(66); // 'B'
        baos.write(67); // 'C'

        byte[] data = "Hello World".getBytes();
        baos.write(data);

        byte[] moreData = "!!! Extra !!!".getBytes();
        baos.write(moreData, 4, 5); // Write "Extra"

        System.out.println("ByteArrayOutputStream size: " + baos.size());

        // Get the byte array
        byte[] result = baos.toByteArray();
        System.out.println("Result array length: " + result.length);
        System.out.println("Result as string: '" + new String(result) + "'");

        // Test ByteArrayInputStream
        ByteArrayInputStream bais = new ByteArrayInputStream(result);

        System.out.println("Available bytes: " + bais.available());

        // Read byte by byte
        System.out.print("Reading byte by byte: ");
        for (int i = 0; i < 10; i++) {
            int b = bais.read();
            if (b != -1) {
                System.out.print((char) b);
            }
        }
        System.out.println();

        System.out.println("Available after reading 10 bytes: " + bais.available());

        // Read remaining into buffer
        byte[] buffer = new byte[50];
        int bytesRead = bais.read(buffer);
        System.out.println("Read " + bytesRead + " bytes into buffer");
        System.out.println("Buffer content: '" + new String(buffer, 0, bytesRead) + "'");

        System.out.println("Available at end: " + bais.available());

        // Close streams
        baos.close();
        bais.close();
    }

    private static void testByteArrayStreamProperties() throws IOException {
        System.out.println("--- ByteArrayStream Properties Tests ---");

        // Test ByteArrayOutputStream properties
        ByteArrayOutputStream baos = new ByteArrayOutputStream(16); // Initial capacity

        System.out.println("Initial size: " + baos.size());

        // Write data to test growth
        for (int i = 0; i < 20; i++) {
            baos.write(('A' + i));
        }

        System.out.println("Size after 20 writes: " + baos.size());

        // Test toString methods
        String asString = baos.toString();
        System.out.println("toString(): '" + asString + "'");

        try {
            String asUTF8 = baos.toString("UTF-8");
            System.out.println("toString(UTF-8): '" + asUTF8 + "'");
        } catch (Exception e) {
            System.out.println("toString(UTF-8) error: " + e.getMessage());
        }

        // Test reset
        baos.reset();
        System.out.println("Size after reset: " + baos.size());

        // Write again after reset
        baos.write("After reset".getBytes());
        System.out.println("Size after writing post-reset: " + baos.size());
        System.out.println("Content after reset: '" + baos.toString() + "'");

        // Test ByteArrayInputStream properties
        byte[] testData = "ByteArrayInputStream test data".getBytes();
        ByteArrayInputStream bais = new ByteArrayInputStream(testData);

        System.out.println("BAIS available: " + bais.available());
        System.out.println("BAIS mark supported: " + bais.markSupported());

        // Test skip
        long skipped = bais.skip(10);
        System.out.println("Skipped " + skipped + " bytes");
        System.out.println("Available after skip: " + bais.available());

        // Read remaining
        byte[] remaining = new byte[bais.available()];
        int read = bais.read(remaining);
        System.out.println("Read remaining " + read + " bytes: '" + new String(remaining, 0, read) + "'");

        baos.close();
        bais.close();
    }

    private static void testBinaryDataOperations() throws IOException {
        System.out.println("--- Binary Data Operations Tests ---");

        ByteArrayOutputStream baos = new ByteArrayOutputStream();

        // Write binary data patterns
        for (int i = 0; i < 256; i++) {
            baos.write(i);
        }

        // Write some specific patterns
        byte[] pattern1 = {(byte)0xDE, (byte)0xAD, (byte)0xBE, (byte)0xEF};
        byte[] pattern2 = {(byte)0xCA, (byte)0xFE, (byte)0xBA, (byte)0xBE};

        baos.write(pattern1);
        baos.write(pattern2, 1, 2); // Write FE BA

        System.out.println("Written binary data, total size: " + baos.size());

        // Read back and verify
        byte[] binaryData = baos.toByteArray();
        ByteArrayInputStream bais = new ByteArrayInputStream(binaryData);

        // Verify sequence 0-255
        boolean sequenceCorrect = true;
        for (int i = 0; i < 256; i++) {
            int b = bais.read();
            if (b != i) {
                sequenceCorrect = false;
                System.out.println("Sequence error at " + i + ": got " + b);
                break;
            }
        }
        System.out.println("Binary sequence 0-255 correct: " + sequenceCorrect);

        // Read patterns
        byte[] readPattern = new byte[6];
        int patternRead = bais.read(readPattern);
        System.out.println("Read " + patternRead + " pattern bytes");

        System.out.print("Pattern: ");
        for (int i = 0; i < patternRead; i++) {
            System.out.printf("0x%02X ", readPattern[i] & 0xFF);
        }
        System.out.println();

        baos.close();
        bais.close();
    }

    private static void testStreamCopying() throws IOException {
        System.out.println("--- Stream Copying Tests ---");

        // Create source data
        String sourceData = "This is test data for stream copying operations. " +
                          "It contains multiple sentences to test the copying process.";

        ByteArrayInputStream source = new ByteArrayInputStream(sourceData.getBytes());
        ByteArrayOutputStream destination = new ByteArrayOutputStream();

        // Copy using single byte reads
        int b;
        int byteCount = 0;
        while ((b = source.read()) != -1) {
            destination.write(b);
            byteCount++;
        }

        System.out.println("Copied " + byteCount + " bytes using single-byte operations");
        System.out.println("Destination size: " + destination.size());

        // Verify copy
        String copiedData = destination.toString();
        boolean copyCorrect = sourceData.equals(copiedData);
        System.out.println("Copy correct: " + copyCorrect);

        // Test copying with buffer
        source = new ByteArrayInputStream(sourceData.getBytes());
        destination.reset();

        byte[] buffer = new byte[16];
        int totalBytes = 0;
        int operations = 0;
        int bytesRead;

        while ((bytesRead = source.read(buffer)) != -1) {
            destination.write(buffer, 0, bytesRead);
            totalBytes += bytesRead;
            operations++;
        }

        System.out.println("Buffered copy: " + totalBytes + " bytes in " + operations + " operations");
        System.out.println("Buffered copy correct: " + sourceData.equals(destination.toString()));

        source.close();
        destination.close();
    }

    private static void testMarkAndReset() throws IOException {
        System.out.println("--- Mark and Reset Tests ---");

        String testData = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        ByteArrayInputStream bais = new ByteArrayInputStream(testData.getBytes());

        System.out.println("Mark supported: " + bais.markSupported());
        System.out.println("Initial available: " + bais.available());

        // Read some data
        byte[] initial = new byte[5];
        int initialRead = bais.read(initial);
        System.out.println("Initial read: " + new String(initial, 0, initialRead));
        System.out.println("Available after initial read: " + bais.available());

        // Mark current position
        bais.mark(20);
        System.out.println("Marked position");

        // Read more data
        byte[] middle = new byte[10];
        int middleRead = bais.read(middle);
        System.out.println("Middle read: " + new String(middle, 0, middleRead));
        System.out.println("Available after middle read: " + bais.available());

        // Reset to marked position
        bais.reset();
        System.out.println("Reset to marked position");
        System.out.println("Available after reset: " + bais.available());

        // Read again from marked position
        byte[] afterReset = new byte[10];
        int afterResetRead = bais.read(afterReset);
        System.out.println("After reset read: " + new String(afterReset, 0, afterResetRead));

        // Verify data matches
        String middleStr = new String(middle, 0, middleRead);
        String afterResetStr = new String(afterReset, 0, afterResetRead);
        System.out.println("Reset data matches: " + middleStr.equals(afterResetStr));

        // Test mark with read limit
        bais.reset();
        bais.mark(5); // Small read limit

        // Read more than read limit
        byte[] tooMuch = new byte[10];
        bais.read(tooMuch);
        bais.close();
    }

    private static void testExceptionHandling() throws IOException {
        System.out.println("--- Exception Handling Tests ---");

        // Test with null array
        try {
            new ByteArrayInputStream(null);
            System.out.println("ERROR: Should have thrown exception for null array");
        } catch (NullPointerException e) {
            System.out.println("Correctly caught NullPointerException for null array");
        }

        // Test with invalid offset/length
        byte[] testData = "Test data".getBytes();
        try {
            new ByteArrayInputStream(testData, -1, 5);
            System.out.println("ERROR: Should have thrown exception for negative offset");
        } catch (Exception e) {
            System.out.println("Correctly caught exception for negative offset: " + e.getClass().getSimpleName());
        }

        try {
            new ByteArrayInputStream(testData, 0, testData.length + 1);
            System.out.println("ERROR: Should have thrown exception for length > array length");
        } catch (Exception e) {
            System.out.println("Correctly caught exception for invalid length: " + e.getClass().getSimpleName());
        }

        // Test operations after close
        ByteArrayInputStream bais = new ByteArrayInputStream(testData);
        ByteArrayOutputStream baos = new ByteArrayOutputStream();

        bais.close();
        baos.close();

        // ByteArrayInputStream should still work after close (per Java spec)
        int b = bais.read();
        System.out.println("Read after close (should work): " + (b != -1 ? (char)b : "EOF"));

        // ByteArrayOutputStream should still work after close (per Java spec)
        baos.write(65);
        System.out.println("Write after close worked: " + (baos.size() > 0));
    }

    private static void testLargeDataOperations() throws IOException {
        System.out.println("--- Large Data Operations Tests ---");

        ByteArrayOutputStream baos = new ByteArrayOutputStream();

        // Write large amount of data
        int dataSize = 100000;
        for (int i = 0; i < dataSize; i++) {
            baos.write(i % 256);
        }

        System.out.println("Written " + dataSize + " bytes");
        System.out.println("ByteArrayOutputStream size: " + baos.size());

        // Convert to byte array
        byte[] largeData = baos.toByteArray();
        System.out.println("Byte array length: " + largeData.length);

        // Read back with ByteArrayInputStream
        ByteArrayInputStream bais = new ByteArrayInputStream(largeData);

        // Verify data integrity
        boolean dataCorrect = true;
        for (int i = 0; i < dataSize && dataCorrect; i++) {
            int expected = i % 256;
            int actual = bais.read();
            if (actual != expected) {
                dataCorrect = false;
                System.out.println("Data mismatch at position " + i + ": expected " + expected + ", got " + actual);
            }
        }

        System.out.println("Large data integrity check: " + (dataCorrect ? "PASSED" : "FAILED"));
        System.out.println("Remaining bytes: " + bais.available());

        baos.close();
        bais.close();
    }

    private static void testMemoryEfficiency() throws IOException {
        System.out.println("--- Memory Efficiency Tests ---");

        // Test ByteArrayOutputStream growth
        ByteArrayOutputStream baos = new ByteArrayOutputStream(8); // Small initial capacity

        System.out.println("Initial capacity: 8 bytes");

        // Write data in chunks and observe growth
        int[] chunkSizes = {10, 20, 50, 100};

        for (int chunkSize : chunkSizes) {
            int sizeBefore = baos.size();

            for (int i = 0; i < chunkSize; i++) {
                baos.write(65 + (i % 26)); // Write A-Z repeatedly
            }

            int sizeAfter = baos.size();
            System.out.println("After writing " + chunkSize + " bytes: size " + sizeBefore + " -> " + sizeAfter);
        }

        // Test writeTo method for efficiency
        File outputFile = new File(TEST_DIR, "memory_test.dat");
        try (FileOutputStream fos = new FileOutputStream(outputFile)) {
            baos.writeTo(fos);
            System.out.println("Written to file using writeTo: " + outputFile.length() + " bytes");
        }

        // Verify file content matches ByteArrayOutputStream
        try (FileInputStream fis = new FileInputStream(outputFile)) {
            byte[] fileData = new byte[(int) outputFile.length()];
            fis.read(fileData);

            byte[] baosData = baos.toByteArray();
            boolean matches = java.util.Arrays.equals(fileData, baosData);
            System.out.println("File content matches ByteArrayOutputStream: " + matches);
        }

        baos.close();
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
