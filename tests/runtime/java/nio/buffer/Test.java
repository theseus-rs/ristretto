import java.nio.*;
import java.io.File;
import java.util.Arrays;

/**
 * Tests for java.nio.Buffer and its subclasses
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Java NIO Buffer Tests ===");

        testByteBuffer();
        testCharBuffer();
        testIntBuffer();
        testLongBuffer();
        testFloatBuffer();
        testDoubleBuffer();
        testShortBuffer();
        testBufferProperties();
        testBufferSlicing();
        testBufferDuplication();
        testDirectBuffers();

        System.out.println("=== Buffer Tests Complete ===");
    }

    private static void testByteBuffer() {
        System.out.println("--- ByteBuffer Tests ---");

        // Test allocation
        ByteBuffer buffer = ByteBuffer.allocate(10);
        System.out.println("ByteBuffer capacity: " + buffer.capacity());
        System.out.println("Initial position: " + buffer.position());
        System.out.println("Initial limit: " + buffer.limit());
        System.out.println("Limit equals capacity: " + (buffer.limit() == buffer.capacity()));

        // Test put/get operations
        buffer.put((byte) 42);
        System.out.println("Position after put: " + buffer.position());
        buffer.flip();
        byte getValue = buffer.get();
        System.out.println("Get returns: " + getValue);

        // Test array operations
        byte[] data = {1, 2, 3, 4, 5};
        buffer.clear();
        buffer.put(data);
        System.out.println("Position after put array: " + buffer.position());

        buffer.flip();
        byte[] result = new byte[5];
        buffer.get(result);
        System.out.println("Array get/put consistency: " + Arrays.equals(data, result));

        // Test wrap
        ByteBuffer wrapped = ByteBuffer.wrap(data);
        System.out.println("Wrapped buffer capacity: " + wrapped.capacity());
        System.out.println("Wrapped buffer position: " + wrapped.position());

        // Test order
        buffer.clear();
        buffer.order(ByteOrder.LITTLE_ENDIAN);
        System.out.println("Little endian order: " + buffer.order());
        buffer.order(ByteOrder.BIG_ENDIAN);
        System.out.println("Big endian order: " + buffer.order());

        // Test typed access
        buffer.clear();
        buffer.putInt(0x12345678);
        buffer.flip();
        int intValue = buffer.getInt();
        System.out.println("Int put/get: " + Integer.toHexString(intValue));
    }

    private static void testCharBuffer() {
        System.out.println("--- CharBuffer Tests ---");

        CharBuffer buffer = CharBuffer.allocate(10);
        System.out.println("CharBuffer capacity: " + buffer.capacity());

        String text = "Hello";
        buffer.put(text);
        System.out.println("Position after put string: " + buffer.position());

        buffer.flip();
        StringBuilder sb = new StringBuilder();
        while (buffer.hasRemaining()) {
            sb.append(buffer.get());
        }
        System.out.println("String reconstruction: '" + sb.toString() + "'");

        // Test wrap
        CharBuffer wrapped = CharBuffer.wrap("World");
        System.out.println("Wrapped char buffer: '" + wrapped.toString() + "'");

        // Test subSequence
        CharSequence sub = wrapped.subSequence(1, 4);
        System.out.println("SubSequence: '" + sub.toString() + "'");
    }

    private static void testIntBuffer() {
        System.out.println("--- IntBuffer Tests ---");

        IntBuffer buffer = IntBuffer.allocate(5);
        int[] data = {1, 2, 3, 4, 5};

        buffer.put(data);
        System.out.println("IntBuffer position after put: " + buffer.position());

        buffer.flip();
        int[] result = new int[5];
        buffer.get(result);
        System.out.println("IntBuffer array consistency: " + Arrays.equals(data, result));

        // Test wrap
        IntBuffer wrapped = IntBuffer.wrap(data);
        System.out.println("IntBuffer wrap capacity: " + wrapped.capacity());
    }

    private static void testLongBuffer() {
        System.out.println("--- LongBuffer Tests ---");

        LongBuffer buffer = LongBuffer.allocate(3);
        long[] data = {Long.MAX_VALUE, 0L, Long.MIN_VALUE};

        buffer.put(data);
        buffer.flip();

        System.out.println("Long MAX_VALUE: " + buffer.get());
        System.out.println("Long 0: " + buffer.get());
        System.out.println("Long MIN_VALUE: " + buffer.get());
    }

    private static void testFloatBuffer() {
        System.out.println("--- FloatBuffer Tests ---");

        FloatBuffer buffer = FloatBuffer.allocate(3);
        float[] data = {3.14f, Float.NaN, Float.POSITIVE_INFINITY};

        buffer.put(data);
        buffer.flip();

        float pi = buffer.get();
        System.out.println("Float pi: " + pi);
        float nan = buffer.get();
        System.out.println("Float NaN: " + Float.isNaN(nan));
        float inf = buffer.get();
        System.out.println("Float infinity: " + Float.isInfinite(inf));
    }

    private static void testDoubleBuffer() {
        System.out.println("--- DoubleBuffer Tests ---");

        DoubleBuffer buffer = DoubleBuffer.allocate(3);
        double[] data = {Math.PI, Double.NaN, Double.NEGATIVE_INFINITY};

        buffer.put(data);
        buffer.flip();

        double pi = buffer.get();
        System.out.println("Double pi: " + pi);
        double nan = buffer.get();
        System.out.println("Double NaN: " + Double.isNaN(nan));
        double negInf = buffer.get();
        System.out.println("Double negative infinity: " + (Double.isInfinite(negInf) && negInf < 0));
    }

    private static void testShortBuffer() {
        System.out.println("--- ShortBuffer Tests ---");

        ShortBuffer buffer = ShortBuffer.allocate(3);
        short[] data = {Short.MAX_VALUE, 0, Short.MIN_VALUE};

        buffer.put(data);
        buffer.flip();

        System.out.println("Short MAX_VALUE: " + buffer.get());
        System.out.println("Short 0: " + buffer.get());
        System.out.println("Short MIN_VALUE: " + buffer.get());
    }

    private static void testBufferProperties() {
        System.out.println("--- Buffer Properties Tests ---");

        ByteBuffer buffer = ByteBuffer.allocate(10);

        // Test mark and reset
        buffer.position(3);
        buffer.mark();
        buffer.position(7);
        buffer.reset();
        System.out.println("Position after mark and reset: " + buffer.position());

        // Test rewind
        buffer.position(5);
        buffer.rewind();
        System.out.println("Position after rewind: " + buffer.position());

        // Test clear
        buffer.position(3);
        buffer.limit(7);
        buffer.clear();
        System.out.println("Position after clear: " + buffer.position());
        System.out.println("Limit after clear: " + buffer.limit());
        System.out.println("Capacity: " + buffer.capacity());

        // Test flip
        buffer.position(5);
        buffer.flip();
        System.out.println("Limit after flip: " + buffer.limit());
        System.out.println("Position after flip: " + buffer.position());

        // Test remaining
        buffer.limit(8);
        buffer.position(3);
        System.out.println("Remaining: " + buffer.remaining());
        System.out.println("HasRemaining: " + buffer.hasRemaining());

        buffer.position(8);
        System.out.println("HasRemaining at end: " + buffer.hasRemaining());
    }

    private static void testBufferSlicing() {
        System.out.println("--- Buffer Slicing Tests ---");

        ByteBuffer original = ByteBuffer.allocate(10);
        for (int i = 0; i < 10; i++) {
            original.put((byte) i);
        }

        original.position(3);
        original.limit(7);
        ByteBuffer slice = original.slice();

        System.out.println("Original position: " + original.position());
        System.out.println("Original limit: " + original.limit());
        System.out.println("Slice capacity: " + slice.capacity());
        System.out.println("Slice position: " + slice.position());
        System.out.println("Slice limit: " + slice.limit());

        // Test that slice shares data with original
        slice.put(0, (byte) 99);
        System.out.println("Original data after slice modification: " + original.get(3));
    }

    private static void testBufferDuplication() {
        System.out.println("--- Buffer Duplication Tests ---");

        ByteBuffer original = ByteBuffer.allocate(10);
        original.put((byte) 42);
        original.position(5);
        original.limit(8);

        ByteBuffer duplicate = original.duplicate();

        System.out.println("Original capacity: " + original.capacity());
        System.out.println("Duplicate capacity: " + duplicate.capacity());
        System.out.println("Original position: " + original.position());
        System.out.println("Duplicate position: " + duplicate.position());
        System.out.println("Original limit: " + original.limit());
        System.out.println("Duplicate limit: " + duplicate.limit());

        // Test independence of position/limit
        duplicate.position(2);
        System.out.println("Original position after duplicate change: " + original.position());

        // Test shared data
        duplicate.put(0, (byte) 99);
        System.out.println("Original data after duplicate modification: " + original.get(0));
    }

    private static void testDirectBuffers() {
        System.out.println("--- Direct Buffer Tests ---");

        ByteBuffer direct = ByteBuffer.allocateDirect(100);
        System.out.println("Direct buffer is direct: " + direct.isDirect());
        System.out.println("Direct buffer capacity: " + direct.capacity());

        // Test basic operations on direct buffer
        direct.putInt(42);
        direct.flip();
        System.out.println("Direct buffer int value: " + direct.getInt());

        // Test that regular buffer is not direct
        ByteBuffer heap = ByteBuffer.allocate(100);
        System.out.println("Heap buffer is direct: " + heap.isDirect());
    }
}
