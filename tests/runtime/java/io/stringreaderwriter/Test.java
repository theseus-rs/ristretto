import java.io.*;

/**
 * Tests for java.io.StringReader and StringWriter classes
 */
public class Test {
    private static final String TEST_DIR = "test_string_readers_writers";

    public static void main(String[] args) {
        System.out.println("=== StringReader and StringWriter Tests ===");

        try {
            // Setup
            File testDir = new File(TEST_DIR);
            cleanup(testDir);
            testDir.mkdir();

            testBasicStringOperations();
            testStringReaderProperties();
            testStringWriterOperations();
            testMarkAndReset();
            testLargeStringOperations();
            testUnicodeSupport();
            testBufferedOperations();
            testExceptionHandling();
        } catch (Exception e) {
            System.out.println("ERROR: " + e.getMessage());
            e.printStackTrace();
        } finally {
            cleanup(new File(TEST_DIR));
        }

        System.out.println("=== StringReader/Writer Tests Complete ===");
    }

    private static void testBasicStringOperations() throws IOException {
        System.out.println("--- Basic String Operations Tests ---");

        // Test StringWriter
        StringWriter sw = new StringWriter();

        sw.write("Hello");
        sw.write(' ');
        sw.write("World");
        sw.write('!');
        sw.write('\n');

        // Write character array
        char[] chars = {'T', 'e', 's', 't'};
        sw.write(chars);
        sw.write('\n');

        // Write partial character array
        char[] moreChars = {'P', 'a', 'r', 't', 'i', 'a', 'l'};
        sw.write(moreChars, 1, 3); // Write "art"
        sw.write('\n');

        // Write string
        sw.write("String write");
        sw.write('\n');

        // Write partial string
        String partialStr = "Partial string test";
        sw.write(partialStr, 8, 6); // Write "string"
        sw.write('\n');

        String result = sw.toString();
        System.out.println("StringWriter result:");
        String[] lines = result.split("\n");
        for (int i = 0; i < lines.length; i++) {
            System.out.println("  Line " + (i+1) + ": '" + lines[i] + "'");
        }

        System.out.println("StringWriter buffer size: " + sw.getBuffer().length());

        // Test StringReader
        StringReader sr = new StringReader(result);

        System.out.println("StringReader ready: " + sr.ready());

        // Read character by character
        System.out.print("First 10 characters: ");
        for (int i = 0; i < 10; i++) {
            int c = sr.read();
            if (c != -1) {
                if (c == '\n') {
                    System.out.print("\\n");
                } else {
                    System.out.print((char) c);
                }
            }
        }
        System.out.println();

        // Read remaining into buffer
        char[] buffer = new char[1024];
        int charsRead = sr.read(buffer);
        String remaining = new String(buffer, 0, charsRead);
        System.out.println("Remaining content: '" + remaining.replace("\n", "\\n") + "'");

        sw.close();
        sr.close();
    }

    private static void testStringReaderProperties() throws IOException {
        System.out.println("--- StringReader Properties Tests ---");

        String testString = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        StringReader sr = new StringReader(testString);

        System.out.println("Test string length: " + testString.length());
        System.out.println("StringReader ready: " + sr.ready());
        System.out.println("Mark supported: " + sr.markSupported());

        // Test skip
        long skipped = sr.skip(5);
        System.out.println("Skipped " + skipped + " characters");

        // Read next 5 characters
        char[] buffer = new char[5];
        int charsRead = sr.read(buffer);
        System.out.println("Read after skip: '" + new String(buffer, 0, charsRead) + "'");

        // Test ready after partial read
        System.out.println("Ready after partial read: " + sr.ready());

        // Skip to near end
        skipped = sr.skip(20);
        System.out.println("Skipped " + skipped + " more characters");

        // Read remaining
        buffer = new char[10];
        charsRead = sr.read(buffer);
        System.out.println("Read remaining: '" + new String(buffer, 0, charsRead) + "'");

        System.out.println("Ready at end: " + sr.ready());

        // Try to read beyond end
        int c = sr.read();
        System.out.println("Read beyond end: " + c + " (should be -1)");

        sr.close();
    }

    private static void testStringWriterOperations() throws IOException {
        System.out.println("--- StringWriter Operations Tests ---");

        StringWriter sw = new StringWriter(16); // Initial capacity

        System.out.println("Initial buffer capacity: " + sw.getBuffer().capacity());

        // Write data to test growth
        for (int i = 0; i < 20; i++) {
            sw.write('A' + i);
        }

        System.out.println("After writing 20 chars, buffer capacity: " + sw.getBuffer().capacity());
        System.out.println("Buffer length: " + sw.getBuffer().length());
        System.out.println("Content: '" + sw.toString() + "'");

        // Test append operations
        sw.append(" - Appended");
        sw.append(' ');
        sw.append("More", 1, 3); // Append "or"
        sw.append('!');

        System.out.println("After appends: '" + sw.toString() + "'");

        // Test flush (should be no-op for StringWriter)
        sw.flush();
        System.out.println("After flush: '" + sw.toString() + "'");

        // Test getBuffer vs toString
        StringBuffer buffer = sw.getBuffer();
        String stringResult = sw.toString();

        System.out.println("Buffer equals toString: " + buffer.toString().equals(stringResult));
        System.out.println("Buffer is same object: " + (buffer == sw.getBuffer()));

        // Modify the buffer directly
        buffer.append(" - Direct buffer append");
        System.out.println("After direct buffer modification: '" + sw.toString() + "'");

        sw.close();
    }

    private static void testMarkAndReset() throws IOException {
        System.out.println("--- Mark and Reset Tests ---");

        String testData = "Mark and reset test: 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        StringReader sr = new StringReader(testData);

        System.out.println("Mark supported: " + sr.markSupported());

        // Read some characters
        char[] initial = new char[10];
        int initialRead = sr.read(initial);
        System.out.println("Initial read: '" + new String(initial, 0, initialRead) + "'");

        // Mark current position
        sr.mark(20);
        System.out.println("Marked position after reading " + initialRead + " characters");

        // Read more characters
        char[] middle = new char[15];
        int middleRead = sr.read(middle);
        System.out.println("Middle read: '" + new String(middle, 0, middleRead) + "'");

        // Reset to marked position
        sr.reset();
        System.out.println("Reset to marked position");

        // Read again from marked position
        char[] afterReset = new char[15];
        int afterResetRead = sr.read(afterReset);
        System.out.println("After reset read: '" + new String(afterReset, 0, afterResetRead) + "'");

        // Verify data matches
        String middleStr = new String(middle, 0, middleRead);
        String afterResetStr = new String(afterReset, 0, afterResetRead);
        System.out.println("Reset data matches: " + middleStr.equals(afterResetStr));

        // Test mark with read limit
        sr.reset();
        sr.mark(5); // Small read limit

        // Read more than read limit
        char[] tooMuch = new char[10];
        sr.read(tooMuch);

        try {
            sr.reset();
            System.out.println("Reset successful after exceeding read limit");
        } catch (IOException e) {
            System.out.println("Reset failed after exceeding read limit: " + e.getMessage());
        }

        sr.close();
    }

    private static void testLargeStringOperations() throws IOException {
        System.out.println("--- Large String Operations Tests ---");

        // Build large string
        StringBuilder largeBuilder = new StringBuilder();
        int lineCount = 1000;
        for (int i = 0; i < lineCount; i++) {
            largeBuilder.append("Line ").append(String.format("%04d", i)).append(": This is test content\n");
        }
        String largeString = largeBuilder.toString();

        System.out.println("Large string length: " + largeString.length());

        // Test StringWriter with large content
        StringWriter sw = new StringWriter();
        long startTime = System.currentTimeMillis();

        // Write in chunks
        int chunkSize = 1000;
        for (int i = 0; i < largeString.length(); i += chunkSize) {
            int end = Math.min(i + chunkSize, largeString.length());
            sw.write(largeString.substring(i, end));
        }

        long writeTime = System.currentTimeMillis() - startTime;
        System.out.println("Large string write time: " + writeTime + "ms");
        System.out.println("StringWriter result length: " + sw.toString().length());

        // Test StringReader with large content
        StringReader sr = new StringReader(largeString);
        startTime = System.currentTimeMillis();

        char[] buffer = new char[1024];
        int totalChars = 0;
        int charsRead;
        while ((charsRead = sr.read(buffer)) != -1) {
            totalChars += charsRead;
        }

        long readTime = System.currentTimeMillis() - startTime;
        System.out.println("Large string read time: " + readTime + "ms");
        System.out.println("Total characters read: " + totalChars);
        System.out.println("Read all characters: " + (totalChars == largeString.length()));

        sw.close();
        sr.close();
    }

    private static void testUnicodeSupport() throws IOException {
        System.out.println("--- Unicode Support Tests ---");

        String unicodeText = "Unicode test:\n" +
                           "Latin: café, résumé, naïve\n" +
                           "Greek: α, β, γ, δ, ε\n" +
                           "Math: ∑, ∏, ∆, ∞, ≠\n" +
                           "Emoji: 😀, 🌟, 🚀\n" +
                           "Chinese: 你好世界\n" +
                           "Arabic: مرحبا";

        // Test StringWriter with Unicode
        StringWriter sw = new StringWriter();
        sw.write(unicodeText);

        String result = sw.toString();
        System.out.println("Unicode StringWriter result length: " + result.length());
        System.out.println("Unicode content matches: " + unicodeText.equals(result));

        // Test StringReader with Unicode
        StringReader sr = new StringReader(unicodeText);

        char[] buffer = new char[1024];
        int charsRead = sr.read(buffer);
        String readResult = new String(buffer, 0, charsRead);

        System.out.println("Unicode StringReader result:");
        String[] lines = readResult.split("\n");
        for (int i = 0; i < lines.length; i++) {
            System.out.println("  " + lines[i]);
        }

        System.out.println("Unicode read matches: " + unicodeText.equals(readResult));

        // Test character-by-character Unicode reading
        sr = new StringReader("🚀🌟😀"); // Multi-byte Unicode characters
        System.out.print("Character-by-character Unicode: ");
        int c;
        while ((c = sr.read()) != -1) {
            System.out.print((char) c);
        }
        System.out.println();

        sw.close();
        sr.close();
    }

    private static void testBufferedOperations() throws IOException {
        System.out.println("--- Buffered Operations Tests ---");

        String testContent = "This is test content for buffered operations. " +
                           "It contains multiple sentences to test buffering. " +
                           "Each sentence tests different aspects of the implementation.";

        // Test BufferedReader with StringReader
        StringReader sr = new StringReader(testContent);
        BufferedReader br = new BufferedReader(sr);

        System.out.println("BufferedReader ready: " + br.ready());

        // Read line by line (even though there are no line breaks)
        String line = br.readLine();
        System.out.println("Read line: '" + line + "'");

        String nextLine = br.readLine();
        System.out.println("Next line: " + nextLine + " (should be null)");

        br.close();

        // Test BufferedWriter with StringWriter
        StringWriter sw = new StringWriter();
        BufferedWriter bw = new BufferedWriter(sw);

        bw.write("Buffered line 1");
        bw.newLine();
        bw.write("Buffered line 2");
        bw.newLine();
        bw.write("Buffered line 3");
        bw.flush();

        String bufferedResult = sw.toString();
        System.out.println("Buffered write result:");
        String[] lines = bufferedResult.split("\n");
        for (int i = 0; i < lines.length; i++) {
            System.out.println("  Line " + (i+1) + ": '" + lines[i] + "'");
        }

        // Test reading the buffered result
        sr = new StringReader(bufferedResult);
        br = new BufferedReader(sr);

        String readLine;
        int lineNum = 1;
        while ((readLine = br.readLine()) != null) {
            System.out.println("Read buffered line " + lineNum + ": '" + readLine + "'");
            lineNum++;
        }

        bw.close();
        br.close();
    }

    private static void testExceptionHandling() {
        System.out.println("--- Exception Handling Tests ---");

        // Test null string
        try {
            new StringReader(null);
            System.out.println("ERROR: Should have thrown exception for null string");
        } catch (NullPointerException e) {
            System.out.println("Correctly caught NullPointerException for null string");
        }

        // Test operations on closed StringReader
        try {
            StringReader sr = new StringReader("test content");
            sr.close();

            sr.read();
            System.out.println("Read after close successful (StringReader allows this)");
        } catch (IOException e) {
            System.out.println("IOException on closed StringReader: " + e.getMessage());
        }

        // Test operations on closed StringWriter
        try {
            StringWriter sw = new StringWriter();
            sw.close();

            sw.write("test");
            System.out.println("Write after close successful (StringWriter allows this)");
        } catch (IOException e) {
            System.out.println("IOException on closed StringWriter: " + e.getMessage());
        }

        // Test invalid mark/reset operations
        try {
            StringReader sr = new StringReader("test");

            try {
                sr.reset(); // Reset without mark
                System.out.println("ERROR: Should have thrown IOException for reset without mark");
            } catch (IOException e) {
                System.out.println("Correctly caught IOException for reset without mark");
            }

            sr.close();
        } catch (Exception e) {
            System.out.println("Mark/reset test error: " + e.getMessage());
        }

        // Test invalid array operations
        try {
            StringReader sr = new StringReader("test content");

            try {
                char[] buffer = new char[10];
                sr.read(buffer, -1, 5);
                System.out.println("ERROR: Should have thrown exception for negative offset");
            } catch (IndexOutOfBoundsException e) {
                System.out.println("Correctly caught IndexOutOfBoundsException for negative offset");
            }

            sr.close();
        } catch (Exception e) {
            System.out.println("Array operation test error: " + e.getMessage());
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
