import java.util.zip.CRC32;
import java.nio.ByteBuffer;

public class Test {
    public static void main(String[] args) {
        testBasic();
        testByteBuffer();
        testIncremental();
        testKnownValues();
        System.out.println("Test completed successfully");
    }

    private static void testBasic() {
        System.out.println("Testing CRC32 basic functionality");

        // Test default constructor
        CRC32 crc = new CRC32();
        System.out.println("Initial value: " + crc.getValue());

        // Test update with single byte
        crc.reset();
        crc.update(65); // 'A'
        System.out.println("After update(65): " + crc.getValue());

        // Test update with byte array
        crc.reset();
        byte[] data = "Hello World".getBytes();
        crc.update(data);
        System.out.println("After update(\"Hello World\"): " + crc.getValue());

        // Test update with byte array slice
        crc.reset();
        crc.update(data, 0, 5); // "Hello"
        System.out.println("After update(\"Hello\"): " + crc.getValue());

        // Test reset
        crc.reset();
        System.out.println("After reset: " + crc.getValue());

        // Test known CRC32 value for empty string
        crc.reset();
        crc.update(new byte[0]);
        System.out.println("Empty byte array CRC: " + crc.getValue());
    }

    private static void testByteBuffer() {
        System.out.println("Testing CRC32 with ByteBuffer");

        CRC32 crc = new CRC32();

        // Test with heap ByteBuffer
        ByteBuffer heapBuffer = ByteBuffer.wrap("Test Data".getBytes());
        crc.update(heapBuffer);
        System.out.println("Heap ByteBuffer CRC: " + crc.getValue());

        // Test with partial buffer
        crc.reset();
        ByteBuffer partialBuffer = ByteBuffer.wrap("Hello World".getBytes());
        partialBuffer.position(0);
        partialBuffer.limit(5);
        crc.update(partialBuffer);
        System.out.println("Partial buffer CRC: " + crc.getValue());

        // Test buffer position after update
        ByteBuffer testBuffer = ByteBuffer.wrap("ABC".getBytes());
        int posBefore = testBuffer.position();
        crc.reset();
        crc.update(testBuffer);
        int posAfter = testBuffer.position();
        System.out.println("Position before: " + posBefore + ", after: " + posAfter);
    }

    private static void testIncremental() {
        System.out.println("Testing CRC32 with multiple updates");

        CRC32 crc = new CRC32();

        // Test incremental updates produce same result as single update
        crc.reset();
        crc.update("Hello".getBytes());
        crc.update(" ".getBytes());
        crc.update("World".getBytes());
        long incrementalCrc = crc.getValue();
        System.out.println("Incremental CRC: " + incrementalCrc);

        crc.reset();
        crc.update("Hello World".getBytes());
        long singleCrc = crc.getValue();
        System.out.println("Single CRC: " + singleCrc);

        System.out.println("Incremental equals single: " + (incrementalCrc == singleCrc));

        // Test byte-by-byte update
        crc.reset();
        String testString = "ABC";
        for (byte b : testString.getBytes()) {
            crc.update(b);
        }
        long byteByByteCrc = crc.getValue();
        System.out.println("Byte-by-byte CRC: " + byteByByteCrc);

        crc.reset();
        crc.update(testString.getBytes());
        long fullCrc = crc.getValue();
        System.out.println("Full array CRC: " + fullCrc);

        System.out.println("Byte-by-byte equals full: " + (byteByByteCrc == fullCrc));
    }

    private static void testKnownValues() {
        System.out.println("Testing CRC32 known values");

        CRC32 crc = new CRC32();

        // Initial value should be 0
        System.out.println("Initial value: " + crc.getValue());

        // Known test vectors
        crc.reset();
        crc.update("a".getBytes());
        System.out.println("CRC32(\"a\"): " + crc.getValue());

        crc.reset();
        crc.update("abc".getBytes());
        System.out.println("CRC32(\"abc\"): " + crc.getValue());

        crc.reset();
        crc.update("message digest".getBytes());
        System.out.println("CRC32(\"message digest\"): " + crc.getValue());

        crc.reset();
        crc.update("abcdefghijklmnopqrstuvwxyz".getBytes());
        System.out.println("CRC32(\"abcdefghijklmnopqrstuvwxyz\"): " + crc.getValue());

        // "123456789" - well known CRC32 test vector
        crc.reset();
        crc.update("123456789".getBytes());
        System.out.println("CRC32(\"123456789\"): " + crc.getValue());

        // All zeros
        crc.reset();
        byte[] zeros = new byte[100];
        crc.update(zeros);
        System.out.println("CRC32(100 zeros): " + crc.getValue());

        // All 0xFF
        crc.reset();
        byte[] ones = new byte[100];
        for (int i = 0; i < ones.length; i++) {
            ones[i] = (byte) 0xFF;
        }
        crc.update(ones);
        System.out.println("CRC32(100 x 0xFF): " + crc.getValue());
    }
}
