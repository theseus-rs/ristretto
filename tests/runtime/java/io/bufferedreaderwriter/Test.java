import java.io.*;

/**
 * Tests for java.io.BufferedReader and BufferedWriter classes
 */
public class Test {
    private static final String TEST_DIR = "test_buffered_readers_writers";
    private static final String TEST_FILE = "buffered_test.txt";
    private static final String LINE_FILE = "line_test.txt";

    public static void main(String[] args) {
        System.out.println("=== BufferedReader and BufferedWriter Tests ===");

        try {
            // Setup
            File testDir = new File(TEST_DIR);
            cleanup(testDir);
            testDir.mkdir();

            testBasicBufferedOperations();
            testLineOperations();
            testMarkAndReset();
            testBufferSizes();
            testMixedOperations();
            testExceptionHandling();
        } catch (Exception e) {
            System.out.println("ERROR: " + e.getMessage());
            e.printStackTrace();
        } finally {
            cleanup(new File(TEST_DIR));
        }

        System.out.println("=== BufferedReader/Writer Tests Complete ===");
    }

    private static void testBasicBufferedOperations() throws IOException {
        System.out.println("--- Basic Buffered Operations Tests ---");

        File testFile = new File(TEST_DIR, TEST_FILE);

        // Write with BufferedWriter
        try (FileWriter fw = new FileWriter(testFile);
             BufferedWriter bw = new BufferedWriter(fw)) {

            bw.write("Hello, Buffered World!");
            bw.newLine();
            bw.write("Second line here");
            bw.newLine();
            bw.write("Third line");

            System.out.println("Written content with BufferedWriter");
        }

        System.out.println("File size: " + testFile.length() + " bytes");

        // Read with BufferedReader
        try (FileReader fr = new FileReader(testFile);
             BufferedReader br = new BufferedReader(fr)) {

            System.out.println("BufferedReader ready: " + br.ready());

            // Read character by character for first few
            System.out.print("First 10 chars: ");
            for (int i = 0; i < 10; i++) {
                int c = br.read();
                if (c != -1) {
                    System.out.print((char) c);
                }
            }
            System.out.println();

            // Read remaining content
            char[] buffer = new char[1024];
            int charsRead = br.read(buffer);
            String remaining = new String(buffer, 0, charsRead);
            System.out.println("Remaining content: '" + remaining + "'");
        }
    }

    private static void testLineOperations() throws IOException {
        System.out.println("--- Line Operations Tests ---");

        File lineFile = new File(TEST_DIR, LINE_FILE);

        // Write lines with BufferedWriter
        String[] testLines = {
            "First line of text",
            "Second line with numbers: 12345",
            "Third line with special chars: !@#$%",
            "",  // Empty line
            "Fifth line after empty",
            "Last line"
        };

        try (FileWriter fw = new FileWriter(lineFile);
             BufferedWriter bw = new BufferedWriter(fw)) {

            for (String line : testLines) {
                bw.write(line);
                bw.newLine();
            }
            System.out.println("Written " + testLines.length + " lines");
        }

        // Read lines with BufferedReader
        try (FileReader fr = new FileReader(lineFile);
             BufferedReader br = new BufferedReader(fr)) {

            String line;
            int lineNumber = 1;
            while ((line = br.readLine()) != null) {
                System.out.println("Line " + lineNumber + ": '" + line + "'");
                lineNumber++;
            }
            System.out.println("Total lines read: " + (lineNumber - 1));
        }

        // Test readLine() edge cases
        try (FileReader fr = new FileReader(lineFile);
             BufferedReader br = new BufferedReader(fr)) {

            // Read first line
            String firstLine = br.readLine();
            System.out.println("First line: '" + firstLine + "'");

            // Skip some characters
            long skipped = br.skip(10);
            System.out.println("Skipped " + skipped + " characters");

            // Read next line (should be partial)
            String partialLine = br.readLine();
            System.out.println("Partial line after skip: '" + partialLine + "'");
        }
    }

    private static void testMarkAndReset() throws IOException {
        System.out.println("--- Mark and Reset Tests ---");

        File markFile = new File(TEST_DIR, "mark_test.txt");

        // Create test content
        try (FileWriter fw = new FileWriter(markFile);
             BufferedWriter bw = new BufferedWriter(fw)) {

            for (int i = 0; i < 20; i++) {
                bw.write("Line " + i + ": Some test content here");
                bw.newLine();
            }
        }

        // Test mark and reset
        try (FileReader fr = new FileReader(markFile);
             BufferedReader br = new BufferedReader(fr)) {

            System.out.println("Mark supported: " + br.markSupported());

            // Read some content
            String line1 = br.readLine();
            String line2 = br.readLine();
            System.out.println("Read line 1: '" + line1 + "'");
            System.out.println("Read line 2: '" + line2 + "'");

            // Mark current position
            br.mark(1000);
            System.out.println("Marked position after reading 2 lines");

            // Read more content
            String line3 = br.readLine();
            String line4 = br.readLine();
            System.out.println("Read line 3: '" + line3 + "'");
            System.out.println("Read line 4: '" + line4 + "'");

            // Reset to marked position
            br.reset();
            System.out.println("Reset to marked position");

            // Read again from marked position
            String line3Again = br.readLine();
            String line4Again = br.readLine();
            System.out.println("Read line 3 again: '" + line3Again + "'");
            System.out.println("Read line 4 again: '" + line4Again + "'");

            // Verify they match
            boolean line3Match = (line3 != null && line3.equals(line3Again));
            boolean line4Match = (line4 != null && line4.equals(line4Again));
            System.out.println("Line 3 matches: " + line3Match);
            System.out.println("Line 4 matches: " + line4Match);
        }
    }

    private static void testBufferSizes() throws IOException {
        System.out.println("--- Buffer Size Tests ---");

        File bufferFile = new File(TEST_DIR, "buffer_size_test.txt");

        // Create test content
        StringBuilder content = new StringBuilder();
        for (int i = 0; i < 100; i++) {
            content.append("Line ").append(i).append(": This is test content with some length\n");
        }

        // Test different buffer sizes
        int[] bufferSizes = {8, 16, 64, 256, 1024, 8192};

        for (int bufferSize : bufferSizes) {
            System.out.println("Testing buffer size: " + bufferSize);

            // Write with specific buffer size
            try (FileWriter fw = new FileWriter(bufferFile);
                 BufferedWriter bw = new BufferedWriter(fw, bufferSize)) {

                bw.write(content.toString());
                System.out.println("  Written with buffer size " + bufferSize);
            }

            // Read with specific buffer size
            try (FileReader fr = new FileReader(bufferFile);
                 BufferedReader br = new BufferedReader(fr, bufferSize)) {

                int linesRead = 0;
                while (br.readLine() != null) {
                    linesRead++;
                }
                System.out.println("  Read " + linesRead + " lines with buffer size " + bufferSize);
            }

            // Read characters with specific buffer size
            try (FileReader fr = new FileReader(bufferFile);
                 BufferedReader br = new BufferedReader(fr, bufferSize)) {

                int charsRead = 0;
                while (br.read() != -1) {
                    charsRead++;
                }
                System.out.println("  Read " + charsRead + " characters with buffer size " + bufferSize);
            }
        }
    }

    private static void testMixedOperations() throws IOException {
        System.out.println("--- Mixed Operations Tests ---");

        File mixedFile = new File(TEST_DIR, "mixed_test.txt");

        // Write with mixed operations
        try (FileWriter fw = new FileWriter(mixedFile);
             BufferedWriter bw = new BufferedWriter(fw)) {

            // Single characters
            bw.write('H');
            bw.write('e');
            bw.write('l');
            bw.write('l');
            bw.write('o');
            bw.newLine();

            // Character array
            char[] chars = {'W', 'o', 'r', 'l', 'd'};
            bw.write(chars);
            bw.newLine();

            // Partial character array
            char[] moreChars = {'T', 'e', 's', 't', 'i', 'n', 'g'};
            bw.write(moreChars, 1, 4); // Write "esti"
            bw.newLine();

            // String operations
            bw.write("Full string line");
            bw.newLine();

            String partial = "Partial string content";
            bw.write(partial, 8, 6); // Write "string"
            bw.newLine();

            // Explicit flush
            bw.flush();
            System.out.println("Written mixed content");
        }

        // Read with mixed operations
        try (FileReader fr = new FileReader(mixedFile);
             BufferedReader br = new BufferedReader(fr)) {

            // Read first line normally
            String line1 = br.readLine();
            System.out.println("Line 1: '" + line1 + "'");

            // Read some characters from second line
            char[] buffer = new char[3];
            int charsRead = br.read(buffer);
            System.out.println("Read " + charsRead + " chars: '" + new String(buffer, 0, charsRead) + "'");

            // Read rest of second line
            String restOfLine2 = br.readLine();
            System.out.println("Rest of line 2: '" + restOfLine2 + "'");

            // Skip some characters
            long skipped = br.skip(2);
            System.out.println("Skipped " + skipped + " characters");

            // Read remaining lines
            String line;
            int lineNum = 3;
            while ((line = br.readLine()) != null) {
                System.out.println("Line " + lineNum + ": '" + line + "'");
                lineNum++;
            }
        }
    }

    private static void testExceptionHandling() {
        System.out.println("--- Exception Handling Tests ---");

        // Test invalid buffer size
        try {
            File testFile = new File(TEST_DIR, "exception_test.txt");
            try (FileWriter fw = new FileWriter(testFile)) {
                fw.write("test");
            }

            try (FileReader fr = new FileReader(testFile)) {
                new BufferedReader(fr, 0);
                System.out.println("ERROR: Should have thrown exception for zero buffer size");
            } catch (IllegalArgumentException e) {
                System.out.println("Correctly caught IllegalArgumentException for zero buffer size");
            }

            try (FileReader fr = new FileReader(testFile)) {
                new BufferedReader(fr, -1);
                System.out.println("ERROR: Should have thrown exception for negative buffer size");
            } catch (IllegalArgumentException e) {
                System.out.println("Correctly caught IllegalArgumentException for negative buffer size");
            }

        } catch (Exception e) {
            System.out.println("Setup error: " + e.getMessage());
        }

        // Test reset without mark
        try {
            File testFile = new File(TEST_DIR, "reset_test.txt");
            try (FileWriter fw = new FileWriter(testFile);
                 BufferedWriter bw = new BufferedWriter(fw)) {
                bw.write("Test content for reset");
                bw.newLine();
            }

            try (FileReader fr = new FileReader(testFile);
                 BufferedReader br = new BufferedReader(fr)) {

                br.readLine(); // Read some content

                try {
                    br.reset(); // Reset without mark
                    System.out.println("ERROR: Should have thrown IOException for reset without mark");
                } catch (IOException e) {
                    System.out.println("Correctly caught IOException for reset without mark");
                }
            }

        } catch (Exception e) {
            System.out.println("Reset test error: " + e.getMessage());
        }

        // Test operations on closed streams
        try {
            File testFile = new File(TEST_DIR, "closed_test.txt");
            try (FileWriter fw = new FileWriter(testFile)) {
                fw.write("test content");
            }

            BufferedReader br = new BufferedReader(new FileReader(testFile));
            br.close();

            try {
                br.readLine();
                System.out.println("ERROR: Should have thrown IOException for closed reader");
            } catch (IOException e) {
                System.out.println("Correctly caught IOException for closed reader");
            }

        } catch (Exception e) {
            System.out.println("Closed stream test error: " + e.getMessage());
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
