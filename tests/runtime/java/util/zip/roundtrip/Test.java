import java.util.zip.Deflater;
import java.util.zip.Inflater;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing Deflater and Inflater roundtrip");

        // Test various input sizes
        testRoundtrip("Short");
        testRoundtrip("Medium length string for testing");
        testRoundtrip(generateLargeString(1000));

        // Test with repeating content (high compression)
        testRoundtrip("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

        // Test with random-like content (low compression)
        testRoundtrip("aZ1bY2cX3dW4eV5fU6gT7hS8iR9jQ0");

        System.out.println("Test completed successfully");
    }

    private static void testRoundtrip(String original) {
        byte[] inputBytes = original.getBytes();

        // Compress
        Deflater deflater = new Deflater();
        deflater.setInput(inputBytes);
        deflater.finish();

        byte[] compressed = new byte[inputBytes.length + 100];
        int compressedLength = deflater.deflate(compressed);
        deflater.end();

        // Decompress
        Inflater inflater = new Inflater();
        inflater.setInput(compressed, 0, compressedLength);

        byte[] decompressed = new byte[inputBytes.length + 100];
        try {
            int decompressedLength = inflater.inflate(decompressed);
            String result = new String(decompressed, 0, decompressedLength);

            System.out.println("Input length: " + inputBytes.length +
                ", Compressed: " + (compressedLength > 0) +
                ", Roundtrip match: " + original.equals(result));
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
        inflater.end();
    }

    private static String generateLargeString(int length) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < length; i++) {
            sb.append((char) ('A' + (i % 26)));
        }
        return sb.toString();
    }
}
