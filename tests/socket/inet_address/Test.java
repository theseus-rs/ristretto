import java.net.InetAddress;
import java.net.Inet4Address;

/**
 * Test InetAddress operations.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== InetAddress Test ===");

        // Test loopback address
        InetAddress loopback = InetAddress.getLoopbackAddress();
        System.out.println("Loopback: " + loopback.getHostAddress());
        System.out.println("Loopback isLoopback: " + loopback.isLoopbackAddress());

        // Test getByName with IP string
        InetAddress byName = InetAddress.getByName("127.0.0.1");
        System.out.println("ByName 127.0.0.1: " + byName.getHostAddress());
        System.out.println("ByName isLoopback: " + byName.isLoopbackAddress());

        // Test getByAddress
        byte[] addrBytes = {127, 0, 0, 1};
        InetAddress byAddr = InetAddress.getByAddress(addrBytes);
        System.out.println("ByAddress: " + byAddr.getHostAddress());

        // Test localhost
        InetAddress localhost = InetAddress.getByName("localhost");
        System.out.println("Localhost isLoopback: " + localhost.isLoopbackAddress());

        // Test getLocalHost
        InetAddress localHost = InetAddress.getLocalHost();
        System.out.println("LocalHost not null: " + (localHost != null));
        System.out.println("LocalHost hostname not empty: " + (!localHost.getHostName().isEmpty()));

        // Test wildcard address
        byte[] wildcardBytes = {0, 0, 0, 0};
        InetAddress wildcard = InetAddress.getByAddress(wildcardBytes);
        System.out.println("Wildcard isAnyLocal: " + wildcard.isAnyLocalAddress());

        // Test address properties
        System.out.println("Loopback isSiteLocal: " + loopback.isSiteLocalAddress());
        System.out.println("Loopback isMulticast: " + loopback.isMulticastAddress());
        System.out.println("Loopback isLinkLocal: " + loopback.isLinkLocalAddress());

        // Test equality
        InetAddress loopback2 = InetAddress.getByName("127.0.0.1");
        System.out.println("Loopback equals: " + byName.equals(loopback2));

        System.out.println("=== InetAddress Test Complete ===");
    }
}
