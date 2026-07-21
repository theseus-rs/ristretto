import java.net.InetSocketAddress;
import java.net.StandardProtocolFamily;
import java.nio.ByteBuffer;
import java.nio.channels.Pipe;
import java.nio.channels.SelectionKey;
import java.nio.channels.Selector;
import java.nio.channels.ServerSocketChannel;
import java.nio.channels.SocketChannel;
import java.nio.charset.StandardCharsets;
import java.util.Iterator;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicReference;

public class Test {
    @FunctionalInterface
    private interface CheckedRunnable {
        void run() throws Exception;
    }

    private static void runStep(String name, CheckedRunnable action) throws Exception {
        AtomicReference<Throwable> failure = new AtomicReference<>();
        Thread worker = new Thread(() -> {
            try {
                action.run();
            } catch (Throwable error) {
                failure.set(error);
            }
        }, "nio-selector-" + name);
        worker.setDaemon(true);
        worker.start();
        worker.join(10_000);
        if (worker.isAlive()) {
            throw new AssertionError(name + " timed out");
        }
        Throwable error = failure.get();
        if (error instanceof Exception exception) {
            throw exception;
        }
        if (error instanceof Error fatal) {
            throw fatal;
        }
        if (error != null) {
            throw new AssertionError(name + " failed", error);
        }
    }

    private static SelectionKey selected(Selector selector, int operation) {
        Iterator<SelectionKey> iterator = selector.selectedKeys().iterator();
        while (iterator.hasNext()) {
            SelectionKey key = iterator.next();
            iterator.remove();
            if ((key.readyOps() & operation) != 0) {
                return key;
            }
        }
        throw new AssertionError("missing ready operation " + operation);
    }

    private static void pipeReadiness() throws Exception {
        Pipe pipe = Pipe.open();
        try (Selector selector = Selector.open();
                Pipe.SourceChannel source = pipe.source();
                Pipe.SinkChannel sink = pipe.sink()) {
            pipe.source().configureBlocking(false);
            SelectionKey key = pipe.source().register(selector, SelectionKey.OP_READ);
            int emptyRead = pipe.source().read(ByteBuffer.allocate(1));
            pipe.sink().write(ByteBuffer.wrap("pipe".getBytes(StandardCharsets.UTF_8)));
            int count = selector.select(5000);
            selected(selector, SelectionKey.OP_READ);
            ByteBuffer buffer = ByteBuffer.allocate(4);
            int read = pipe.source().read(buffer);
            buffer.flip();
            System.out.println("Pipe selected: " + (count > 0));
            System.out.println("Pipe nonblocking empty read: " + emptyRead);
            System.out.println("Pipe payload: " + StandardCharsets.UTF_8.decode(buffer));
            key.cancel();
            selector.selectNow();
            System.out.println("Pipe key cancelled: " + !key.isValid());
        }
    }

    private static void socketReadiness() throws Exception {
        try (Selector selector = Selector.open();
                ServerSocketChannel server = ServerSocketChannel.open()) {
            server.configureBlocking(false);
            server.bind(new InetSocketAddress("127.0.0.1", 0));
            server.register(selector, SelectionKey.OP_ACCEPT);
            int port = ((InetSocketAddress) server.getLocalAddress()).getPort();
            try (SocketChannel client = SocketChannel.open()) {
                client.connect(new InetSocketAddress("127.0.0.1", port));
                int acceptedCount = selector.select(5000);
                selected(selector, SelectionKey.OP_ACCEPT);
                try (SocketChannel accepted = server.accept()) {
                    accepted.configureBlocking(false);
                    int emptyRead = accepted.read(ByteBuffer.allocate(0));
                    accepted.register(selector, SelectionKey.OP_READ);
                    client.write(ByteBuffer.wrap("socket".getBytes(StandardCharsets.UTF_8)));
                    int readableCount = selector.select(5000);
                    selected(selector, SelectionKey.OP_READ);
                    ByteBuffer buffer = ByteBuffer.allocate(6);
                    int read = accepted.read(buffer);
                    buffer.flip();
                    System.out.println("Accept selected: " + (acceptedCount > 0));
                    System.out.println("Read selected: " + (readableCount > 0));
                    System.out.println("Socket bytes: " + read);
                    System.out.println("Socket zero-length read: " + emptyRead);
                    System.out.println("Socket payload: " + StandardCharsets.UTF_8.decode(buffer));
                }
            }
        }
    }

    private static void wakeup() throws Exception {
        try (Selector selector = Selector.open()) {
            selector.wakeup();
            System.out.println("Wakeup call returned: true");
            long start = System.nanoTime();
            int selected = selector.select(5000);
            long elapsedMillis = (System.nanoTime() - start) / 1_000_000;
            System.out.println("Wakeup selected: " + selected);
            System.out.println("Wakeup prompt: " + (elapsedMillis < 1000));
        }
    }

    private static void ipv6Socket() throws Exception {
        try (ServerSocketChannel server = ServerSocketChannel.open(StandardProtocolFamily.INET6)) {
            server.bind(new InetSocketAddress("::1", 0));
            int port = ((InetSocketAddress) server.getLocalAddress()).getPort();
            try (SocketChannel client = SocketChannel.open(StandardProtocolFamily.INET6)) {
                client.connect(new InetSocketAddress("::1", port));
                try (SocketChannel accepted = server.accept()) {
                    client.write(ByteBuffer.wrap("v6".getBytes(StandardCharsets.UTF_8)));
                    ByteBuffer payload = ByteBuffer.allocate(2);
                    while (payload.hasRemaining()) {
                        if (accepted.read(payload) < 0) {
                            throw new AssertionError("unexpected IPv6 EOF");
                        }
                    }
                    payload.flip();
                    InetSocketAddress remote = (InetSocketAddress) accepted.getRemoteAddress();
                    System.out.println("IPv6 remote address: "
                            + remote.getAddress().getHostAddress().contains(":"));
                    System.out.println("IPv6 payload: " + StandardCharsets.UTF_8.decode(payload));
                }
            }
        }
    }

    private static void blockingClose() throws Exception {
        try (ServerSocketChannel server = ServerSocketChannel.open()) {
            server.bind(new InetSocketAddress("127.0.0.1", 0));
            int port = ((InetSocketAddress) server.getLocalAddress()).getPort();
            try (SocketChannel client = SocketChannel.open(new InetSocketAddress("127.0.0.1", port));
                    SocketChannel accepted = server.accept()) {
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
                reader.setDaemon(true);
                reader.start();
                if (!started.await(5, TimeUnit.SECONDS)) {
                    throw new AssertionError("blocking read did not start");
                }
                Thread.sleep(100);
                accepted.close();
                reader.join(5000);
                if (reader.isAlive()) {
                    throw new AssertionError("blocking read was not released by close");
                }
                System.out.println("Blocking read close: " + result.get());
            }
        }

        ServerSocketChannel server = ServerSocketChannel.open();
        server.bind(new InetSocketAddress("127.0.0.1", 0));
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
        accepter.setDaemon(true);
        accepter.start();
        if (!started.await(5, TimeUnit.SECONDS)) {
            throw new AssertionError("blocking accept did not start");
        }
        Thread.sleep(100);
        server.close();
        accepter.join(5000);
        if (accepter.isAlive()) {
            throw new AssertionError("blocking accept was not released by close");
        }
        System.out.println("Blocking accept close: " + result.get());
    }

    public static void main(String[] args) throws Exception {
        runStep("pipe readiness", Test::pipeReadiness);
        runStep("socket readiness", Test::socketReadiness);
        runStep("selector wakeup", Test::wakeup);
        runStep("IPv6 socket", Test::ipv6Socket);
        runStep("blocking close", Test::blockingClose);
    }
}
