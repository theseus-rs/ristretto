import java.io.File;
import java.io.IOException;
import java.io.RandomAccessFile;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.nio.charset.StandardCharsets;

/** Tests memory-mapped file I/O via FileChannel.map(READ_WRITE). */
public class Test {
    public static void main(String[] args) throws IOException {
        File file = new File("mapped_buffer_test.bin");
        byte[] initial = "Hello, mapped world!".getBytes(StandardCharsets.UTF_8);

        try (RandomAccessFile raf = new RandomAccessFile(file, "rw")) {
            raf.write(initial);
        }

        try (RandomAccessFile raf = new RandomAccessFile(file, "rw");
             FileChannel channel = raf.getChannel()) {
            MappedByteBuffer buffer = channel.map(FileChannel.MapMode.READ_WRITE, 0, initial.length);
            System.out.println("capacity: " + buffer.capacity());
            System.out.println("isLoaded: " + buffer.isLoaded());

            buffer.load();
            System.out.println("loaded");

            byte[] readBack = new byte[initial.length];
            buffer.get(readBack);
            System.out.println("contents: " + new String(readBack, StandardCharsets.UTF_8));

            buffer.position(0);
            buffer.put((byte) 'h');
            buffer.force();
        }

        try (RandomAccessFile raf = new RandomAccessFile(file, "r")) {
            byte[] data = new byte[initial.length];
            raf.readFully(data);
            System.out.println("after force: " + new String(data, StandardCharsets.UTF_8));
        }

        if (file.delete()) {
            System.out.println("deleted");
        } else {
            System.out.println("delete failed");
        }
    }
}
