import java.net.InetSocketAddress;
import java.nio.ByteBuffer;
import java.nio.channels.AsynchronousServerSocketChannel;
import java.nio.channels.AsynchronousSocketChannel;
import java.nio.channels.CompletionHandler;
import java.nio.charset.StandardCharsets;
import java.util.Locale;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;

/** Exercises AcceptEx, ConnectEx, WSARecv, WSASend, and close through public APIs. */
public class Test {
    private static final long TIMEOUT_SECONDS = 10;

    private static int writeFully(AsynchronousSocketChannel channel, ByteBuffer buffer)
            throws Exception {
        int total = 0;
        while (buffer.hasRemaining()) {
            total += channel.write(buffer).get(TIMEOUT_SECONDS, TimeUnit.SECONDS);
        }
        return total;
    }

    private static String readFully(AsynchronousSocketChannel channel, int length)
            throws Exception {
        ByteBuffer buffer = ByteBuffer.allocate(length);
        while (buffer.hasRemaining()) {
            int count = channel.read(buffer).get(TIMEOUT_SECONDS, TimeUnit.SECONDS);
            if (count < 0) {
                break;
            }
        }
        buffer.flip();
        return StandardCharsets.UTF_8.decode(buffer).toString();
    }

    private static boolean hasRemaining(ByteBuffer[] buffers) {
        for (ByteBuffer buffer : buffers) {
            if (buffer.hasRemaining()) {
                return true;
            }
        }
        return false;
    }

    private static long gatheringWriteFully(
            AsynchronousSocketChannel channel, ByteBuffer[] buffers) throws Exception {
        long total = 0;
        while (hasRemaining(buffers)) {
            CompletableFuture<Long> completed = new CompletableFuture<>();
            channel.write(
                    buffers,
                    0,
                    buffers.length,
                    TIMEOUT_SECONDS,
                    TimeUnit.SECONDS,
                    null,
                    new CompletionHandler<Long, Void>() {
                        @Override
                        public void completed(Long result, Void attachment) {
                            completed.complete(result);
                        }

                        @Override
                        public void failed(Throwable error, Void attachment) {
                            completed.completeExceptionally(error);
                        }
                    });
            long count = completed.get(TIMEOUT_SECONDS + 1, TimeUnit.SECONDS);
            if (count <= 0) {
                throw new IllegalStateException("gathering write made no progress");
            }
            total += count;
        }
        return total;
    }

    private static long scatteringReadFully(
            AsynchronousSocketChannel channel, ByteBuffer[] buffers) throws Exception {
        long total = 0;
        while (hasRemaining(buffers)) {
            CompletableFuture<Long> completed = new CompletableFuture<>();
            channel.read(
                    buffers,
                    0,
                    buffers.length,
                    TIMEOUT_SECONDS,
                    TimeUnit.SECONDS,
                    null,
                    new CompletionHandler<Long, Void>() {
                        @Override
                        public void completed(Long result, Void attachment) {
                            completed.complete(result);
                        }

                        @Override
                        public void failed(Throwable error, Void attachment) {
                            completed.completeExceptionally(error);
                        }
                    });
            long count = completed.get(TIMEOUT_SECONDS + 1, TimeUnit.SECONDS);
            if (count < 0) {
                throw new IllegalStateException("unexpected EOF during scattering read");
            }
            total += count;
        }
        return total;
    }

    private static String decode(ByteBuffer[] buffers) {
        StringBuilder result = new StringBuilder();
        for (ByteBuffer buffer : buffers) {
            buffer.flip();
            result.append(StandardCharsets.UTF_8.decode(buffer));
        }
        return result.toString();
    }

    private static void testIpv6() throws Exception {
        try (AsynchronousServerSocketChannel server = AsynchronousServerSocketChannel.open()) {
            server.bind(new InetSocketAddress("::1", 0));
            int port = ((InetSocketAddress) server.getLocalAddress()).getPort();
            var acceptFuture = server.accept();
            try (AsynchronousSocketChannel client = AsynchronousSocketChannel.open()) {
                client.connect(new InetSocketAddress("::1", port))
                        .get(TIMEOUT_SECONDS, TimeUnit.SECONDS);
                try (AsynchronousSocketChannel accepted =
                        acceptFuture.get(TIMEOUT_SECONDS, TimeUnit.SECONDS)) {
                    writeFully(client, ByteBuffer.wrap("ipv6".getBytes(StandardCharsets.UTF_8)));
                    System.out.println("IPv6 loopback read: " + readFully(accepted, 4));
                }
            }
        }
    }

    private static void testPeerClose() throws Exception {
        try (AsynchronousServerSocketChannel server = AsynchronousServerSocketChannel.open()) {
            server.bind(new InetSocketAddress("127.0.0.1", 0));
            int port = ((InetSocketAddress) server.getLocalAddress()).getPort();
            var acceptFuture = server.accept();
            AsynchronousSocketChannel client = AsynchronousSocketChannel.open();
            client.connect(new InetSocketAddress("127.0.0.1", port))
                    .get(TIMEOUT_SECONDS, TimeUnit.SECONDS);
            try (AsynchronousSocketChannel accepted =
                    acceptFuture.get(TIMEOUT_SECONDS, TimeUnit.SECONDS)) {
                client.close();
                int eof = accepted.read(ByteBuffer.allocate(1))
                        .get(TIMEOUT_SECONDS, TimeUnit.SECONDS);
                System.out.println("Read after peer close: " + eof);
            }
        }
    }

    public static void main(String[] args) throws Exception {
        if (!System.getProperty("os.name", "").toLowerCase(Locale.ROOT).contains("windows")) {
            System.out.println("Asynchronous socket channel test skipped on non-Windows host");
            return;
        }

        System.out.println("=== Asynchronous Socket Channel Test ===");
        try (AsynchronousServerSocketChannel server = AsynchronousServerSocketChannel.open()) {
            server.bind(new InetSocketAddress("127.0.0.1", 0));
            int port = ((InetSocketAddress) server.getLocalAddress()).getPort();
            var acceptFuture = server.accept();

            try (AsynchronousSocketChannel client = AsynchronousSocketChannel.open()) {
                client.connect(new InetSocketAddress("127.0.0.1", port))
                        .get(TIMEOUT_SECONDS, TimeUnit.SECONDS);
                try (AsynchronousSocketChannel accepted =
                        acceptFuture.get(TIMEOUT_SECONDS, TimeUnit.SECONDS)) {
                    System.out.println("Client connected: " + client.isOpen());
                    System.out.println("Server accepted: " + accepted.isOpen());
                    System.out.println("Client remote loopback: "
                            + ((InetSocketAddress) client.getRemoteAddress()).getAddress().isLoopbackAddress());

                    int clientWritten = writeFully(
                            client, ByteBuffer.wrap("client-data".getBytes(StandardCharsets.UTF_8)));
                    System.out.println("Client write bytes: " + clientWritten);
                    System.out.println("Server read: " + readFully(accepted, clientWritten));

                    ByteBuffer[] gathered = {
                        ByteBuffer.allocate(0),
                        ByteBuffer.wrap("multi-".getBytes(StandardCharsets.UTF_8)),
                        ByteBuffer.wrap("buffer".getBytes(StandardCharsets.UTF_8))
                    };
                    long gatheredBytes = gatheringWriteFully(client, gathered);
                    ByteBuffer[] scattered = {
                        ByteBuffer.allocate(0), ByteBuffer.allocate(6), ByteBuffer.allocate(6)
                    };
                    long scatteredBytes = scatteringReadFully(accepted, scattered);
                    System.out.println("Gathering write bytes: " + gatheredBytes);
                    System.out.println("Scattering read bytes: " + scatteredBytes);
                    System.out.println("Scattering read: " + decode(scattered));

                    int serverWritten = writeFully(
                            accepted, ByteBuffer.wrap("server-reply".getBytes(StandardCharsets.UTF_8)));
                    System.out.println("Server write bytes: " + serverWritten);
                    System.out.println("Client read: " + readFully(client, serverWritten));

                    var pendingRead = client.read(ByteBuffer.allocate(1));
                    client.close();
                    try {
                        pendingRead.get(TIMEOUT_SECONDS, TimeUnit.SECONDS);
                        System.out.println("Pending read after close: completed");
                    } catch (ExecutionException error) {
                        System.out.println("Pending read after close: "
                                + error.getCause().getClass().getSimpleName());
                    }
                }
            }
        }

        try (AsynchronousServerSocketChannel server = AsynchronousServerSocketChannel.open()) {
            server.bind(new InetSocketAddress("127.0.0.1", 0));
            var pendingAccept = server.accept();
            server.close();
            try {
                pendingAccept.get(TIMEOUT_SECONDS, TimeUnit.SECONDS);
                System.out.println("Pending accept after close: completed");
            } catch (ExecutionException error) {
                System.out.println("Pending accept after close: "
                        + error.getCause().getClass().getSimpleName());
            }
        }

        testPeerClose();
        testIpv6();
        System.out.println("=== Asynchronous Socket Channel Test Complete ===");
    }
}
