import java.io.RandomAccessFile;
import java.nio.ByteBuffer;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.nio.channels.FileLock;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

public class Test {
    public static void main(String[] args) throws Exception {
        Path path = Files.createTempFile("ristretto-nio-", ".dat");
        boolean windows = System.getProperty("os.name").startsWith("Windows");
        if (windows) {
            // Windows keeps a mapped file locked until the process releases the mapping.
            // Defer cleanup until VM shutdown, after the channel and mapping are gone.
            path.toFile().deleteOnExit();
        }
        try (RandomAccessFile file = new RandomAccessFile(path.toFile(), "rw");
                FileChannel channel = file.getChannel()) {
            ByteBuffer[] writes = {
                ByteBuffer.wrap("abc".getBytes(StandardCharsets.UTF_8)),
                ByteBuffer.wrap("def".getBytes(StandardCharsets.UTF_8))
            };
            long written = channel.write(writes);
            channel.write(ByteBuffer.wrap("Z".getBytes(StandardCharsets.UTF_8)), 2);
            ByteBuffer first = ByteBuffer.allocate(2);
            ByteBuffer second = ByteBuffer.allocate(4);
            channel.position(0);
            long read = channel.read(new ByteBuffer[] { first, second });
            first.flip();
            second.flip();
            String payload = StandardCharsets.UTF_8.decode(first).toString()
                    + StandardCharsets.UTF_8.decode(second).toString();
            try (FileLock lock = channel.lock(1, 3, false)) {
                System.out.println("File lock range: " + lock.position() + ":" + lock.size());
            }
            try (FileLock firstRange = channel.lock(0, 1, false);
                    RandomAccessFile peerFile = new RandomAccessFile(path.toFile(), "rw");
                    FileChannel peerChannel = peerFile.getChannel();
                    FileLock disjointRange = peerChannel.tryLock(4, 1, false)) {
                System.out.println("File disjoint range lock: " + (disjointRange != null));
            }
            long positionBeforeMap = channel.position();
            if (windows) {
                // Windows does not permit truncating a file while a live mapped section
                // exists, so perform the same truncate operation before mapping there.
                channel.truncate(5);
            }
            MappedByteBuffer mapped = channel.map(FileChannel.MapMode.READ_WRITE, 0, channel.size());
            boolean mapPreservedPosition = channel.position() == positionBeforeMap;
            mapped.put(0, (byte) 'M');
            mapped.force();
            channel.force(true);
            if (!windows) {
                channel.truncate(5);
            }
            System.out.println("File gathered bytes: " + written);
            System.out.println("File scattered bytes: " + read);
            System.out.println("File payload: " + payload);
            System.out.println("File mapped byte: " + (char) mapped.get(0));
            System.out.println("File map preserved position: " + mapPreservedPosition);
            System.out.println("File truncated size: " + channel.size());
        } finally {
            if (!windows) {
                Files.deleteIfExists(path);
            }
        }
    }
}
