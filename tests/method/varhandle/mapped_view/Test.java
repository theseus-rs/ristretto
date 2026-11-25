import java.io.IOException;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.nio.ByteOrder;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;
import java.util.Arrays;

public class Test {
    static void mappingModes(FileChannel channel, Path path, byte[] original) throws IOException {
        VarHandle word = MethodHandles.byteBufferViewVarHandle(int[].class, ByteOrder.BIG_ENDIAN);
        int initial = 0x01020304;
        MappedByteBuffer readOnly = channel.map(FileChannel.MapMode.READ_ONLY, 0, original.length);
        if ((int) word.getAcquire(readOnly, 4) != initial) throw new AssertionError("read-only load");
        try {
            word.compareAndSet(readOnly, 4, initial, 0);
            throw new AssertionError("read-only CAS succeeded");
        } catch (java.nio.ReadOnlyBufferException expected) {}
        MappedByteBuffer privateMap = channel.map(FileChannel.MapMode.PRIVATE, 0, original.length);
        int previous = (int) word.getAndSet(privateMap, 4, 37);
        if (previous != initial || (int) word.get(privateMap.duplicate(), 4) != 37) {
            throw new AssertionError("private mapping witness or alias");
        }
        privateMap.force();
        if ((int) word.get(readOnly, 4) != initial || !Arrays.equals(original, Files.readAllBytes(path))) {
            throw new AssertionError("private mapping modified the file");
        }
    }

    public static void main(String[] args) throws IOException {
        Path path = Files.createTempFile("varhandle-view-", ".bin");
        try {
            byte[] expected = {9, 8, 7, 6, 1, 2, 3, 4, 5};
            Files.write(path, expected);
            MappedByteBuffer mapped;
            try (FileChannel channel = FileChannel.open(path, StandardOpenOption.READ, StandardOpenOption.WRITE)) {
                mappingModes(channel, path, expected);
                mapped = channel.map(FileChannel.MapMode.READ_WRITE, 0, expected.length);
                VarHandle word = MethodHandles.byteBufferViewVarHandle(int[].class, ByteOrder.BIG_ENDIAN);
                int old = (int) word.getAndAdd(mapped.duplicate(), 4, 1);
                if (old != 0x01020304 || mapped.getInt(4) != 0x01020305) throw new AssertionError("mapped exchange");
                mapped.force();
                expected[7] = 5;
                if (!Arrays.equals(expected, Files.readAllBytes(path))) throw new AssertionError("mapped file contents");
            }
            mapped.put(0, (byte) 42);
            mapped.force();
            expected[0] = 42;
            if (!Arrays.equals(expected, Files.readAllBytes(path))) throw new AssertionError("mapping after channel close");
        } finally {
            // Windows can keep the mapping live after the channel closes.
            if (!path.toFile().delete()) path.toFile().deleteOnExit();
        }
        System.out.println("Mapped VarHandle view passed");
    }
}
