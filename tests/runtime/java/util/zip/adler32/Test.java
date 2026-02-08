import java.util.zip.Adler32;
import java.nio.ByteBuffer;

public class Test {
    public static void main(String[] args) {
        testBasic();
        testByteBuffer();
        testKnownValues();
        System.out.println("Test completed successfully");
    }

    private static void testBasic() {
        System.out.println("Testing Adler32 basic functionality");

        // Test default constructor
        Adler32 adler = new Adler32();
        System.out.println("Initial value: " + adler.getValue());

        // Test update with single byte
        adler.reset();
        adler.update(65); // 'A'
        System.out.println("After update(65): " + adler.getValue());

        // Test update with byte array
        adler.reset();
        byte[] data = "Hello World".getBytes();
        adler.update(data);
        System.out.println("After update(\"Hello World\"): " + adler.getValue());

        // Test update with byte array slice
        adler.reset();
        adler.update(data, 0, 5); // "Hello"
        System.out.println("After update(\"Hello\"): " + adler.getValue());

        // Test reset
        adler.reset();
        System.out.println("After reset: " + adler.getValue());
    }

    private static void testByteBuffer() {
        System.out.println("Testing Adler32 with ByteBuffer");

        Adler32 adler = new Adler32();

        // Test with heap ByteBuffer
        ByteBuffer heapBuffer = ByteBuffer.wrap("Test Data".getBytes());
        adler.update(heapBuffer);
        System.out.println("Heap ByteBuffer checksum: " + adler.getValue());

        // Test with partial buffer
        adler.reset();
        ByteBuffer partialBuffer = ByteBuffer.wrap("Hello World".getBytes());
        partialBuffer.position(0);
        partialBuffer.limit(5);
        adler.update(partialBuffer);
        System.out.println("Partial buffer checksum: " + adler.getValue());

        // Test buffer position after update
        ByteBuffer testBuffer = ByteBuffer.wrap("ABC".getBytes());
        int posBefore = testBuffer.position();
        adler.reset();
        adler.update(testBuffer);
        int posAfter = testBuffer.position();
        System.out.println("Position before: " + posBefore + ", after: " + posAfter);
    }

    private static void testKnownValues() {
        System.out.println("Testing Adler32 known values");

        Adler32 adler = new Adler32();

        // Initial value should be 1
        System.out.println("Initial value: " + adler.getValue());

        // Known test vectors
        adler.reset();
        adler.update("a".getBytes());
        System.out.println("Adler32(\"a\"): " + adler.getValue());

        adler.reset();
        adler.update("abc".getBytes());
        System.out.println("Adler32(\"abc\"): " + adler.getValue());

        adler.reset();
        adler.update("message digest".getBytes());
        System.out.println("Adler32(\"message digest\"): " + adler.getValue());

        adler.reset();
        adler.update("abcdefghijklmnopqrstuvwxyz".getBytes());
        System.out.println("Adler32(\"abcdefghijklmnopqrstuvwxyz\"): " + adler.getValue());

        // All zeros
        adler.reset();
        byte[] zeros = new byte[100];
        adler.update(zeros);
        System.out.println("Adler32(100 zeros): " + adler.getValue());

        // All 0xFF
        adler.reset();
        byte[] ones = new byte[100];
        for (int i = 0; i < ones.length; i++) {
            ones[i] = (byte) 0xFF;
        }
        adler.update(ones);
        System.out.println("Adler32(100 x 0xFF): " + adler.getValue());
    }
}
