import java.util.zip.Deflater;
import java.util.zip.Inflater;
import java.nio.ByteBuffer;

public class Test {
    public static void main(String[] args) {
        testBasic();
        testByteBuffer();
        testDictionary();
        testLevels();
        testNowrap();
        testReset();
        testStrategy();
        System.out.println("Test completed successfully");
    }

    private static void testBasic() {
        System.out.println("Testing Deflater basic functionality");

        // Test default constructor
        Deflater deflater = new Deflater();
        System.out.println("Deflater created with default level");

        // Test compression of simple data
        String input = "Hello World Hello World Hello World";
        byte[] inputBytes = input.getBytes();
        deflater.setInput(inputBytes);
        deflater.finish();

        byte[] output = new byte[100];
        int compressedLength = deflater.deflate(output);
        System.out.println("Input length: " + inputBytes.length);
        System.out.println("Compression achieved: " + (compressedLength > 0 && compressedLength < inputBytes.length));

        System.out.println("Bytes read: " + deflater.getBytesRead());
        System.out.println("Bytes written matches: " + (deflater.getBytesWritten() == compressedLength));
        System.out.println("Finished: " + deflater.finished());
        System.out.println("Needs input: " + deflater.needsInput());

        deflater.end();
        System.out.println("Deflater ended");
    }

    private static void testByteBuffer() {
        System.out.println("Testing Deflater with ByteBuffer");

        String input = "Hello World Hello World Hello World";
        byte[] inputBytes = input.getBytes();

        // Test with heap ByteBuffer input
        Deflater deflater = new Deflater();
        ByteBuffer heapInput = ByteBuffer.wrap(inputBytes);
        deflater.setInput(heapInput);
        deflater.finish();

        byte[] output = new byte[100];
        int compressedLength = deflater.deflate(output);
        System.out.println("Heap input compressed: " + (compressedLength > 0));
        System.out.println("Heap buffer position after setInput: " + heapInput.position());
        deflater.end();

        // Test with ByteBuffer output
        deflater = new Deflater();
        deflater.setInput(inputBytes);
        deflater.finish();

        ByteBuffer outputBuffer = ByteBuffer.allocate(100);
        int compressedLength3 = deflater.deflate(outputBuffer);
        System.out.println("ByteBuffer output compressed: " + (compressedLength3 > 0));
        System.out.println("Output buffer position matches: " + (outputBuffer.position() == compressedLength3));
        deflater.end();
    }

    private static void testDictionary() {
        System.out.println("Testing Deflater with dictionary");

        // Create a dictionary based on expected content
        String dictionary = "Hello World test string";
        byte[] dictBytes = dictionary.getBytes();

        String input = "Hello World test string Hello World";
        byte[] inputBytes = input.getBytes();

        // Compress with dictionary
        Deflater deflater = new Deflater();
        deflater.setDictionary(dictBytes);
        deflater.setInput(inputBytes);
        deflater.finish();

        byte[] compressed = new byte[100];
        int compressedLength = deflater.deflate(compressed);
        long adler = deflater.getAdler();
        System.out.println("Compressed with dictionary: " + (compressedLength > 0));
        System.out.println("Adler checksum: " + adler);
        deflater.end();

        // Compress without dictionary for comparison
        Deflater deflater2 = new Deflater();
        deflater2.setInput(inputBytes);
        deflater2.finish();

        byte[] compressed2 = new byte[100];
        int compressedLength2 = deflater2.deflate(compressed2);
        System.out.println("Compressed without dictionary: " + (compressedLength2 > 0));
        deflater2.end();

        // Dictionary should help with compression (or at least not hurt)
        System.out.println("Dictionary compression is smaller or equal: " + (compressedLength <= compressedLength2));
    }

    private static void testLevels() {
        System.out.println("Testing Deflater compression levels");

        String input = "This is a test string that will be compressed at different levels. Let's add more text to make compression more meaningful.";
        byte[] inputBytes = input.getBytes();

        // Test different compression levels and verify they work
        int[] levels = {
            Deflater.NO_COMPRESSION,
            Deflater.BEST_SPEED,
            Deflater.DEFAULT_COMPRESSION,
            Deflater.BEST_COMPRESSION
        };

        String[] levelNames = {
            "NO_COMPRESSION",
            "BEST_SPEED",
            "DEFAULT_COMPRESSION",
            "BEST_COMPRESSION"
        };

        int noCompressLen = 0;
        int bestCompressLen = 0;
        for (int i = 0; i < levels.length; i++) {
            Deflater deflater = new Deflater(levels[i]);
            deflater.setInput(inputBytes);
            deflater.finish();

            byte[] output = new byte[300];
            int compressedLength = deflater.deflate(output);

            System.out.println(levelNames[i] + " works: " + (compressedLength > 0));
            if (i == 0) noCompressLen = compressedLength;
            if (i == 3) bestCompressLen = compressedLength;
            deflater.end();
        }
        // Best compression should produce smaller or equal output than no compression
        System.out.println("Best compression <= no compression: " + (bestCompressLen <= noCompressLen));
    }

    private static void testNowrap() {
        System.out.println("Testing Deflater nowrap mode");

        String input = "Hello World Hello World Hello World";
        byte[] inputBytes = input.getBytes();

        // Test with default (with zlib header)
        Deflater deflaterWithHeader = new Deflater();
        deflaterWithHeader.setInput(inputBytes);
        deflaterWithHeader.finish();

        byte[] outputWithHeader = new byte[100];
        int lengthWithHeader = deflaterWithHeader.deflate(outputWithHeader);
        System.out.println("With header works: " + (lengthWithHeader > 0));
        deflaterWithHeader.end();

        // Test with nowrap (raw deflate, no header)
        Deflater deflaterNoWrap = new Deflater(Deflater.DEFAULT_COMPRESSION, true);
        deflaterNoWrap.setInput(inputBytes);
        deflaterNoWrap.finish();

        byte[] outputNoWrap = new byte[100];
        int lengthNoWrap = deflaterNoWrap.deflate(outputNoWrap);
        System.out.println("No wrap works: " + (lengthNoWrap > 0));
        deflaterNoWrap.end();

        // No wrap should be smaller (no zlib header overhead)
        System.out.println("No wrap is smaller: " + (lengthNoWrap < lengthWithHeader));
    }

    private static void testReset() {
        System.out.println("Testing Deflater reset functionality");

        Deflater deflater = new Deflater();

        // First compression
        String input1 = "First test string";
        deflater.setInput(input1.getBytes());
        deflater.finish();

        byte[] output1 = new byte[100];
        int len1 = deflater.deflate(output1);
        System.out.println("First compression works: " + (len1 > 0));
        System.out.println("Bytes read: " + deflater.getBytesRead());

        // Reset
        deflater.reset();
        System.out.println("After reset:");
        System.out.println("Bytes read: " + deflater.getBytesRead());
        System.out.println("Bytes written: " + deflater.getBytesWritten());
        System.out.println("Finished: " + deflater.finished());

        // Second compression
        String input2 = "Second test string different";
        deflater.setInput(input2.getBytes());
        deflater.finish();

        byte[] output2 = new byte[100];
        int len2 = deflater.deflate(output2);
        System.out.println("Second compression works: " + (len2 > 0));
        System.out.println("Bytes read: " + deflater.getBytesRead());

        deflater.end();
    }

    private static void testStrategy() {
        System.out.println("Testing Deflater strategies");

        String input = "This is a test string with some repeating patterns patterns patterns";
        byte[] inputBytes = input.getBytes();

        // Test different strategies
        testStrategyHelper(inputBytes, Deflater.DEFAULT_STRATEGY, "DEFAULT_STRATEGY");
        testStrategyHelper(inputBytes, Deflater.FILTERED, "FILTERED");
        testStrategyHelper(inputBytes, Deflater.HUFFMAN_ONLY, "HUFFMAN_ONLY");
    }

    private static void testStrategyHelper(byte[] input, int strategy, String name) {
        Deflater deflater = new Deflater();
        deflater.setStrategy(strategy);
        deflater.setInput(input);
        deflater.finish();

        // Use a loop to get all output (some strategies may need multiple calls)
        byte[] output = new byte[200];
        int totalCompressed = 0;
        int maxCalls = 10; // Prevent infinite loop
        int calls = 0;
        while (!deflater.finished() && calls < maxCalls) {
            int len = deflater.deflate(output);
            totalCompressed += len;
            calls++;
        }

        System.out.println(name + " works: " + (totalCompressed > 0));
        deflater.end();
    }
}
