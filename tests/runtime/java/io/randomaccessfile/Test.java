import java.io.*;

/**
 * Tests for java.io.RandomAccessFile class
 */
public class Test {
    private static final String TEST_DIR = "test_random_access_file";
    private static final String TEST_FILE = "random_access_test.dat";

    public static void main(String[] args) {
        System.out.println("=== RandomAccessFile Tests ===");

        try {
            // Setup
            File testDir = new File(TEST_DIR);
            cleanup(testDir);
            testDir.mkdir();

            testBasicRandomAccess();
            testDataTypeOperations();
            testSeekOperations();
            testFilePointerManagement();
            testReadWriteModes();
            testLargeFileOperations();
            testExceptionHandling();
        } catch (Exception e) {
            System.out.println("ERROR: " + e.getMessage());
            e.printStackTrace();
        } finally {
            cleanup(new File(TEST_DIR));
        }

        System.out.println("=== RandomAccessFile Tests Complete ===");
    }

    private static void testBasicRandomAccess() throws IOException {
        System.out.println("--- Basic Random Access Tests ---");

        File testFile = new File(TEST_DIR, TEST_FILE);

        // Create and write initial data
        try (RandomAccessFile raf = new RandomAccessFile(testFile, "rw")) {
            // Write some initial data
            raf.writeBytes("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ");

            System.out.println("Written initial data");
            System.out.println("File length: " + raf.length());
            System.out.println("File pointer: " + raf.getFilePointer());

            // Seek to beginning and read
            raf.seek(0);
            System.out.println("After seek(0), file pointer: " + raf.getFilePointer());

            // Read first 10 characters
            byte[] buffer = new byte[10];
            int bytesRead = raf.read(buffer);
            System.out.println("Read " + bytesRead + " bytes: '" + new String(buffer) + "'");
            System.out.println("File pointer after read: " + raf.getFilePointer());

            // Seek to middle and read
            raf.seek(15);
            int singleByte = raf.read();
            System.out.println("Byte at position 15: '" + (char) singleByte + "'");

            // Seek to end and write more data
            raf.seek(raf.length());
            raf.writeBytes("_EXTENDED");
            System.out.println("Extended file length: " + raf.length());

            // Read the extended part
            raf.seek(36);
            byte[] extended = new byte[9];
            raf.read(extended);
            System.out.println("Extended data: '" + new String(extended) + "'");
        }
    }

    private static void testDataTypeOperations() throws IOException {
        System.out.println("--- Data Type Operations Tests ---");

        File dataFile = new File(TEST_DIR, "data_types.dat");

        // Write various data types
        try (RandomAccessFile raf = new RandomAccessFile(dataFile, "rw")) {
            // Write primitive data types
            raf.writeBoolean(true);
            raf.writeByte(127);
            raf.writeShort(32767);
            raf.writeChar('A');
            raf.writeInt(2147483647);
            raf.writeLong(9223372036854775807L);
            raf.writeFloat(3.14159f);
            raf.writeDouble(2.718281828459045);

            // Write UTF string
            raf.writeUTF("Hello, RandomAccessFile!");

            // Write bytes
            raf.writeBytes("Raw bytes data");

            System.out.println("Written various data types");
            System.out.println("File length: " + raf.length());
        }

        // Read back the data types
        try (RandomAccessFile raf = new RandomAccessFile(dataFile, "r")) {
            raf.seek(0);

            boolean boolVal = raf.readBoolean();
            byte byteVal = raf.readByte();
            short shortVal = raf.readShort();
            char charVal = raf.readChar();
            int intVal = raf.readInt();
            long longVal = raf.readLong();
            float floatVal = raf.readFloat();
            double doubleVal = raf.readDouble();
            String utfVal = raf.readUTF();

            System.out.println("Read data types:");
            System.out.println("  boolean: " + boolVal);
            System.out.println("  byte: " + byteVal);
            System.out.println("  short: " + shortVal);
            System.out.println("  char: '" + charVal + "'");
            System.out.println("  int: " + intVal);
            System.out.println("  long: " + longVal);
            System.out.println("  float: " + floatVal);
            System.out.println("  double: " + doubleVal);
            System.out.println("  UTF string: '" + utfVal + "'");

            // Read remaining bytes
            byte[] remaining = new byte[(int)(raf.length() - raf.getFilePointer())];
            raf.read(remaining);
            System.out.println("  Raw bytes: '" + new String(remaining) + "'");
        }
    }

    private static void testSeekOperations() throws IOException {
        System.out.println("--- Seek Operations Tests ---");

        File seekFile = new File(TEST_DIR, "seek_test.dat");

        // Create file with known content
        try (RandomAccessFile raf = new RandomAccessFile(seekFile, "rw")) {
            for (int i = 0; i < 100; i++) {
                raf.writeBytes(String.format("%02d ", i));
            }

            System.out.println("Created file with 100 entries");
            System.out.println("File length: " + raf.length());

            // Test various seek positions
            long[] seekPositions = {0, 10, 50, 100, 150, 200, 299};

            for (long pos : seekPositions) {
                if (pos < raf.length()) {
                    raf.seek(pos);
                    System.out.println("Seek to " + pos + ", file pointer: " + raf.getFilePointer());

                    // Read a few bytes to see content
                    byte[] buffer = new byte[6];
                    int bytesRead = raf.read(buffer);
                    String content = new String(buffer, 0, bytesRead);
                    System.out.println("  Content: '" + content + "'");
                }
            }

            // Test seeking beyond file length
            try {
                raf.seek(raf.length() + 100);
                System.out.println("Seek beyond EOF successful, pointer: " + raf.getFilePointer());

                // Write at this position (should extend file)
                raf.writeBytes("BEYOND_EOF");
                System.out.println("New file length after write beyond EOF: " + raf.length());

                // Check the gap (should be zeros)
                raf.seek(300);
                byte[] gap = new byte[10];
                raf.read(gap);
                System.out.print("Gap content (should be zeros): ");
                for (byte b : gap) {
                    System.out.print(b + " ");
                }
                System.out.println();

            } catch (IOException e) {
                System.out.println("Seek beyond EOF failed: " + e.getMessage());
            }
        }
    }

    private static void testFilePointerManagement() throws IOException {
        System.out.println("--- File Pointer Management Tests ---");

        File pointerFile = new File(TEST_DIR, "pointer_test.dat");

        try (RandomAccessFile raf = new RandomAccessFile(pointerFile, "rw")) {
            // Test pointer movement with different operations
            System.out.println("Initial pointer: " + raf.getFilePointer());

            // Write and check pointer
            raf.writeInt(12345);
            System.out.println("After writeInt, pointer: " + raf.getFilePointer());

            raf.writeDouble(98.76);
            System.out.println("After writeDouble, pointer: " + raf.getFilePointer());

            raf.writeUTF("Test string");
            System.out.println("After writeUTF, pointer: " + raf.getFilePointer());

            // Seek back and read
            raf.seek(0);
            System.out.println("After seek(0), pointer: " + raf.getFilePointer());

            int intVal = raf.readInt();
            System.out.println("Read int: " + intVal + ", pointer: " + raf.getFilePointer());

            double doubleVal = raf.readDouble();
            System.out.println("Read double: " + doubleVal + ", pointer: " + raf.getFilePointer());

            String stringVal = raf.readUTF();
            System.out.println("Read UTF: '" + stringVal + "', pointer: " + raf.getFilePointer());

            // Test skipBytes
            raf.seek(0);
            int skipped = raf.skipBytes(4); // Skip the int
            System.out.println("Skipped " + skipped + " bytes, pointer: " + raf.getFilePointer());

            double directDouble = raf.readDouble();
            System.out.println("Read double after skip: " + directDouble);
        }
    }

    private static void testReadWriteModes() throws IOException {
        System.out.println("--- Read/Write Modes Tests ---");

        File modeFile = new File(TEST_DIR, "mode_test.dat");

        // Create file with "rw" mode
        try (RandomAccessFile raf = new RandomAccessFile(modeFile, "rw")) {
            raf.writeBytes("Read-write mode test");
            System.out.println("Created file with 'rw' mode");
        }

        // Test "r" mode (read-only)
        try (RandomAccessFile raf = new RandomAccessFile(modeFile, "r")) {
            System.out.println("Opened file with 'r' mode");

            // Read should work
            raf.seek(0);
            byte[] buffer = new byte[20];
            int bytesRead = raf.read(buffer);
            System.out.println("Read in 'r' mode: '" + new String(buffer, 0, bytesRead) + "'");

            // Write should fail
            try {
                raf.writeBytes("This should fail");
                System.out.println("ERROR: Write in 'r' mode should have failed");
            } catch (IOException e) {
                System.out.println("Correctly caught IOException in 'r' mode: " + e.getMessage());
            }
        }

        // Test "rw" mode
        try (RandomAccessFile raf = new RandomAccessFile(modeFile, "rw")) {
            System.out.println("Opened file with 'rw' mode");

            // Both read and write should work
            raf.seek(0);
            byte[] buffer = new byte[20];
            int bytesRead = raf.read(buffer);
            System.out.println("Read in 'rw' mode: '" + new String(buffer, 0, bytesRead) + "'");

            raf.seek(raf.length());
            raf.writeBytes(" - APPENDED");
            System.out.println("Write in 'rw' mode successful");

            // Verify the write
            raf.seek(0);
            buffer = new byte[(int) raf.length()];
            raf.read(buffer);
            System.out.println("Final content: '" + new String(buffer) + "'");
        }

        // Test "rws" mode (synchronous writes)
        try (RandomAccessFile raf = new RandomAccessFile(modeFile, "rws")) {
            System.out.println("Opened file with 'rws' mode");
            raf.writeBytes(" - RWS mode");
            System.out.println("Write in 'rws' mode successful (synchronous)");
        } catch (IllegalArgumentException e) {
            System.out.println("'rws' mode not supported: " + e.getMessage());
        }

        // Test "rwd" mode (synchronous data writes)
        try (RandomAccessFile raf = new RandomAccessFile(modeFile, "rwd")) {
            System.out.println("Opened file with 'rwd' mode");
            raf.writeBytes(" - RWD mode");
            System.out.println("Write in 'rwd' mode successful (synchronous data)");
        } catch (IllegalArgumentException e) {
            System.out.println("'rwd' mode not supported: " + e.getMessage());
        }
    }

    private static void testLargeFileOperations() throws IOException {
        System.out.println("--- Large File Operations Tests ---");

        File largeFile = new File(TEST_DIR, "large_test.dat");

        try (RandomAccessFile raf = new RandomAccessFile(largeFile, "rw")) {
            // Write data at various large positions
            long[] positions = {0, 1000, 10000, 100000, 1000000};

            for (long pos : positions) {
                raf.seek(pos);
                raf.writeBytes("Data at position " + pos);
                System.out.println("Written at position " + pos);
            }

            System.out.println("Large file length: " + raf.length());

            // Read back from random positions
            for (long pos : positions) {
                raf.seek(pos);
                byte[] buffer = new byte[20];
                int bytesRead = raf.read(buffer);
                String content = new String(buffer, 0, bytesRead);
                System.out.println("Position " + pos + ": '" + content + "'");
            }

            // Test seeking to very large position
            try {
                long largePos = 10000000L; // 10MB
                raf.seek(largePos);
                raf.writeBytes("Far position");
                System.out.println("Wrote at position " + largePos);
                System.out.println("File length now: " + raf.length());

                // Read it back
                raf.seek(largePos);
                byte[] buffer = new byte[20];
                int bytesRead = raf.read(buffer);
                String content = new String(buffer, 0, bytesRead);
                System.out.println("Content at " + largePos + ": '" + content + "'");

            } catch (Exception e) {
                System.out.println("Large position test failed: " + e.getMessage());
            }
        }
    }

    private static void testExceptionHandling() {
        System.out.println("--- Exception Handling Tests ---");

        // Test invalid mode
        try {
            File testFile = new File(TEST_DIR, "exception_test.dat");
            new RandomAccessFile(testFile, "invalid_mode");
            System.out.println("ERROR: Should have thrown exception for invalid mode");
        } catch (IllegalArgumentException e) {
            System.out.println("Correctly caught IllegalArgumentException for invalid mode: " + e.getMessage());
        } catch (IOException e) {
            System.out.println("Caught IOException for invalid mode: " + e.getMessage());
        }

        // Test non-existent file in read mode
        try {
            new RandomAccessFile("non_existent_file.dat", "r");
            System.out.println("ERROR: Should have thrown FileNotFoundException");
        } catch (FileNotFoundException e) {
            System.out.println("Correctly caught FileNotFoundException: " + e.getMessage());
        }

        // Test operations on closed file
        try {
            File testFile = new File(TEST_DIR, "closed_test.dat");
            RandomAccessFile raf = new RandomAccessFile(testFile, "rw");
            raf.writeBytes("test");
            raf.close();

            try {
                raf.read();
                System.out.println("ERROR: Should have thrown IOException for closed file");
            } catch (IOException e) {
                System.out.println("Correctly caught IOException for closed file: " + e.getMessage());
            }

        } catch (IOException e) {
            System.out.println("Setup error for closed file test: " + e.getMessage());
        }

        // Test invalid seek position
        File testFile = new File(TEST_DIR, "seek_exception_test.dat");
        try (RandomAccessFile raf = new RandomAccessFile(testFile, "rw")) {
            raf.seek(-1);
            System.out.println("ERROR: Should have thrown IOException for negative seek");
        } catch (IOException e) {
            System.out.println("Correctly caught IOException for negative seek: " + e.getMessage());
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
