import com.sun.nio.sctp.Association;
import com.sun.nio.sctp.MessageInfo;
import com.sun.nio.sctp.SctpChannel;
import com.sun.nio.sctp.SctpMultiChannel;
import com.sun.nio.sctp.SctpServerChannel;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.atomic.AtomicReference;

/** Integration coverage for one-to-many associations, association shutdown, and peeloff. */
public class Test {
    private static void check(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }

    private static MessageInfo receive(SctpMultiChannel channel, String expected) throws Exception {
        ByteBuffer buffer = ByteBuffer.allocateDirect(64);
        MessageInfo info = channel.receive(buffer, null, null);
        check(info != null, "multi-channel receive returned null");
        buffer.flip();
        byte[] bytes = new byte[buffer.remaining()];
        buffer.get(bytes);
        check(expected.equals(new String(bytes, StandardCharsets.UTF_8)), "multi-channel payload");
        return info;
    }

    private static String receive(SctpChannel channel) throws Exception {
        ByteBuffer buffer = ByteBuffer.allocate(64);
        MessageInfo info = channel.receive(buffer, null, null);
        check(info != null, "channel receive returned null");
        buffer.flip();
        byte[] bytes = new byte[buffer.remaining()];
        buffer.get(bytes);
        return new String(bytes, StandardCharsets.UTF_8);
    }

    private static void exerciseMultiChannel() throws Exception {
        InetAddress loopback = InetAddress.getByName("127.0.0.1");
        AtomicReference<Throwable> serverFailure = new AtomicReference<>();

        try (SctpServerChannel server = SctpServerChannel.open()) {
            server.bind(new InetSocketAddress(loopback, 0));
            InetSocketAddress serverAddress =
                    (InetSocketAddress) server.getAllLocalAddresses().iterator().next();

            Thread serverThread = new Thread(() -> {
                try (SctpChannel accepted = server.accept()) {
                    check("first".equals(receive(accepted)), "first server payload");
                    accepted.send(ByteBuffer.wrap("one".getBytes(StandardCharsets.UTF_8)),
                            MessageInfo.createOutgoing(null, 0));
                    check("second".equals(receive(accepted)), "second server payload");
                    accepted.send(ByteBuffer.wrap("two".getBytes(StandardCharsets.UTF_8)),
                            MessageInfo.createOutgoing(null, 0));
                } catch (Throwable throwable) {
                    serverFailure.set(throwable);
                }
            });
            serverThread.start();

            try (SctpMultiChannel multi = SctpMultiChannel.open()) {
                multi.bind(new InetSocketAddress(loopback, 0));
                check(!multi.getAllLocalAddresses().isEmpty(), "multi-channel local addresses");
                MessageInfo outgoing = MessageInfo.createOutgoing(serverAddress, 0)
                        .payloadProtocolID(77);
                check(multi.send(ByteBuffer.wrap("first".getBytes(StandardCharsets.UTF_8)), outgoing) == 5,
                        "initial multi-channel send");

                MessageInfo response = receive(multi, "one");
                Association association = response.association();
                check(association != null, "multi-channel association");
                check(multi.associations().contains(association), "association tracking");
                check(!multi.getRemoteAddresses(association).isEmpty(), "association addresses");

                try (SctpChannel branched = multi.branch(association)) {
                    check(!multi.associations().contains(association), "association peeloff");
                    check(branched.association().equals(association), "branched association");
                    check(branched.send(ByteBuffer.wrap("second".getBytes(StandardCharsets.UTF_8)),
                            MessageInfo.createOutgoing(null, 0)) == 6, "branched send");
                    check("two".equals(receive(branched)), "branched receive");
                    branched.shutdown();
                }
            }

            serverThread.join(10_000);
            check(!serverThread.isAlive(), "multi-channel server thread did not finish");
            if (serverFailure.get() != null) {
                throw new AssertionError("multi-channel server failed", serverFailure.get());
            }
        }
    }

    public static void main(String[] args) throws Exception {
        try {
            exerciseMultiChannel();
        } catch (UnsupportedOperationException unavailable) {
            // libsctp.so.1 is optional for the reference JDK.
            if ("ristretto".equals(System.getProperty("java.vm.vendor"))) {
                throw unavailable;
            }
        }
        System.out.println("SCTP multi-channel integration complete");
    }
}
