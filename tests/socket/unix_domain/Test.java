import java.net.StandardProtocolFamily;
import java.net.UnixDomainSocketAddress;
import java.nio.ByteBuffer;
import java.nio.channels.SelectionKey;
import java.nio.channels.Selector;
import java.nio.channels.ServerSocketChannel;
import java.nio.channels.SocketChannel;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicReference;

public class Test {
    private static void blockingClose(Path readSocket, Path acceptSocket) throws Exception {
        try (ServerSocketChannel server = ServerSocketChannel.open(StandardProtocolFamily.UNIX)) {
            server.bind(UnixDomainSocketAddress.of(readSocket));
            try (SocketChannel client = SocketChannel.open(StandardProtocolFamily.UNIX)) {
                client.connect(server.getLocalAddress());
                try (SocketChannel accepted = server.accept()) {
                    CountDownLatch started = new CountDownLatch(1);
                    AtomicReference<String> result = new AtomicReference<>("completed");
                    Thread reader = new Thread(() -> {
                        started.countDown();
                        try {
                            accepted.read(ByteBuffer.allocate(1));
                        } catch (Exception error) {
                            result.set(error.getClass().getSimpleName());
                        }
                    });
                    reader.start();
                    if (!started.await(5, TimeUnit.SECONDS)) {
                        throw new AssertionError("Unix blocking read did not start");
                    }
                    Thread.sleep(100);
                    accepted.close();
                    reader.join(5000);
                    if (reader.isAlive()) {
                        throw new AssertionError("Unix blocking read was not released by close");
                    }
                    System.out.println("Unix blocking read close: " + result.get());
                }
            }
        }

        ServerSocketChannel server = ServerSocketChannel.open(StandardProtocolFamily.UNIX);
        server.bind(UnixDomainSocketAddress.of(acceptSocket));
        CountDownLatch started = new CountDownLatch(1);
        AtomicReference<String> result = new AtomicReference<>("completed");
        Thread accepter = new Thread(() -> {
            started.countDown();
            try {
                server.accept();
            } catch (Exception error) {
                result.set(error.getClass().getSimpleName());
            }
        });
        accepter.start();
        if (!started.await(5, TimeUnit.SECONDS)) {
            throw new AssertionError("Unix blocking accept did not start");
        }
        Thread.sleep(100);
        server.close();
        accepter.join(5000);
        if (accepter.isAlive()) {
            throw new AssertionError("Unix blocking accept was not released by close");
        }
        System.out.println("Unix blocking accept close: " + result.get());
    }

    public static void main(String[] args) throws Exception {
        // sockaddr_un paths are limited to 104 bytes on macOS. The default temporary
        // directory there can already consume most of that allowance, so prefer the
        // short, system-wide temporary directory when it is available.
        Path shortTempRoot = Path.of("/tmp");
        Path directory = Files.isDirectory(shortTempRoot)
                ? Files.createTempDirectory(shortTempRoot, "ruds-")
                : Files.createTempDirectory("ruds-");
        Path socketFile = directory.resolve("c");
        Path readSocket = directory.resolve("r");
        Path acceptSocket = directory.resolve("a");
        UnixDomainSocketAddress address = UnixDomainSocketAddress.of(socketFile);
        try (Selector selector = Selector.open();
                ServerSocketChannel server = ServerSocketChannel.open(StandardProtocolFamily.UNIX)) {
            server.configureBlocking(false);
            server.bind(address);
            server.register(selector, SelectionKey.OP_ACCEPT);
            try (SocketChannel client = SocketChannel.open(StandardProtocolFamily.UNIX)) {
                boolean connected = client.connect(address);
                int selected = selector.select(5000);
                try (SocketChannel accepted = server.accept()) {
                    accepted.configureBlocking(false);
                    int unavailable = accepted.read(ByteBuffer.allocate(1));
                    accepted.configureBlocking(true);
                    client.write(ByteBuffer.wrap("unix".getBytes(StandardCharsets.UTF_8)));
                    ByteBuffer buffer = ByteBuffer.allocate(4);
                    while (buffer.hasRemaining()) {
                        int count = accepted.read(buffer);
                        if (count < 0) {
                            throw new AssertionError("unexpected EOF");
                        }
                    }
                    buffer.flip();
                    System.out.println("Unix connected: " + connected);
                    System.out.println("Unix accept selected: " + (selected > 0));
                    System.out.println("Unix local address: "
                            + (server.getLocalAddress() instanceof UnixDomainSocketAddress));
                    System.out.println("Unix nonblocking empty read: " + unavailable);
                    System.out.println("Unix payload: " + StandardCharsets.UTF_8.decode(buffer));
                }
            }
            blockingClose(readSocket, acceptSocket);
        } finally {
            Files.deleteIfExists(socketFile);
            Files.deleteIfExists(readSocket);
            Files.deleteIfExists(acceptSocket);
            Files.deleteIfExists(directory);
        }
    }
}
