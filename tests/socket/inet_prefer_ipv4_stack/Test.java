import java.net.DatagramSocket;
import java.net.Inet4Address;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.net.ServerSocket;

public class Test {
    public static void main(String[] args) throws Exception {
        InetAddress[] localhost = InetAddress.getAllByName("localhost");
        boolean ipv4Only = localhost.length > 0;
        for (InetAddress address : localhost) {
            ipv4Only &= address instanceof Inet4Address;
        }

        System.out.println("prefer-ipv4-property:"
                + System.getProperty("java.net.preferIPv4Stack"));
        System.out.println("prefer-ipv4-localhost:" + ipv4Only);

        int javaVersion = javaVersion();
        if (javaVersion <= 17) {
            try (DatagramSocket socket = new DatagramSocket(null)) {
                socket.bind(new InetSocketAddress(0));
                System.out.println("prefer-ipv4-datagram:"
                        + (socket.getLocalAddress() instanceof Inet4Address));
            }
            try (ServerSocket socket = new ServerSocket()) {
                socket.bind(new InetSocketAddress(0));
                System.out.println("prefer-ipv4-server:"
                        + (socket.getInetAddress() instanceof Inet4Address));
            }
        }
    }

    private static int javaVersion() {
        String version = System.getProperty("java.specification.version");
        if (version.startsWith("1.")) {
            version = version.substring(2);
        }
        int dot = version.indexOf('.');
        return Integer.parseInt(dot < 0 ? version : version.substring(0, dot));
    }
}
