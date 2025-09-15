import java.io.*;

/**
 * Tests for java.io.PipedInputStream, PipedOutputStream, CharArrayReader, and CharArrayWriter classes
 */
public class Test {
    private static final String TEST_DIR = "test_piped_and_char_array_streams";

    public static void main(String[] args) {
        System.out.println("=== PipedStreams and CharArrayStreams Tests ===");

        try {
            // Setup
            File testDir = new File(TEST_DIR);
            cleanup(testDir);
            testDir.mkdir();

            testPipedStreamsBasic();
            testPipedStreamsThreaded();
            testCharArrayReaderWriter();
            testCharArrayOperations();
            testExceptionHandling();
        } catch (Exception e) {
            System.out.println("ERROR: " + e.getMessage());
            e.printStackTrace();
        } finally {
            cleanup(new File(TEST_DIR));
        }

        System.out.println("=== PipedStreams and CharArrayStreams Tests Complete ===");
    }

    private static void testPipedStreamsBasic() throws IOException {
        System.out.println("--- Piped Streams Basic Tests ---");

        // Test PipedInputStream and PipedOutputStream
        PipedInputStream pis = new PipedInputStream();
        PipedOutputStream pos = new PipedOutputStream(pis);

        // Write some data
        byte[] testData = "Hello, Piped Streams!".getBytes();
        pos.write(testData);
        pos.flush();

        System.out.println("Written to piped output stream: " + testData.length + " bytes");

        // Read the data
        byte[] buffer = new byte[testData.length];
        int bytesRead = pis.read(buffer);
        String result = new String(buffer, 0, bytesRead);

        System.out.println("Read from piped input stream: '" + result + "'");
        System.out.println("Data matches: " + "Hello, Piped Streams!".equals(result));

        // Test available
        pos.write("More data".getBytes());
        pos.flush();
        System.out.println("Available bytes: " + pis.available());

        // Read remaining
        buffer = new byte[20];
        bytesRead = pis.read(buffer);
        String moreResult = new String(buffer, 0, bytesRead);
        System.out.println("Read more data: '" + moreResult + "'");

        pos.close();
        pis.close();
    }

    private static void testPipedStreamsThreaded() throws IOException, InterruptedException {
        System.out.println("--- Piped Streams Threaded Tests ---");

        final PipedInputStream pis = new PipedInputStream();
        final PipedOutputStream pos = new PipedOutputStream(pis);

        final String[] messages = {
            "Message 1: Hello from writer thread",
            "Message 2: This is a test",
            "Message 3: Piped streams work across threads",
            "Message 4: Final message"
        };

        // Writer thread
        Thread writerThread = new Thread(new Runnable() {
            public void run() {
                try {
                    System.out.println("Writer thread started");
                    for (String message : messages) {
                        pos.write(message.getBytes());
                        pos.write('\n');
                        pos.flush();
                        System.out.println("Wrote: " + message);
                        Thread.sleep(100); // Small delay
                    }
                    pos.close();
                    System.out.println("Writer thread finished");
                } catch (Exception e) {
                    System.out.println("Writer thread error: " + e.getMessage());
                }
            }
        });

        // Reader thread
        Thread readerThread = new Thread(new Runnable() {
            public void run() {
                try {
                    System.out.println("Reader thread started");
                    BufferedReader br = new BufferedReader(new InputStreamReader(pis));
                    String line;
                    int messageCount = 0;
                    while ((line = br.readLine()) != null) {
                        System.out.println("Read: " + line);
                        messageCount++;
                    }
                    System.out.println("Reader thread finished, read " + messageCount + " messages");
                    br.close();
                } catch (Exception e) {
                    System.out.println("Reader thread error: " + e.getMessage());
                }
            }
        });

        // Start both threads
        writerThread.start();
        readerThread.start();

        // Wait for completion
        writerThread.join(5000); // 5 second timeout
        readerThread.join(5000);

        System.out.println("Threaded piped streams test completed");
    }

    private static void testCharArrayReaderWriter() throws IOException {
        System.out.println("--- CharArrayReader/Writer Tests ---");

        // Test CharArrayWriter
        CharArrayWriter caw = new CharArrayWriter();

        caw.write("Hello");
        caw.write(' ');
        caw.write("CharArrayWriter");
        caw.write('!');
        caw.write('\n');

        // Write character array
        char[] chars = {'T', 'e', 's', 't'};
        caw.write(chars);
        caw.write('\n');

        // Write partial character array
        char[] moreChars = {'P', 'a', 'r', 't', 'i', 'a', 'l'};
        caw.write(moreChars, 1, 3); // Write "art"
        caw.write('\n');

        // Write string
        caw.write("String content");
        caw.write('\n');

        System.out.println("CharArrayWriter size: " + caw.size());
        System.out.println("CharArrayWriter content:");
        System.out.println(caw.toString());

        // Get the character array
        char[] result = caw.toCharArray();
        System.out.println("Character array length: " + result.length);

        // Test CharArrayReader
        CharArrayReader car = new CharArrayReader(result);

        System.out.println("CharArrayReader ready: " + car.ready());
        System.out.println("Mark supported: " + car.markSupported());

        // Read character by character
        System.out.print("First 10 characters: ");
        for (int i = 0; i < 10; i++) {
            int c = car.read();
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
        int charsRead = car.read(buffer);
        String remaining = new String(buffer, 0, charsRead);
        System.out.println("Remaining content: '" + remaining.replace("\n", "\\n") + "'");

        caw.close();
        car.close();
    }

    private static void testCharArrayOperations() throws IOException {
        System.out.println("--- CharArray Operations Tests ---");

        // Test CharArrayWriter with initial capacity
        CharArrayWriter caw = new CharArrayWriter(16);
        System.out.println("Initial capacity: 16");

        // Write data to test growth
        for (int i = 0; i < 30; i++) {
            caw.write('A' + (i % 26));
        }

        System.out.println("After writing 30 chars, size: " + caw.size());

        // Test reset
        caw.reset();
        System.out.println("After reset, size: " + caw.size());

        // Write new content after reset
        caw.write("Content after reset");
        System.out.println("New content: '" + caw.toString() + "'");

        // Test writeTo method
        File outputFile = new File(TEST_DIR, "char_array_output.txt");
        try (FileWriter fw = new FileWriter(outputFile)) {
            caw.writeTo(fw);
            System.out.println("Written to file using writeTo");
        }

        // Verify file content
        try (FileReader fr = new FileReader(outputFile)) {
            char[] fileBuffer = new char[1024];
            int fileCharsRead = fr.read(fileBuffer);
            String fileContent = new String(fileBuffer, 0, fileCharsRead);
            System.out.println("File content: '" + fileContent + "'");
            System.out.println("File content matches: " + caw.toString().equals(fileContent));
        }

        // Test CharArrayReader with offset and length
        char[] testArray = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".toCharArray();
        CharArrayReader car = new CharArrayReader(testArray, 10, 15); // Start at 'A', read 15 chars

        char[] subBuffer = new char[20];
        int subCharsRead = car.read(subBuffer);
        String subContent = new String(subBuffer, 0, subCharsRead);
        System.out.println("Subset read: '" + subContent + "'");
        System.out.println("Expected subset: 'ABCDEFGHIJKLMNO'");
        System.out.println("Subset correct: " + "ABCDEFGHIJKLMNO".equals(subContent));

        caw.close();
        car.close();
    }

    private static void testExceptionHandling() {
        System.out.println("--- Exception Handling Tests ---");

        // Test PipedStreams not connected
        try {
            PipedInputStream pis = new PipedInputStream();
            // Don't connect to any output stream
            pis.read();
            System.out.println("ERROR: Should have thrown IOException for unconnected pipe");
        } catch (IOException e) {
            System.out.println("Correctly caught IOException for unconnected pipe: " + e.getMessage());
        }

        // Test writing to closed PipedOutputStream
        try {
            PipedInputStream pis = new PipedInputStream();
            PipedOutputStream pos = new PipedOutputStream(pis);
            pos.close();
            pos.write(65);
            System.out.println("ERROR: Should have thrown IOException for closed pipe");
        } catch (IOException e) {
            System.out.println("Correctly caught IOException for closed pipe: " + e.getMessage());
        }

        // Test CharArrayReader with null array
        try {
            new CharArrayReader(null);
            System.out.println("ERROR: Should have thrown exception for null array");
        } catch (NullPointerException e) {
            System.out.println("Correctly caught NullPointerException for null array");
        }

        // Test CharArrayReader with invalid bounds
        try {
            char[] testArray = "test".toCharArray();
            new CharArrayReader(testArray, -1, 2);
            System.out.println("ERROR: Should have thrown exception for negative offset");
        } catch (IllegalArgumentException e) {
            System.out.println("Correctly caught IllegalArgumentException for negative offset");
        }

        try {
            char[] testArray = "test".toCharArray();
            new CharArrayReader(testArray, 0, testArray.length + 1);
            System.out.println("ERROR: Should have thrown exception for length > array length");
        } catch (IllegalArgumentException e) {
            System.out.println("Correctly caught IllegalArgumentException for invalid length");
        }

        // Test operations on closed streams
        try {
            CharArrayWriter caw = new CharArrayWriter();
            caw.close();
            caw.write("test"); // Should still work for CharArrayWriter
            System.out.println("Write after close works for CharArrayWriter");
        } catch (Exception e) {
            System.out.println("CharArrayWriter close behavior: " + e.getClass().getSimpleName());
        }

        try {
            CharArrayReader car = new CharArrayReader("test".toCharArray());
            car.close();
            car.read(); // Should still work for CharArrayReader
            System.out.println("Read after close works for CharArrayReader");
        } catch (Exception e) {
            System.out.println("CharArrayReader close behavior: " + e.getClass().getSimpleName());
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
