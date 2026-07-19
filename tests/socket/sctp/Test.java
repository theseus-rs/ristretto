import com.sun.nio.sctp.MessageInfo;
import com.sun.nio.sctp.SctpChannel;
import com.sun.nio.sctp.SctpServerChannel;
import com.sun.nio.sctp.SctpStandardSocketOptions;
import java.io.IOException;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.net.SocketAddress;
import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.atomic.AtomicReference;

/** Integration coverage for the Linux SCTP intrinsic implementation. */
public class Test {
    private static void check(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }

    private static String readMessage(SctpChannel channel, boolean direct,
                                      int expectedPpid, boolean expectedUnordered) throws Exception {
        ByteBuffer buffer = direct ? ByteBuffer.allocateDirect(128) : ByteBuffer.allocate(128);
        MessageInfo info = channel.receive(buffer, null, null);
        check(info != null, "receive returned null");
        check(info.bytes() > 0, "receive returned no payload");
        check(info.isComplete(), "message was incomplete");
        check(info.address() != null, "message source was missing");
        check(info.payloadProtocolID() == expectedPpid, "payload protocol ID");
        check(info.isUnordered() == expectedUnordered, "unordered flag");
        buffer.flip();
        byte[] bytes = new byte[buffer.remaining()];
        buffer.get(bytes);
        return new String(bytes, StandardCharsets.UTF_8);
    }

    private static void exerciseSctp() throws Exception {
        InetAddress loopback = InetAddress.getByName("127.0.0.1");
        AtomicReference<Throwable> serverFailure = new AtomicReference<>();

        try (SctpServerChannel server = SctpServerChannel.open()) {
            SctpStandardSocketOptions.InitMaxStreams serverStreams =
                    SctpStandardSocketOptions.InitMaxStreams.create(4, 5);
            server.setOption(SctpStandardSocketOptions.SCTP_INIT_MAXSTREAMS, serverStreams);
            SctpStandardSocketOptions.InitMaxStreams actualServerStreams =
                    server.getOption(SctpStandardSocketOptions.SCTP_INIT_MAXSTREAMS);
            check(actualServerStreams.maxInStreams() == 4, "server max inbound streams");
            check(actualServerStreams.maxOutStreams() == 5, "server max outbound streams");

            server.bind(new InetSocketAddress(loopback, 0));
            check(!server.getAllLocalAddresses().isEmpty(), "server local addresses");
            int port = ((InetSocketAddress) server.getAllLocalAddresses().iterator().next()).getPort();
            check(port > 0, "server port");
            InetAddress secondaryLoopback = InetAddress.getByName("127.0.0.2");
            InetSocketAddress secondaryAddress = new InetSocketAddress(secondaryLoopback, port);
            server.bindAddress(secondaryLoopback);
            check(server.getAllLocalAddresses().contains(secondaryAddress), "bindAddress");
            server.unbindAddress(secondaryLoopback);
            check(!server.getAllLocalAddresses().contains(secondaryAddress), "unbindAddress");

            server.configureBlocking(false);
            check(server.accept() == null, "non-blocking accept should be unavailable");
            server.configureBlocking(true);

            Thread serverThread = new Thread(() -> {
                try (SctpChannel accepted = server.accept()) {
                    check(accepted != null, "server accept returned null");
                    check("request".equals(readMessage(accepted, true, 0x01020304, false)),
                            "server payload");
                    ByteBuffer reply = ByteBuffer.wrap("response".getBytes(StandardCharsets.UTF_8));
                    MessageInfo outgoing = MessageInfo.createOutgoing(null, 0)
                            .payloadProtocolID(0x10203040)
                            .unordered(true);
                    check(accepted.send(reply, outgoing) == 8, "server send count");
                } catch (Throwable throwable) {
                    serverFailure.set(throwable);
                }
            });
            serverThread.start();

            try (SctpChannel client = SctpChannel.open()) {
                client.setOption(SctpStandardSocketOptions.SCTP_NODELAY, true);
                client.setOption(SctpStandardSocketOptions.SO_LINGER, 2);
                client.setOption(SctpStandardSocketOptions.SO_SNDBUF, 32 * 1024);
                client.setOption(SctpStandardSocketOptions.SO_RCVBUF, 32 * 1024);
                client.setOption(SctpStandardSocketOptions.SCTP_DISABLE_FRAGMENTS, true);
                client.setOption(SctpStandardSocketOptions.SCTP_FRAGMENT_INTERLEAVE, 1);
                check(client.getOption(SctpStandardSocketOptions.SCTP_NODELAY), "SCTP_NODELAY");
                check(client.getOption(SctpStandardSocketOptions.SO_LINGER) == 2, "SO_LINGER");
                check(client.getOption(SctpStandardSocketOptions.SO_SNDBUF) == 32 * 1024,
                        "SO_SNDBUF");
                check(client.getOption(SctpStandardSocketOptions.SO_RCVBUF) == 32 * 1024,
                        "SO_RCVBUF");
                check(client.getOption(SctpStandardSocketOptions.SCTP_DISABLE_FRAGMENTS),
                        "SCTP_DISABLE_FRAGMENTS");
                check(client.getOption(SctpStandardSocketOptions.SCTP_FRAGMENT_INTERLEAVE) == 1,
                        "SCTP_FRAGMENT_INTERLEAVE");
                try {
                    client.setOption(SctpStandardSocketOptions.SCTP_EXPLICIT_COMPLETE, true);
                    throw new AssertionError("SCTP_EXPLICIT_COMPLETE should be unsupported");
                } catch (IOException expected) {
                    // Linux deliberately maps this optional socket option to -1.
                }

                client.configureBlocking(false);
                boolean connected = client.connect(new InetSocketAddress(loopback, port));
                long deadline = System.currentTimeMillis() + 10_000;
                while (!connected && System.currentTimeMillis() < deadline) {
                    connected = client.finishConnect();
                    if (!connected) {
                        Thread.yield();
                    }
                }
                check(connected, "non-blocking connect completion");
                client.configureBlocking(true);
                check(client.association() != null, "association notification");
                check(!client.getAllLocalAddresses().isEmpty(), "client local addresses");
                check(!client.getRemoteAddresses().isEmpty(), "client remote addresses");
                SocketAddress primary = client.getOption(SctpStandardSocketOptions.SCTP_PRIMARY_ADDR);
                check(primary instanceof InetSocketAddress, "primary address");
                client.setOption(SctpStandardSocketOptions.SCTP_PRIMARY_ADDR, primary);

                ByteBuffer request = ByteBuffer.allocateDirect(16);
                request.put("request".getBytes(StandardCharsets.UTF_8)).flip();
                MessageInfo outgoing = MessageInfo.createOutgoing(null, 0)
                        .payloadProtocolID(0x01020304);
                check(client.send(request, outgoing) == 7, "client send count");
                check("response".equals(readMessage(client, false, 0x10203040, true)),
                        "client payload");
                client.shutdown();
            }

            serverThread.join(10_000);
            check(!serverThread.isAlive(), "server thread did not finish");
            if (serverFailure.get() != null) {
                throw new AssertionError("server thread failed", serverFailure.get());
            }
        }
    }

    public static void main(String[] args) throws Exception {
        try {
            exerciseSctp();
        } catch (UnsupportedOperationException unavailable) {
            // The reference JDK dynamically loads libsctp.so.1, which is optional on Linux.
            if ("ristretto".equals(System.getProperty("java.vm.vendor"))) {
                throw unavailable;
            }
        }
        System.out.println("SCTP integration complete");
    }
}
