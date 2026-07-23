import java.net.InetSocketAddress;
import java.net.StandardProtocolFamily;
import java.nio.ByteBuffer;
import java.nio.channels.DatagramChannel;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicReference;

public class Test {
    private static InetSocketAddress loopback() {
        // Use a fresh wildcard-port address for every bind.
        return new InetSocketAddress("127.0.0.1", 0);
    }

    private static String decode(ByteBuffer... buffers) {
        StringBuilder result = new StringBuilder();
        for (ByteBuffer buffer : buffers) {
            buffer.flip();
            result.append(StandardCharsets.UTF_8.decode(buffer));
        }
        return result.toString();
    }

    public static void main(String[] args) throws Exception {
        try (DatagramChannel receiver = DatagramChannel.open(StandardProtocolFamily.INET);
                DatagramChannel sender = DatagramChannel.open(StandardProtocolFamily.INET);
                DatagramChannel left = DatagramChannel.open(StandardProtocolFamily.INET);
                DatagramChannel right = DatagramChannel.open(StandardProtocolFamily.INET);
                DatagramChannel blockingReceiver =
                        DatagramChannel.open(StandardProtocolFamily.INET)) {
            // Allocate every endpoint before closing any of them. Darwin can briefly retain a
            // recently closed UDP ephemeral binding, which makes an immediate bind(0) fail.
            receiver.bind(loopback());
            left.bind(loopback());
            right.bind(loopback());
            blockingReceiver.bind(loopback());

            receiver.configureBlocking(false);
            boolean unavailable = receiver.receive(ByteBuffer.allocateDirect(8)) == null;
            receiver.configureBlocking(true);

            InetSocketAddress target = (InetSocketAddress) receiver.getLocalAddress();
            ByteBuffer direct = ByteBuffer.allocateDirect(6);
            direct.put("direct".getBytes(StandardCharsets.UTF_8)).flip();
            int sent = sender.send(direct, target);
            ByteBuffer received = ByteBuffer.allocateDirect(6);
            InetSocketAddress source = (InetSocketAddress) receiver.receive(received);

            int emptySent = sender.send(ByteBuffer.allocateDirect(0), target);
            ByteBuffer emptyReceived = ByteBuffer.allocateDirect(1);
            InetSocketAddress emptySource = (InetSocketAddress) receiver.receive(emptyReceived);

            sender.send(ByteBuffer.wrap("truncate".getBytes(StandardCharsets.UTF_8)), target);
            ByteBuffer truncated = ByteBuffer.allocateDirect(3);
            InetSocketAddress truncatedSource = (InetSocketAddress) receiver.receive(truncated);

            System.out.println("NIO datagram unavailable: " + unavailable);
            System.out.println("NIO datagram sent: " + sent);
            System.out.println("NIO datagram source: " + source.getAddress().isLoopbackAddress());
            System.out.println("NIO datagram payload: " + decode(received));
            System.out.println("NIO empty datagram sent: " + emptySent);
            System.out.println("NIO empty datagram received: "
                    + (emptySource != null && emptyReceived.position() == 0));
            System.out.println("NIO datagram truncated source: " + (truncatedSource != null));
            System.out.println("NIO datagram truncated payload: " + decode(truncated));

            left.connect(right.getLocalAddress());
            right.connect(left.getLocalAddress());

            ByteBuffer[] outgoing = {
                ByteBuffer.wrap("ga".getBytes(StandardCharsets.UTF_8)),
                ByteBuffer.wrap("ther".getBytes(StandardCharsets.UTF_8))
            };
            long gathered = left.write(outgoing);
            ByteBuffer first = ByteBuffer.allocate(2);
            ByteBuffer second = ByteBuffer.allocateDirect(4);
            long scattered = right.read(new ByteBuffer[] { first, second });

            left.write(ByteBuffer.wrap("oversize".getBytes(StandardCharsets.UTF_8)));
            ByteBuffer connectedTruncated = ByteBuffer.allocateDirect(3);
            int connectedTruncatedCount = right.read(connectedTruncated);
            left.disconnect();

            System.out.println("NIO datagram gathered: " + gathered);
            System.out.println("NIO datagram scattered: " + scattered);
            System.out.println("NIO datagram gathered payload: " + decode(first, second));
            System.out.println("NIO connected datagram truncated bytes: " + connectedTruncatedCount);
            System.out.println("NIO connected datagram truncated payload: " + decode(connectedTruncated));
            System.out.println("NIO datagram disconnected: " + !left.isConnected());

            CountDownLatch started = new CountDownLatch(1);
            AtomicReference<String> result = new AtomicReference<>("completed");
            Thread reader = new Thread(() -> {
                started.countDown();
                try {
                    blockingReceiver.receive(ByteBuffer.allocateDirect(1));
                } catch (Exception error) {
                    result.set(error.getClass().getSimpleName());
                }
            });
            reader.start();
            if (!started.await(5, TimeUnit.SECONDS)) {
                throw new AssertionError("blocking datagram receive did not start");
            }
            Thread.sleep(100);
            blockingReceiver.close();
            reader.join(5000);
            if (reader.isAlive()) {
                throw new AssertionError("blocking datagram receive was not released by close");
            }
            System.out.println("NIO datagram blocking close: " + result.get());
        }
    }
}
