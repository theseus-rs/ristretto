import java.io.File;
import java.io.IOException;
import java.io.RandomAccessFile;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.nio.charset.StandardCharsets;

/** Tests read-only memory mapping. */
public class Test {
    public static void main(String[] args) throws IOException {
        File file = new File("mapped_buffer_ro_test.bin");
        byte[] initial = "read-only-mapping-test".getBytes(StandardCharsets.UTF_8);

        try (RandomAccessFile raf = new RandomAccessFile(file, "rw")) {
            raf.write(initial);
        }

        try (RandomAccessFile raf = new RandomAccessFile(file, "r");
             FileChannel channel = raf.getChannel()) {
            MappedByteBuffer buffer = channel.map(FileChannel.MapMode.READ_ONLY, 0, initial.length);
            System.out.println("capacity: " + buffer.capacity());
            System.out.println("isReadOnly: " + buffer.isReadOnly());
            System.out.println("isLoaded: " + buffer.isLoaded());
            buffer.load();

            byte[] data = new byte[initial.length];
            buffer.get(data);
            System.out.println("contents: " + new String(data, StandardCharsets.UTF_8));

            try {
                buffer.put(0, (byte) 'X');
                System.out.println("put-succeeded-unexpectedly");
            } catch (java.nio.ReadOnlyBufferException e) {
                System.out.println("got ReadOnlyBufferException");
            }
        }

        if (file.delete()) {
            System.out.println("deleted");
        } else {
            System.out.println("delete failed");
        }
    }
}
