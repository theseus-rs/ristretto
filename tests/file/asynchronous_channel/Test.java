import java.nio.ByteBuffer;
import java.nio.channels.AsynchronousFileChannel;
import java.nio.channels.CompletionHandler;
import java.nio.channels.FileLock;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

/** Exercises the Windows overlapped file natives through the public asynchronous-channel API. */
public class Test {
    private static final long TIMEOUT_SECONDS = 10;

    public static void main(String[] args) throws Exception {
        System.out.println("=== Asynchronous File Channel Test ===");
        Path path = Files.createTempFile("ristretto-async-file", ".tmp");

        try (AsynchronousFileChannel channel = AsynchronousFileChannel.open(
                path, StandardOpenOption.READ, StandardOpenOption.WRITE)) {
            ByteBuffer first = ByteBuffer.wrap("hello".getBytes(StandardCharsets.UTF_8));
            int written = channel.write(first, 0).get(TIMEOUT_SECONDS, TimeUnit.SECONDS);
            System.out.println("Future write bytes: " + written);

            ByteBuffer read = ByteBuffer.allocate(5);
            int readCount = channel.read(read, 0).get(TIMEOUT_SECONDS, TimeUnit.SECONDS);
            read.flip();
            System.out.println("Future read: " + StandardCharsets.UTF_8.decode(read));
            System.out.println("Future read bytes: " + readCount);

            CountDownLatch writeDone = new CountDownLatch(1);
            String[] callbackResult = new String[1];
            channel.write(
                    ByteBuffer.wrap("!".getBytes(StandardCharsets.UTF_8)),
                    5,
                    "write",
                    new CompletionHandler<Integer, String>() {
                        @Override
                        public void completed(Integer result, String attachment) {
                            callbackResult[0] = attachment + ":" + result;
                            writeDone.countDown();
                        }

                        @Override
                        public void failed(Throwable error, String attachment) {
                            callbackResult[0] = attachment + ":" + error.getClass().getSimpleName();
                            writeDone.countDown();
                        }
                    });
            System.out.println("Callback completed: " + writeDone.await(TIMEOUT_SECONDS, TimeUnit.SECONDS));
            System.out.println("Callback result: " + callbackResult[0]);

            try (FileLock lock = channel.lock(0, 1, false).get(TIMEOUT_SECONDS, TimeUnit.SECONDS)) {
                System.out.println("Exclusive lock valid: " + lock.isValid());
                System.out.println("Exclusive lock shared: " + lock.isShared());
                System.out.println("Exclusive lock range: " + lock.position() + ":" + lock.size());
            }

            try (FileLock lock = channel.lock(2, 3, true).get(TIMEOUT_SECONDS, TimeUnit.SECONDS)) {
                System.out.println("Shared lock valid: " + lock.isValid());
                System.out.println("Shared lock shared: " + lock.isShared());
                System.out.println("Shared lock range: " + lock.position() + ":" + lock.size());
            }

            ByteBuffer eof = ByteBuffer.allocate(1);
            int eofResult = channel.read(eof, 1024).get(TIMEOUT_SECONDS, TimeUnit.SECONDS);
            System.out.println("EOF result: " + eofResult);
            System.out.println("Final size: " + channel.size());
        } finally {
            Files.deleteIfExists(path);
        }
        System.out.println("=== Asynchronous File Channel Test Complete ===");
    }
}
