import java.lang.reflect.Field;
import sun.misc.Unsafe;

/** Integration tests for the VM's native-memory allocator. */
public class Test {
    private static final Unsafe UNSAFE = unsafe();

    public static void main(String[] args) {
        testAlignmentAndAccess();
        testOversizedAllocation();
    }

    private static void testAlignmentAndAccess() {
        int[] sizes = {1, 3, 7, 15, 40, 127};

        for (int size : sizes) {
            long address = UNSAFE.allocateMemory(size);
            try {
                System.out.println("size " + size + " aligned: " + ((address & 15) == 0));

                UNSAFE.putByte(address, (byte) size);
                UNSAFE.putByte(address + size - 1, (byte) (size + 1));
                System.out.println(
                    "size " + size + " values: "
                        + UNSAFE.getByte(address) + ","
                        + UNSAFE.getByte(address + size - 1));
            } finally {
                UNSAFE.freeMemory(address);
            }
        }
    }

    private static void testOversizedAllocation() {
        boolean failed = false;
        long oversizedAddress = 0;
        try {
            oversizedAddress = UNSAFE.allocateMemory(1L << 62);
        } catch (OutOfMemoryError error) {
            failed = true;
        } finally {
            if (oversizedAddress != 0) {
                UNSAFE.freeMemory(oversizedAddress);
            }
        }
        System.out.println("oversized allocation failed: " + failed);

        long address = UNSAFE.allocateMemory(Long.BYTES);
        try {
            UNSAFE.putLong(address, 0x0102_0304_0506_0708L);
            System.out.println("allocation after failure works: "
                + (UNSAFE.getLong(address) == 0x0102_0304_0506_0708L));
        } finally {
            UNSAFE.freeMemory(address);
        }
    }

    private static Unsafe unsafe() {
        try {
            Field field = Unsafe.class.getDeclaredField("theUnsafe");
            field.setAccessible(true);
            return (Unsafe) field.get(null);
        } catch (ReflectiveOperationException error) {
            throw new AssertionError(error);
        }
    }
}
