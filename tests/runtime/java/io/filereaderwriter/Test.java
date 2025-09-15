import java.io.*;

/**
 * Tests for java.io.FileReader and FileWriter classes
 */
public class Test {
    private static final String TEST_DIR = "test_file_readers_writers";
    private static final String TEST_FILE = "reader_writer_test.txt";
    private static final String UNICODE_FILE = "unicode_test.txt";
    private static final String APPEND_FILE = "append_test.txt";

    public static void main(String[] args) {
        System.out.println("=== FileReader and FileWriter Tests ===");

        try {
            // Setup
            File testDir = new File(TEST_DIR);
            cleanup(testDir);
            testDir.mkdir();

            testBasicFileReaderWriter();
            testCharacterOperations();
            testUnicodeSupport();
            testAppendMode();
            testBufferOperations();
            testReaderWriterProperties();
            testExceptionHandling();
            testLargeTextOperations();
        } catch (Exception e) {
            System.out.println("ERROR: " + e.getMessage());
            e.printStackTrace();
        } finally {
            cleanup(new File(TEST_DIR));
        }

        System.out.println("=== FileReader/Writer Tests Complete ===");
    }

    private static void testBasicFileReaderWriter() throws IOException {
        System.out.println("--- Basic FileReader/Writer Tests ---");

        File testFile = new File(TEST_DIR, TEST_FILE);
        String testText = "Hello, World!\nThis is a test.\nLine 3 here.";

        // Write with FileWriter
        try (FileWriter writer = new FileWriter(testFile)) {
            writer.write(testText);
            System.out.println("Written text with FileWriter");
        }

        System.out.println("File size: " + testFile.length() + " bytes");

        // Read with FileReader - character by character
        try (FileReader reader = new FileReader(testFile)) {
            System.out.print("Character by character: ");
            int charCount = 0;
            int c;
            while ((c = reader.read()) != -1 && charCount < 15) {
                if (c == '\n') {
                    System.out.print("\\n");
                } else {
                    System.out.print((char) c);
                }
                charCount++;
            }
            System.out.println(" (" + charCount + " chars)");
        }

        // Read entire content
        try (FileReader reader = new FileReader(testFile)) {
            char[] buffer = new char[1024];
            int charsRead = reader.read(buffer);
            String content = new String(buffer, 0, charsRead);
            System.out.println("Total chars read: " + charsRead);
            System.out.println("Content lines:");
            String[] lines = content.split("\n");
            for (int i = 0; i < lines.length; i++) {
                System.out.println("  Line " + (i+1) + ": " + lines[i]);
            }
        }
    }

    private static void testCharacterOperations() throws IOException {
        System.out.println("--- Character Operations Tests ---");

        File testFile = new File(TEST_DIR, "char_ops_test.txt");

        // Write various character operations
        try (FileWriter writer = new FileWriter(testFile)) {
            // Single character
            writer.write('A');
            writer.write('B');
            writer.write('\n');

            // Character array
            char[] chars = {'H', 'e', 'l', 'l', 'o'};
            writer.write(chars);
            writer.write('\n');

            // Partial character array
            char[] moreChars = {'W', 'o', 'r', 'l', 'd', '!', '!', '!'};
            writer.write(moreChars, 0, 6); // Write "World!"
            writer.write('\n');

            // String operations
            writer.write("Full string");
            writer.write('\n');

            String partialString = "Partial string here";
            writer.write(partialString, 8, 6); // Write "string"
            writer.write('\n');

            System.out.println("Written various character operations");
        }

        // Read back and verify
        try (FileReader reader = new FileReader(testFile)) {
            char[] buffer = new char[1024];
            int charsRead = reader.read(buffer);
            String content = new String(buffer, 0, charsRead);

            System.out.println("Read content:");
            String[] lines = content.split("\n");
            for (int i = 0; i < lines.length; i++) {
                System.out.println("  Line " + (i+1) + ": '" + lines[i] + "'");
            }
        }
    }

    private static void testUnicodeSupport() throws IOException {
        System.out.println("--- Unicode Support Tests ---");

        File unicodeFile = new File(TEST_DIR, UNICODE_FILE);

        // Test various Unicode characters
        String unicodeText = "Unicode test:\n" +
                           "Latin: café, résumé\n" +
                           "Greek: α, β, γ, δ\n" +
                           "Math: ∑, ∏, ∆, ∞\n" +
                           "Emoji: 😀, 🌟, 🚀\n" +
                           "Chinese: 你好世界\n" +
                           "Arabic: مرحبا\n" +
                           "Special: ñ, ü, ç, €";

        // Write Unicode text
        try (FileWriter writer = new FileWriter(unicodeFile)) {
            writer.write(unicodeText);
            System.out.println("Written Unicode text");
        }

        System.out.println("Unicode file size: " + unicodeFile.length() + " bytes");

        // Read back Unicode text
        try (FileReader reader = new FileReader(unicodeFile)) {
            char[] buffer = new char[1024];
            int charsRead = reader.read(buffer);
            String readContent = new String(buffer, 0, charsRead);

            System.out.println("Read " + charsRead + " characters");
            System.out.println("Unicode content:");
            String[] lines = readContent.split("\n");
            for (int i = 0; i < lines.length; i++) {
                System.out.println("  " + lines[i]);
            }

            // Verify content matches
            boolean matches = unicodeText.equals(readContent);
            System.out.println("Unicode content matches: " + matches);
        }
    }

    private static void testAppendMode() throws IOException {
        System.out.println("--- Append Mode Tests ---");

        File appendFile = new File(TEST_DIR, APPEND_FILE);

        // Write initial content
        try (FileWriter writer = new FileWriter(appendFile)) {
            writer.write("Initial line\n");
        }

        long initialSize = appendFile.length();
        System.out.println("Initial file size: " + initialSize);

        // Append content
        try (FileWriter writer = new FileWriter(appendFile, true)) {
            writer.write("Appended line 1\n");
            writer.write("Appended line 2\n");
        }

        long appendedSize = appendFile.length();
        System.out.println("Size after append: " + appendedSize);
        System.out.println("Size increased by: " + (appendedSize - initialSize));

        // Read all content
        try (FileReader reader = new FileReader(appendFile)) {
            char[] buffer = new char[1024];
            int charsRead = reader.read(buffer);
            String content = new String(buffer, 0, charsRead);

            System.out.println("Final content:");
            String[] lines = content.split("\n");
            for (int i = 0; i < lines.length; i++) {
                System.out.println("  Line " + (i+1) + ": " + lines[i]);
            }
        }
    }

    private static void testBufferOperations() throws IOException {
        System.out.println("--- Buffer Operations Tests ---");

        File bufferFile = new File(TEST_DIR, "buffer_test.txt");

        // Test reading with different buffer sizes
        String testContent = "This is a longer text that will be used to test buffer operations. " +
                           "It contains multiple sentences. Each sentence tests different aspects. " +
                           "The goal is to verify that buffer operations work correctly.";

        // Write test content
        try (FileWriter writer = new FileWriter(bufferFile)) {
            writer.write(testContent);
        }

        // Test different buffer sizes
        int[] bufferSizes = {1, 5, 10, 50, 100};

        for (int bufferSize : bufferSizes) {
            try (FileReader reader = new FileReader(bufferFile)) {
                char[] buffer = new char[bufferSize];
                int totalChars = 0;
                int readOperations = 0;
                int charsRead;

                while ((charsRead = reader.read(buffer)) != -1) {
                    totalChars += charsRead;
                    readOperations++;
                }

                System.out.println("Buffer size " + bufferSize + ": " +
                                 totalChars + " chars in " + readOperations + " operations");
            }
        }

        // Test partial buffer reads
        try (FileReader reader = new FileReader(bufferFile)) {
            char[] buffer = new char[20];
            int charsRead = reader.read(buffer, 5, 10); // Read 10 chars starting at offset 5
            System.out.println("Partial read: " + charsRead + " chars");
            System.out.println("Buffer content: '" + new String(buffer, 5, charsRead) + "'");
        }
    }

    private static void testReaderWriterProperties() throws IOException {
        System.out.println("--- Reader/Writer Properties Tests ---");

        File propFile = new File(TEST_DIR, "properties_test.txt");

        // Create test file
        try (FileWriter writer = new FileWriter(propFile)) {
            writer.write("Property test content");
        }

        try (FileReader reader = new FileReader(propFile)) {
            // Test ready() method
            System.out.println("Reader ready: " + reader.ready());

            // Test skip
            long skipped = reader.skip(5);
            System.out.println("Skipped " + skipped + " characters");

            // Read remaining
            char[] buffer = new char[50];
            int charsRead = reader.read(buffer);
            String remaining = new String(buffer, 0, charsRead);
            System.out.println("After skip, read: '" + remaining + "'");

            // Test mark support
            System.out.println("Mark supported: " + reader.markSupported());
        }

        // Test writer properties
        try (FileWriter writer = new FileWriter(propFile, true)) {
            writer.write(" - appended");
            writer.flush(); // Test explicit flush
            System.out.println("Flushed writer");
        }

        // Verify flush worked
        try (FileReader reader = new FileReader(propFile)) {
            char[] buffer = new char[1024];
            int charsRead = reader.read(buffer);
            String content = new String(buffer, 0, charsRead);
            System.out.println("Content after flush: '" + content + "'");
        }
    }

    private static void testExceptionHandling() {
        System.out.println("--- Exception Handling Tests ---");

        // Test FileNotFoundException for reading
        try {
            new FileReader("nonexistent_file.txt");
            System.out.println("ERROR: Should have thrown FileNotFoundException");
        } catch (FileNotFoundException e) {
            System.out.println("Correctly caught FileNotFoundException for read: " + e.getMessage());
        }

        // Test IOException for writing to directory
        try {
            File dir = new File(TEST_DIR);
            new FileWriter(dir);
            System.out.println("ERROR: Should have thrown IOException for directory");
        } catch (IOException e) {
            System.out.println("Correctly caught IOException for directory write: " + e.getClass().getSimpleName());
        }

        // Test operations on closed reader
        try {
            File testFile = new File(TEST_DIR, "exception_test.txt");
            try (FileWriter writer = new FileWriter(testFile)) {
                writer.write("test content");
            }

            FileReader reader = new FileReader(testFile);
            reader.close();

            try {
                reader.read();
                System.out.println("ERROR: Should have thrown IOException for closed reader");
            } catch (IOException e) {
                System.out.println("Correctly caught IOException for closed reader: " + e.getClass().getSimpleName());
            }

        } catch (IOException e) {
            System.out.println("Setup error: " + e.getMessage());
        }
    }

    private static void testLargeTextOperations() throws IOException {
        System.out.println("--- Large Text Operations Tests ---");

        File largeFile = new File(TEST_DIR, "large_text.txt");

        // Generate large text content
        int lineCount = 1000;
        try (FileWriter writer = new FileWriter(largeFile)) {
            for (int i = 0; i < lineCount; i++) {
                writer.write("Line " + String.format("%04d", i) +
                           ": This is a test line with some content to make it longer.\n");
            }
        }

        System.out.println("Large file size: " + largeFile.length() + " bytes");

        // Count lines by reading
        try (FileReader reader = new FileReader(largeFile)) {
            int lines = 0;
            int c;
            while ((c = reader.read()) != -1) {
                if (c == '\n') {
                    lines++;
                }
            }
            System.out.println("Counted lines: " + lines);
        }

        // Test skip on large file
        try (FileReader reader = new FileReader(largeFile)) {
            // Skip to approximately middle
            long skipAmount = largeFile.length() / 2;
            long actualSkipped = reader.skip(skipAmount);
            System.out.println("Requested skip: " + skipAmount + ", actual: " + actualSkipped);

            // Read a few characters after skip
            char[] buffer = new char[50];
            int charsRead = reader.read(buffer);
            String content = new String(buffer, 0, Math.min(charsRead, 30));
            System.out.println("Content after skip: '" + content + "...'");
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
