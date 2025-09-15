import java.io.*;
import java.util.Locale;

/**
 * Tests for java.io.PrintWriter and PrintStream classes
 */
public class Test {
    private static final String TEST_DIR = "test_print_classes";
    private static final String PRINT_WRITER_FILE = "printwriter_test.txt";
    private static final String PRINT_STREAM_FILE = "printstream_test.dat";

    public static void main(String[] args) {
        System.out.println("=== PrintWriter and PrintStream Tests ===");

        try {
            // Setup
            File testDir = new File(TEST_DIR);
            cleanup(testDir);
            testDir.mkdir();

            testPrintWriterBasics();
            testPrintWriterFormatting();
            testPrintStreamBasics();
            testPrintStreamFormatting();
            testAutoFlush();
            testErrorHandling();
            testCharsetHandling();
        } catch (Exception e) {
            System.out.println("ERROR: " + e.getMessage());
            e.printStackTrace();
        } finally {
            cleanup(new File(TEST_DIR));
        }

        System.out.println("=== Print Classes Tests Complete ===");
    }

    private static void testPrintWriterBasics() throws IOException {
        System.out.println("--- PrintWriter Basics Tests ---");

        File testFile = new File(TEST_DIR, PRINT_WRITER_FILE);

        // Test PrintWriter with FileWriter
        try (FileWriter fw = new FileWriter(testFile);
             PrintWriter pw = new PrintWriter(fw)) {

            // Test basic print methods
            pw.print("Hello");
            pw.print(' ');
            pw.print("World");
            pw.println("!");

            pw.println("Second line");
            pw.println();  // Empty line
            pw.println("Fourth line");

            // Test different data types
            pw.print("Integer: ");
            pw.println(42);
            pw.print("Long: ");
            pw.println(123456789L);
            pw.print("Float: ");
            pw.println(3.14f);
            pw.print("Double: ");
            pw.println(2.718281828);
            pw.print("Boolean: ");
            pw.println(true);
            pw.print("Character: ");
            pw.println('X');

            // Test object printing
            pw.print("Object: ");
            pw.println(new StringBuilder("StringBuilder content"));

            // Test null handling
            pw.print("Null string: ");
            pw.println((String) null);
            pw.print("Null object: ");
            pw.println((Object) null);

            System.out.println("Written various data types with PrintWriter");
        }

        // Read back and verify
        try (FileReader fr = new FileReader(testFile);
             BufferedReader br = new BufferedReader(fr)) {

            String line;
            int lineNumber = 1;
            while ((line = br.readLine()) != null) {
                System.out.println("Line " + lineNumber + ": '" + line + "'");
                lineNumber++;
            }
        }

        System.out.println("File size: " + testFile.length() + " bytes");
    }

    private static void testPrintWriterFormatting() throws IOException {
        System.out.println("--- PrintWriter Formatting Tests ---");

        File formatFile = new File(TEST_DIR, "format_test.txt");

        try (FileWriter fw = new FileWriter(formatFile);
             PrintWriter pw = new PrintWriter(fw)) {

            // Test printf formatting
            pw.printf("Formatted integer: %d%n", 42);
            pw.printf("Formatted hex: 0x%x%n", 255);
            pw.printf("Formatted octal: %o%n", 64);
            pw.printf("Formatted float: %.2f%n", 3.14159);
            pw.printf("Formatted scientific: %e%n", 1234.5);
            pw.printf("Formatted string: '%s'%n", "Hello");
            pw.printf("Formatted char: %c%n", 'A');
            pw.printf("Formatted boolean: %b%n", true);

            // Test width and alignment
            pw.printf("Right aligned: '%10s'%n", "right");
            pw.printf("Left aligned: '%-10s'%n", "left");
            pw.printf("Zero padded: %05d%n", 42);
            pw.printf("Space padded: % d%n", 42);
            pw.printf("Sign always: %+d%n", 42);

            // Test multiple arguments
            pw.printf("Multiple: %s, %d, %.2f%n", "text", 123, 45.67);

            // Test format with locale (using default)
            pw.format("Locale format: %,d%n", 1234567);

            System.out.println("Written formatted content");
        }

        // Read and display formatted content
        try (FileReader fr = new FileReader(formatFile);
             BufferedReader br = new BufferedReader(fr)) {

            System.out.println("Formatted content:");
            String line;
            while ((line = br.readLine()) != null) {
                System.out.println("  " + line);
            }
        }
    }

    private static void testPrintStreamBasics() throws IOException {
        System.out.println("--- PrintStream Basics Tests ---");

        File testFile = new File(TEST_DIR, PRINT_STREAM_FILE);

        // Test PrintStream with FileOutputStream
        try (FileOutputStream fos = new FileOutputStream(testFile);
             PrintStream ps = new PrintStream(fos)) {

            // Test basic print methods
            ps.print("PrintStream test");
            ps.println();
            ps.println("Second line");

            // Test different data types
            ps.println("Integer: " + 42);
            ps.println("Long: " + 123456789L);
            ps.println("Float: " + 3.14f);
            ps.println("Double: " + 2.718281828);
            ps.println("Boolean: " + true);
            ps.println("Character: " + 'Y');

            // Test byte array writing
            byte[] bytes = "Byte array content\n".getBytes();
            ps.write(bytes);

            // Test single byte writing
            ps.write(65); // 'A'
            ps.write(66); // 'B'
            ps.println();

            // Test partial byte array writing
            byte[] moreBytes = "TESTING123".getBytes();
            ps.write(moreBytes, 0, 7); // Write "TESTING"
            ps.println();

            System.out.println("Written content with PrintStream");
        }

        // Read back as text
        try (FileInputStream fis = new FileInputStream(testFile);
             InputStreamReader isr = new InputStreamReader(fis);
             BufferedReader br = new BufferedReader(isr)) {

            System.out.println("PrintStream content:");
            String line;
            int lineNumber = 1;
            while ((line = br.readLine()) != null) {
                System.out.println("Line " + lineNumber + ": '" + line + "'");
                lineNumber++;
            }
        }
    }

    private static void testPrintStreamFormatting() throws IOException {
        System.out.println("--- PrintStream Formatting Tests ---");

        File formatFile = new File(TEST_DIR, "printstream_format.txt");

        try (FileOutputStream fos = new FileOutputStream(formatFile);
             PrintStream ps = new PrintStream(fos)) {

            // Test printf formatting (same as PrintWriter)
            ps.printf("PS Formatted integer: %d%n", 99);
            ps.printf("PS Formatted hex: 0x%x%n", 255);
            ps.printf("PS Formatted float: %.3f%n", 3.14159);
            ps.printf("PS Formatted string: '%s'%n", "PrintStream");

            // Test format method
            ps.format("PS Format method: %s = %d%n", "answer", 42);

            // Test with explicit locale
            ps.format(Locale.US, "PS US format: %,d%n", 1234567);

            System.out.println("Written formatted PrintStream content");
        }

        // Read back formatted content
        try (FileInputStream fis = new FileInputStream(formatFile);
             InputStreamReader isr = new InputStreamReader(fis);
             BufferedReader br = new BufferedReader(isr)) {

            System.out.println("PrintStream formatted content:");
            String line;
            while ((line = br.readLine()) != null) {
                System.out.println("  " + line);
            }
        }
    }

    private static void testAutoFlush() throws IOException {
        System.out.println("--- Auto Flush Tests ---");

        File flushFile = new File(TEST_DIR, "flush_test.txt");

        // Test PrintWriter without auto-flush
        try (FileWriter fw = new FileWriter(flushFile);
             PrintWriter pw = new PrintWriter(fw, false)) { // No auto-flush

            pw.print("No auto-flush line 1");
            pw.println();
            pw.print("No auto-flush line 2");

            // Check if content is written before explicit flush
            System.out.println("File size before flush: " + flushFile.length());

            pw.flush();
            System.out.println("File size after flush: " + flushFile.length());
        }

        // Test PrintWriter with auto-flush
        try (FileWriter fw = new FileWriter(flushFile, true);
             PrintWriter pw = new PrintWriter(fw, true)) { // Auto-flush

            long sizeBefore = flushFile.length();
            pw.println("Auto-flush line");
            long sizeAfter = flushFile.length();

            System.out.println("Size before auto-flush line: " + sizeBefore);
            System.out.println("Size after auto-flush line: " + sizeAfter);
            System.out.println("Auto-flush worked: " + (sizeAfter > sizeBefore));
        }

        // Test PrintStream auto-flush
        File psFlushFile = new File(TEST_DIR, "ps_flush_test.txt");
        try (FileOutputStream fos = new FileOutputStream(psFlushFile);
             PrintStream ps = new PrintStream(fos, true)) { // Auto-flush

            ps.print("PrintStream auto-flush test");
            System.out.println("PrintStream file size after print: " + psFlushFile.length());

            ps.println(); // This should trigger auto-flush
            System.out.println("PrintStream file size after println: " + psFlushFile.length());
        }
    }

    private static void testErrorHandling() throws IOException {
        System.out.println("--- Error Handling Tests ---");

        // Test error state handling
        File errorFile = new File(TEST_DIR, "error_test.txt");

        // Create a PrintWriter and close the underlying stream
        FileWriter fw = new FileWriter(errorFile);
        PrintWriter pw = new PrintWriter(fw);

        fw.close(); // Close underlying stream

        // Now operations on PrintWriter should set error state
        pw.println("This should cause an error");
        pw.print("Another operation");

        boolean hasError = pw.checkError();
        System.out.println("PrintWriter has error after closed stream: " + hasError);

        pw.close(); // Clean up

        // Test PrintStream error handling
        FileOutputStream fos = new FileOutputStream(errorFile);
        PrintStream ps = new PrintStream(fos);

        fos.close(); // Close underlying stream

        ps.println("PrintStream error test");
        boolean psHasError = ps.checkError();
        System.out.println("PrintStream has error after closed stream: " + psHasError);

        ps.close(); // Clean up
    }

    private static void testCharsetHandling() throws IOException {
        System.out.println("--- Charset Handling Tests ---");

        File charsetFile = new File(TEST_DIR, "charset_test.txt");

        // Test PrintStream with explicit charset
        try (FileOutputStream fos = new FileOutputStream(charsetFile);
             PrintStream ps = new PrintStream(fos, true, "UTF-8")) {

            // Write Unicode content
            ps.println("ASCII: Hello World");
            ps.println("Latin: café, résumé, naïve");
            ps.println("Greek: α, β, γ, δ");
            ps.println("Math: ∑, ∏, ∆, ∞");
            ps.println("Emoji: 😀, 🌟, 🚀");

            System.out.println("Written Unicode content with UTF-8");
        }

        // Read back with explicit charset
        try (FileInputStream fis = new FileInputStream(charsetFile);
             InputStreamReader isr = new InputStreamReader(fis, "UTF-8");
             BufferedReader br = new BufferedReader(isr)) {

            System.out.println("Unicode content read back:");
            String line;
            while ((line = br.readLine()) != null) {
                System.out.println("  " + line);
            }
        }

        System.out.println("File size with Unicode: " + charsetFile.length() + " bytes");

        // Test different operations with PrintWriter on different streams
        ByteArrayOutputStream baos = new ByteArrayOutputStream();
        try (PrintWriter pw = new PrintWriter(baos, true)) {
            pw.println("ByteArrayOutputStream test");
            pw.printf("Formatted: %d%n", 123);
        }

        String result = baos.toString();
        System.out.println("ByteArrayOutputStream result: '" + result.trim() + "'");
        System.out.println("ByteArrayOutputStream size: " + baos.size() + " bytes");
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
