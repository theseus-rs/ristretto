import java.util.zip.Inflater;
import java.util.zip.Deflater;
import java.util.zip.DataFormatException;
import java.nio.ByteBuffer;

public class Test {
    public static void main(String[] args) {
        testBasic();
        testByteBuffer();
        testDictionary();
        testNowrap();
        testRemaining();
        testReset();
        System.out.println("Test completed successfully");
    }

    private static void testBasic() {
        System.out.println("Testing Inflater basic functionality");

        // First compress some data
        String original = "Hello World Hello World Hello World";
        byte[] inputBytes = original.getBytes();

        Deflater deflater = new Deflater();
        deflater.setInput(inputBytes);
        deflater.finish();

        byte[] compressed = new byte[100];
        int compressedLength = deflater.deflate(compressed);
        deflater.end();

        System.out.println("Original length: " + inputBytes.length);
        System.out.println("Compressed length: " + compressedLength);

        // Now decompress
        Inflater inflater = new Inflater();
        inflater.setInput(compressed, 0, compressedLength);

        byte[] decompressed = new byte[100];
        try {
            int decompressedLength = inflater.inflate(decompressed);
            System.out.println("Decompressed length: " + decompressedLength);

            String result = new String(decompressed, 0, decompressedLength);
            System.out.println("Decompressed text: " + result);
            System.out.println("Matches original: " + original.equals(result));

            System.out.println("Bytes read: " + inflater.getBytesRead());
            System.out.println("Bytes written: " + inflater.getBytesWritten());
            System.out.println("Finished: " + inflater.finished());
            System.out.println("Needs input: " + inflater.needsInput());
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }

        inflater.end();
        System.out.println("Inflater ended");
    }

    private static void testByteBuffer() {
        System.out.println("Testing Inflater with ByteBuffer");

        // First compress some data
        String original = "Hello World Hello World Hello World";
        byte[] inputBytes = original.getBytes();

        Deflater deflater = new Deflater();
        deflater.setInput(inputBytes);
        deflater.finish();

        byte[] compressed = new byte[100];
        int compressedLength = deflater.deflate(compressed);
        deflater.end();

        // Test with heap ByteBuffer input
        Inflater inflater = new Inflater();
        ByteBuffer heapInput = ByteBuffer.wrap(compressed, 0, compressedLength);
        inflater.setInput(heapInput);

        try {
            byte[] decompressed = new byte[100];
            int len = inflater.inflate(decompressed);
            String result = new String(decompressed, 0, len);
            System.out.println("Heap input decompressed: " + result.length() + " chars");
            System.out.println("Matches: " + original.equals(result));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
        inflater.end();

        // Test with ByteBuffer output
        inflater = new Inflater();
        inflater.setInput(compressed, 0, compressedLength);

        try {
            ByteBuffer outputBuffer = ByteBuffer.allocate(100);
            int len = inflater.inflate(outputBuffer);
            outputBuffer.flip();
            byte[] resultBytes = new byte[outputBuffer.remaining()];
            outputBuffer.get(resultBytes);
            String result = new String(resultBytes);
            System.out.println("ByteBuffer output decompressed: " + result.length() + " chars");
            System.out.println("Matches: " + original.equals(result));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
        inflater.end();
    }

    private static void testDictionary() {
        System.out.println("Testing Inflater with dictionary");

        // Create a dictionary
        String dictionary = "Hello World test string";
        byte[] dictBytes = dictionary.getBytes();

        String original = "Hello World test string Hello World";
        byte[] inputBytes = original.getBytes();

        // Compress with dictionary
        Deflater deflater = new Deflater();
        deflater.setDictionary(dictBytes);
        deflater.setInput(inputBytes);
        deflater.finish();

        byte[] compressed = new byte[100];
        int compressedLength = deflater.deflate(compressed);
        deflater.end();

        System.out.println("Compressed with dictionary: " + compressedLength + " bytes");

        // Decompress with dictionary
        Inflater inflater = new Inflater();
        inflater.setInput(compressed, 0, compressedLength);

        byte[] decompressed = new byte[100];
        try {
            int len = inflater.inflate(decompressed);
            if (len == 0 && inflater.needsDictionary()) {
                System.out.println("Inflater needs dictionary");
                inflater.setDictionary(dictBytes);
                len = inflater.inflate(decompressed);
            }

            String result = new String(decompressed, 0, len);
            System.out.println("Decompressed: " + result);
            System.out.println("Matches original: " + original.equals(result));
        } catch (DataFormatException e) {
            System.out.println("DataFormatException: " + e.getMessage());
        }

        inflater.end();
    }

    private static void testNowrap() {
        System.out.println("Testing Inflater nowrap mode");

        String original = "Hello World Hello World Hello World";
        byte[] inputBytes = original.getBytes();

        // Compress with nowrap
        Deflater deflater = new Deflater(Deflater.DEFAULT_COMPRESSION, true);
        deflater.setInput(inputBytes);
        deflater.finish();

        byte[] compressed = new byte[100];
        int compressedLength = deflater.deflate(compressed);
        deflater.end();

        System.out.println("Original length: " + inputBytes.length);
        System.out.println("Compressed length (nowrap): " + compressedLength);

        // Decompress with nowrap
        Inflater inflater = new Inflater(true);
        inflater.setInput(compressed, 0, compressedLength);

        byte[] decompressed = new byte[100];
        try {
            int decompressedLength = inflater.inflate(decompressed);
            System.out.println("Decompressed length: " + decompressedLength);

            String result = new String(decompressed, 0, decompressedLength);
            System.out.println("Decompressed text: " + result);
            System.out.println("Matches original: " + original.equals(result));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }

        inflater.end();
    }

    private static void testRemaining() {
        System.out.println("Testing Inflater getRemaining");

        // Create some compressed data
        String original = "Test data for compression";
        byte[] compressed = compress(original.getBytes());

        // Create a buffer with extra data after compressed data
        byte[] bufferWithExtra = new byte[compressed.length + 10];
        System.arraycopy(compressed, 0, bufferWithExtra, 0, compressed.length);
        // Fill extra bytes
        for (int i = compressed.length; i < bufferWithExtra.length; i++) {
            bufferWithExtra[i] = (byte) i;
        }

        Inflater inflater = new Inflater();
        inflater.setInput(bufferWithExtra);

        System.out.println("Before inflate:");
        System.out.println("Remaining: " + inflater.getRemaining());

        byte[] decompressed = new byte[100];
        try {
            int len = inflater.inflate(decompressed);
            String result = new String(decompressed, 0, len);
            System.out.println("Decompressed: " + result);
            System.out.println("After inflate:");
            System.out.println("Remaining: " + inflater.getRemaining());
            System.out.println("Finished: " + inflater.finished());
        } catch (DataFormatException e) {
            System.out.println("Error: " + e.getMessage());
        }

        inflater.end();
    }

    private static void testReset() {
        System.out.println("Testing Inflater reset functionality");

        String original1 = "First test string for compression";
        String original2 = "Second different test string";

        // Compress first string
        byte[] compressed1 = compress(original1.getBytes());
        // Compress second string
        byte[] compressed2 = compress(original2.getBytes());

        Inflater inflater = new Inflater();

        // Decompress first string
        try {
            inflater.setInput(compressed1);
            byte[] result1 = new byte[100];
            int len1 = inflater.inflate(result1);
            String decompressed1 = new String(result1, 0, len1);
            System.out.println("First decompression: " + decompressed1);
            System.out.println("Matches: " + original1.equals(decompressed1));

            // Reset and decompress second string
            inflater.reset();
            inflater.setInput(compressed2);
            byte[] result2 = new byte[100];
            int len2 = inflater.inflate(result2);
            String decompressed2 = new String(result2, 0, len2);
            System.out.println("Second decompression: " + decompressed2);
            System.out.println("Matches: " + original2.equals(decompressed2));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }

        inflater.end();
    }

    private static byte[] compress(byte[] input) {
        Deflater deflater = new Deflater();
        deflater.setInput(input);
        deflater.finish();
        byte[] output = new byte[100];
        int len = deflater.deflate(output);
        deflater.end();
        byte[] result = new byte[len];
        System.arraycopy(output, 0, result, 0, len);
        return result;
    }
}
