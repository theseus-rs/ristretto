import java.net.Inet4Address;
import java.net.InetAddress;
import java.net.NetworkInterface;
import java.net.StandardProtocolFamily;
import java.net.StandardSocketOptions;
import java.nio.channels.DatagramChannel;
import java.nio.channels.MembershipKey;

public class Test {
    public static void main(String[] args) throws Exception {
        InetAddress loopbackAddress = InetAddress.getByName("127.0.0.1");
        NetworkInterface loopback = NetworkInterface.getByInetAddress(loopbackAddress);
        if (loopback == null) {
            System.out.println("Multicast skipped: no loopback interface");
            return;
        }
        InetAddress group = InetAddress.getByName("230.0.0.1");
        try (DatagramChannel channel = DatagramChannel.open(StandardProtocolFamily.INET)) {
            channel.setOption(StandardSocketOptions.SO_REUSEADDR, true);
            channel.setOption(StandardSocketOptions.IP_MULTICAST_IF, loopback);
            NetworkInterface configured = channel.getOption(StandardSocketOptions.IP_MULTICAST_IF);
            MembershipKey key = channel.join(group, loopback);
            System.out.println("Multicast interface roundtrip: " + loopback.equals(configured));
            System.out.println("Multicast joined: " + key.isValid());
            InetAddress source = InetAddress.getByName("192.0.2.1");
            try {
                key.block(source);
                System.out.println("Multicast source filtering: true");
                key.unblock(source);
            } catch (UnsupportedOperationException unsupported) {
                // Source filtering is optional and is not supported by the macOS
                // DatagramChannel implementation.
                System.out.println("Multicast source filtering: false");
            }
            key.drop();
            System.out.println("Multicast dropped: " + !key.isValid());
        }
    }
}
