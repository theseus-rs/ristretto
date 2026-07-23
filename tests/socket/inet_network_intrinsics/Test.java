import java.net.InetAddress;
import java.net.Inet6Address;
import java.net.InterfaceAddress;
import java.net.NetworkInterface;
import java.net.UnknownHostException;
import java.util.Enumeration;

public class Test {
    public static void main(String[] args) throws Exception {
        testNetworkInterfaces();

        InetAddress loopback = InetAddress.getByName("127.0.0.1");
        InetAddress[] localhost = InetAddress.getAllByName("localhost");
        System.out.println("inet-forward:" + (localhost.length > 0));
        System.out.println("inet-reverse:" + !loopback.getHostName().isEmpty());
        System.out.println("inet-reachable:" + loopback.isReachable(250));
        boolean ipv6Literal = false;
        try {
            ipv6Literal = InetAddress.getByName("::1") instanceof Inet6Address;
        } catch (UnknownHostException unsupported) {
            // IPv6 can be disabled by the host or java.net.preferIPv4Stack.
        }
        System.out.println("inet-ipv6-literal:" + ipv6Literal);
        boolean unknown = false;
        try {
            InetAddress.getAllByName("definitely-not-a-host.invalid");
        } catch (UnknownHostException expected) {
            unknown = true;
        }
        System.out.println("inet-unknown:" + unknown);

    }

    private static void testNetworkInterfaces() throws Exception {
        Enumeration<NetworkInterface> interfaces = NetworkInterface.getNetworkInterfaces();
        int count = 0;
        boolean lookupByName = false;
        boolean lookupByIndex = false;
        boolean lookupByAddress = false;
        boolean saneProperties = true;
        boolean saneScopes = true;
        boolean saneBindings = true;
        while (interfaces != null && interfaces.hasMoreElements()) {
            NetworkInterface network = interfaces.nextElement();
            count++;
            lookupByName |= NetworkInterface.getByName(network.getName()) != null;
            lookupByIndex |= network.getIndex() <= 0
                    || NetworkInterface.getByIndex(network.getIndex()) != null;
            Enumeration<InetAddress> addresses = network.getInetAddresses();
            while (addresses.hasMoreElements()) {
                InetAddress address = addresses.nextElement();
                lookupByAddress |= NetworkInterface.getByInetAddress(address) != null;
                if (address instanceof Inet6Address
                        && !address.isLinkLocalAddress()
                        && !address.isMulticastAddress()) {
                    saneScopes &= ((Inet6Address) address).getScopeId() == 0;
                }
            }
            saneProperties &= network.getMTU() >= -1;
            network.isUp();
            network.isLoopback();
            network.isPointToPoint();
            network.supportsMulticast();
            network.getHardwareAddress();
            for (InterfaceAddress binding : network.getInterfaceAddresses()) {
                InetAddress bindingAddress = binding.getAddress();
                short prefix = binding.getNetworkPrefixLength();
                int maxPrefix = bindingAddress instanceof Inet6Address ? 128 : 32;
                saneBindings &= bindingAddress != null && prefix >= 0 && prefix <= maxPrefix;
                saneBindings &= binding.getBroadcast() == null
                        || !(binding.getBroadcast() instanceof Inet6Address);
            }
        }
        System.out.println("network-count:" + (count > 0));
        System.out.println("network-lookups:" + lookupByName + ":" + lookupByIndex
                + ":" + lookupByAddress);
        System.out.println("network-properties:" + saneProperties);
        System.out.println("network-address-metadata:" + saneScopes + ":"
                + saneBindings);
    }
}
