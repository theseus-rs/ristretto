import java.io.*;
import java.nio.charset.Charset;

/**
 * Tests for java.io.InputStreamReader and OutputStreamWriter classes
 */
public class Test {
    private static final String TEST_DIR = "test_stream_readers_writers";
    private static final String TEST_FILE = "encoding_test.txt";
    private static final String UNICODE_FILE = "unicode_encoding_test.txt";

    public static void main(String[] args) {
        System.out.println("=== InputStreamReader and OutputStreamWriter Tests ===");

        try {
            // Setup
            File testDir = new File(TEST_DIR);
            cleanup(testDir);
            testDir.mkdir();

            testBasicStreamReaderWriter();
            testEncodingSupport();
            testUnicodeHandling();
            testBufferedOperations();
            testEncodingConversion();
            testCharsetHandling();
            testExceptionHandling();
        } catch (Exception e) {
            System.out.println("ERROR: " + e.getMessage());
            e.printStackTrace();
        } finally {
            cleanup(new File(TEST_DIR));
        }

        System.out.println("=== StreamReader/Writer Tests Complete ===");
    }

    private static void testBasicStreamReaderWriter() throws IOException {
        System.out.println("--- Basic StreamReader/Writer Tests ---");

        File testFile = new File(TEST_DIR, TEST_FILE);
        String testContent = "Hello, World!\nThis is a test of InputStreamReader and OutputStreamWriter.\nLine 3 here.";

        // Write with OutputStreamWriter
        try (FileOutputStream fos = new FileOutputStream(testFile);
             OutputStreamWriter osw = new OutputStreamWriter(fos)) {

            osw.write(testContent);
            System.out.println("Written content with OutputStreamWriter");

            // Test encoding property
            String encoding = osw.getEncoding();
            System.out.println("OutputStreamWriter encoding: " + encoding);
        }

        System.out.println("File size: " + testFile.length() + " bytes");

        // Read with InputStreamReader
        try (FileInputStream fis = new FileInputStream(testFile);
             InputStreamReader isr = new InputStreamReader(fis)) {

            String encoding = isr.getEncoding();
            System.out.println("InputStreamReader encoding: " + encoding);

            System.out.println("Reader ready: " + isr.ready());

            // Read character by character
            System.out.print("First 15 characters: ");
            for (int i = 0; i < 15; i++) {
                int c = isr.read();
                if (c != -1) {
                    if (c == '\n') {
                        System.out.print("\\n");
                    } else {
                        System.out.print((char) c);
                    }
                }
            }
            System.out.println();

            // Read remaining content
            char[] buffer = new char[1024];
            int charsRead = isr.read(buffer);
            String remaining = new String(buffer, 0, charsRead);
            System.out.println("Remaining content: '" + remaining + "'");
        }
    }

    private static void testEncodingSupport() throws IOException {
        System.out.println("--- Encoding Support Tests ---");

        String[] encodings = {"UTF-8", "UTF-16", "ISO-8859-1", "US-ASCII"};
        String testText = "Test encoding: café, résumé, naïve";

        for (String encoding : encodings) {
            File encodingFile = new File(TEST_DIR, "test_" + encoding.replace("-", "_") + ".txt");

            try {
                // Write with specific encoding
                try (FileOutputStream fos = new FileOutputStream(encodingFile);
                     OutputStreamWriter osw = new OutputStreamWriter(fos, encoding)) {

                    osw.write(testText);
                    System.out.println("Written with " + encoding + ", actual encoding: " + osw.getEncoding());
                }

                System.out.println("File size with " + encoding + ": " + encodingFile.length() + " bytes");

                // Read back with same encoding
                try (FileInputStream fis = new FileInputStream(encodingFile);
                     InputStreamReader isr = new InputStreamReader(fis, encoding)) {

                    char[] buffer = new char[1024];
                    int charsRead = isr.read(buffer);
                    String readText = new String(buffer, 0, charsRead);

                    System.out.println("Read with " + encoding + ", actual encoding: " + isr.getEncoding());
                    System.out.println("Content: '" + readText + "'");
                    System.out.println("Content matches: " + testText.equals(readText));
                }

            } catch (UnsupportedEncodingException e) {
                System.out.println("Encoding " + encoding + " not supported: " + e.getMessage());
            }

            System.out.println();
        }
    }

    private static void testUnicodeHandling() throws IOException {
        System.out.println("--- Unicode Handling Tests ---");

        File unicodeFile = new File(TEST_DIR, UNICODE_FILE);

        String unicodeText = "Unicode test:\n" +
                           "Latin: café, résumé, piñata\n" +
                           "Greek: α, β, γ, δ, ε, ζ\n" +
                           "Cyrillic: А, Б, В, Г, Д\n" +
                           "Math: ∑, ∏, ∆, ∞, ≠, ≤, ≥\n" +
                           "Currency: €, ¥, £, $\n" +
                           "Emoji: 😀, 🌟, 🚀, 🎉\n" +
                           "Chinese: 你好世界\n" +
                           "Japanese: こんにちは\n" +
                           "Arabic: مرحبا بالعالم";

        // Test with UTF-8
        try (FileOutputStream fos = new FileOutputStream(unicodeFile);
             OutputStreamWriter osw = new OutputStreamWriter(fos, "UTF-8")) {

            osw.write(unicodeText);
            System.out.println("Written Unicode text with UTF-8");
        }

        System.out.println("Unicode file size: " + unicodeFile.length() + " bytes");

        // Read back with UTF-8
        try (FileInputStream fis = new FileInputStream(unicodeFile);
             InputStreamReader isr = new InputStreamReader(fis, "UTF-8")) {

            char[] buffer = new char[2048];
            int charsRead = isr.read(buffer);
            String readContent = new String(buffer, 0, charsRead);

            System.out.println("Unicode content read back:");
            String[] lines = readContent.split("\n");
            for (int i = 0; i < lines.length; i++) {
                System.out.println("  " + lines[i]);
            }

            boolean matches = unicodeText.equals(readContent);
            System.out.println("Unicode content matches: " + matches);
        }

        // Test UTF-16 for comparison
        File utf16File = new File(TEST_DIR, "unicode_utf16.txt");
        try (FileOutputStream fos = new FileOutputStream(utf16File);
             OutputStreamWriter osw = new OutputStreamWriter(fos, "UTF-16")) {

            osw.write(unicodeText);
        }

        System.out.println("UTF-16 file size: " + utf16File.length() + " bytes");

        // Verify UTF-16 content
        try (FileInputStream fis = new FileInputStream(utf16File);
             InputStreamReader isr = new InputStreamReader(fis, "UTF-16")) {

            char[] buffer = new char[2048];
            int charsRead = isr.read(buffer);
            String utf16Content = new String(buffer, 0, charsRead);

            boolean utf16Matches = unicodeText.equals(utf16Content);
            System.out.println("UTF-16 content matches: " + utf16Matches);
        }
    }

    private static void testBufferedOperations() throws IOException {
        System.out.println("--- Buffered Operations Tests ---");

        File bufferedFile = new File(TEST_DIR, "buffered_encoding_test.txt");

        // Create test content
        StringBuilder content = new StringBuilder();
        for (int i = 0; i < 100; i++) {
            content.append("Line ").append(i).append(": Unicode test café résumé\n");
        }

        // Write with buffered OutputStreamWriter
        try (FileOutputStream fos = new FileOutputStream(bufferedFile);
             OutputStreamWriter osw = new OutputStreamWriter(fos, "UTF-8");
             BufferedWriter bw = new BufferedWriter(osw)) {

            bw.write(content.toString());
            System.out.println("Written buffered content with UTF-8");
        }

        // Read with buffered InputStreamReader
        try (FileInputStream fis = new FileInputStream(bufferedFile);
             InputStreamReader isr = new InputStreamReader(fis, "UTF-8");
             BufferedReader br = new BufferedReader(isr)) {

            int lineCount = 0;
            String line;
            while ((line = br.readLine()) != null) {
                lineCount++;
                if (lineCount <= 3 || lineCount > 97) {
                    System.out.println("Line " + lineCount + ": " + line);
                }
            }
            System.out.println("Total lines read: " + lineCount);
        }

        // Test different buffer sizes
        int[] bufferSizes = {512, 1024, 4096};
        for (int bufferSize : bufferSizes) {
            long startTime = System.currentTimeMillis();

            try (FileInputStream fis = new FileInputStream(bufferedFile);
                 InputStreamReader isr = new InputStreamReader(fis, "UTF-8");
                 BufferedReader br = new BufferedReader(isr, bufferSize)) {

                int chars = 0;
                while (br.read() != -1) {
                    chars++;
                }

                long time = System.currentTimeMillis() - startTime;
                System.out.println("Buffer size " + bufferSize + ": read " + chars + " chars in " + time + "ms");
            }
        }
    }

    private static void testEncodingConversion() throws IOException {
        System.out.println("--- Encoding Conversion Tests ---");

        String originalText = "Encoding conversion test: café, résumé, naïve, piñata";

        // Write with UTF-8
        File utf8File = new File(TEST_DIR, "utf8_source.txt");
        try (FileOutputStream fos = new FileOutputStream(utf8File);
             OutputStreamWriter osw = new OutputStreamWriter(fos, "UTF-8")) {
            osw.write(originalText);
        }

        // Read UTF-8 and write as ISO-8859-1
        File iso88591File = new File(TEST_DIR, "iso88591_target.txt");
        try (FileInputStream fis = new FileInputStream(utf8File);
             InputStreamReader isr = new InputStreamReader(fis, "UTF-8");
             FileOutputStream fos = new FileOutputStream(iso88591File);
             OutputStreamWriter osw = new OutputStreamWriter(fos, "ISO-8859-1")) {

            char[] buffer = new char[1024];
            int charsRead = isr.read(buffer);
            osw.write(buffer, 0, charsRead);

            System.out.println("Converted UTF-8 to ISO-8859-1");
        }

        // Read back ISO-8859-1
        try (FileInputStream fis = new FileInputStream(iso88591File);
             InputStreamReader isr = new InputStreamReader(fis, "ISO-8859-1")) {

            char[] buffer = new char[1024];
            int charsRead = isr.read(buffer);
            String convertedText = new String(buffer, 0, charsRead);

            System.out.println("Original: '" + originalText + "'");
            System.out.println("Converted: '" + convertedText + "'");
            System.out.println("Conversion successful: " + originalText.equals(convertedText));
        }

        System.out.println("UTF-8 file size: " + utf8File.length());
        System.out.println("ISO-8859-1 file size: " + iso88591File.length());
    }

    private static void testCharsetHandling() throws IOException {
        System.out.println("--- Charset Handling Tests ---");

        // Test with Charset objects
        File charsetFile = new File(TEST_DIR, "charset_test.txt");
        String testText = "Charset test: 日本語, 中文, العربية";

        Charset utf8Charset = Charset.forName("UTF-8");

        // Write with Charset
        try (FileOutputStream fos = new FileOutputStream(charsetFile);
             OutputStreamWriter osw = new OutputStreamWriter(fos, utf8Charset)) {

            osw.write(testText);
            System.out.println("Written with Charset object");
            System.out.println("Writer encoding: " + osw.getEncoding());
        }

        // Read with Charset
        try (FileInputStream fis = new FileInputStream(charsetFile);
             InputStreamReader isr = new InputStreamReader(fis, utf8Charset)) {

            System.out.println("Reader encoding: " + isr.getEncoding());

            char[] buffer = new char[1024];
            int charsRead = isr.read(buffer);
            String readText = new String(buffer, 0, charsRead);

            System.out.println("Read text: '" + readText + "'");
            System.out.println("Text matches: " + testText.equals(readText));
        }

        // Test default charset
        try (FileOutputStream fos = new FileOutputStream(charsetFile);
             OutputStreamWriter osw = new OutputStreamWriter(fos)) {

            System.out.println("Default charset: " + osw.getEncoding());
        }

        // Test available charsets
        System.out.println("Available charsets sample:");
        int count = 0;
        for (String charsetName : Charset.availableCharsets().keySet()) {
            System.out.println("  " + charsetName);
            if (++count >= 5) {
                System.out.println("  ... and more");
                break;
            }
        }
    }

    private static void testExceptionHandling() {
        System.out.println("--- Exception Handling Tests ---");

        // Test unsupported encoding
        try {
            File testFile = new File(TEST_DIR, "exception_test.txt");
            testFile.createNewFile();

            try (FileOutputStream fos = new FileOutputStream(testFile)) {
                new OutputStreamWriter(fos, "INVALID-ENCODING");
                System.out.println("ERROR: Should have thrown UnsupportedEncodingException");
            } catch (UnsupportedEncodingException e) {
                System.out.println("Correctly caught UnsupportedEncodingException: " + e.getMessage());
            }

            try (FileInputStream fis = new FileInputStream(testFile)) {
                new InputStreamReader(fis, "ANOTHER-INVALID-ENCODING");
                System.out.println("ERROR: Should have thrown UnsupportedEncodingException");
            } catch (UnsupportedEncodingException e) {
                System.out.println("Correctly caught UnsupportedEncodingException for reader: " + e.getMessage());
            }

        } catch (IOException e) {
            System.out.println("Setup error: " + e.getMessage());
        }

        // Test null charset
        try {
            File testFile = new File(TEST_DIR, "null_test.txt");
            testFile.createNewFile();

            try (FileOutputStream fos = new FileOutputStream(testFile)) {
                new OutputStreamWriter(fos, (String) null);
                System.out.println("ERROR: Should have thrown exception for null encoding");
            } catch (Exception e) {
                System.out.println("Correctly caught exception for null encoding: " + e.getClass().getSimpleName());
            }

        } catch (IOException e) {
            System.out.println("Null test setup error: " + e.getMessage());
        }

        // Test operations on closed streams
        try {
            File testFile = new File(TEST_DIR, "closed_test.txt");
            try (FileOutputStream fos = new FileOutputStream(testFile)) {
                fos.write("test".getBytes());
            }

            FileInputStream fis = new FileInputStream(testFile);
            InputStreamReader isr = new InputStreamReader(fis);
            fis.close();

            try {
                isr.read();
                System.out.println("ERROR: Should have thrown IOException for closed stream");
            } catch (IOException e) {
                System.out.println("Correctly caught IOException for closed stream: " + e.getClass().getSimpleName());
            }

        } catch (IOException e) {
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
