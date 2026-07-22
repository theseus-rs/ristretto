import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.InetAddress;
import java.net.MulticastSocket;
import java.net.NetworkInterface;
import java.net.SocketAddress;
import java.net.SocketException;
import java.net.SocketOption;
import java.net.StandardSocketOptions;
import java.lang.reflect.Method;
import java.util.Arrays;
import java.util.Set;

public class Test {
    public static void main(String[] args) throws Exception {
        packetOffsets();
        connectAndDisconnect();
        multicastOptions();
        datagramOptions();
        receiveTimeout();
        closeUnblocksReceive();
    }

    private static void packetOffsets() throws Exception {
        DatagramSocket receiver = new DatagramSocket(0, InetAddress.getByName("127.0.0.1"));
        receiver.setSoTimeout(2000);
        DatagramSocket sender = new DatagramSocket();
        byte[] source = new byte[] {99, 1, 2, 3, 4, 88};
        sender.send(new DatagramPacket(source, 1, 4,
                InetAddress.getByName("127.0.0.1"), receiver.getLocalPort()));
        byte[] target = new byte[] {-1, -1, -1, -1, -1, -1, -1, -1};
        DatagramPacket packet = new DatagramPacket(target, 2, 4);
        receiver.receive(packet);
        System.out.println("udp-offset:" + packet.getLength() + ":" + Arrays.toString(target));
        System.out.println("udp-source:" + packet.getAddress().isLoopbackAddress()
                + ":" + (packet.getPort() > 0));
        sender.close();
        receiver.close();
    }

    private static void connectAndDisconnect() throws Exception {
        DatagramSocket first = new DatagramSocket(0, InetAddress.getByName("127.0.0.1"));
        DatagramSocket second = new DatagramSocket(0, InetAddress.getByName("127.0.0.1"));
        first.setSoTimeout(2000);
        second.setSoTimeout(2000);
        DatagramSocket sender = new DatagramSocket();
        sender.connect(InetAddress.getByName("127.0.0.1"), first.getLocalPort());
        sender.send(new DatagramPacket(new byte[] {7}, 1));
        DatagramPacket received = new DatagramPacket(new byte[1], 1);
        first.receive(received);
        System.out.println("udp-connected-send:true");
        int localPort = sender.getLocalPort();
        sender.disconnect();
        System.out.println("udp-disconnected:true");
        sender.send(new DatagramPacket(new byte[] {8}, 1,
                InetAddress.getByName("127.0.0.1"), second.getLocalPort()));
        second.receive(received);
        System.out.println("udp-connect-disconnect:" + received.getData()[0]
                + ":" + (sender.getPort() == -1)
                + ":" + (sender.getLocalPort() == localPort));
        sender.close();
        first.close();
        second.close();
    }

    private static void multicastOptions() throws Exception {
        MulticastSocket socket = new MulticastSocket((java.net.SocketAddress) null);
        NetworkInterface loopback = NetworkInterface.getByInetAddress(
                InetAddress.getByName("127.0.0.1"));
        socket.setReuseAddress(true);
        socket.setTimeToLive(17);
        socket.setLoopbackMode(true);
        socket.setNetworkInterface(loopback);
        socket.bind(new java.net.InetSocketAddress(0));
        System.out.println("udp-options:" + socket.getReuseAddress() + ":"
                + socket.getTimeToLive() + ":" + socket.getLoopbackMode());
        System.out.println("udp-multicast-interface:"
                + (socket.getNetworkInterface().getIndex() == loopback.getIndex()));
        socket.setInterface(InetAddress.getByName("127.0.0.1"));
        System.out.println("udp-multicast-address:"
                + socket.getInterface().isLoopbackAddress());
        socket.close();
    }

    @SuppressWarnings({"unchecked", "rawtypes"})
    private static void datagramOptions() throws Exception {
        DatagramSocket socket = new DatagramSocket((SocketAddress) null);
        socket.setBroadcast(true);
        socket.setTrafficClass(0x20);
        socket.setSendBufferSize(4096);
        socket.setReceiveBufferSize(4096);
        socket.bind(new java.net.InetSocketAddress(
                InetAddress.getByName("127.0.0.1"), 0));
        System.out.println("udp-basic-options:" + socket.getBroadcast() + ":"
                + (socket.getTrafficClass() == 0x20) + ":"
                + (socket.getSendBufferSize() >= 4096) + ":"
                + (socket.getReceiveBufferSize() >= 4096));
        System.out.println("udp-bind-address:" + socket.getLocalAddress().getHostAddress() + ":"
                + socket.getLocalAddress().isLoopbackAddress());

        boolean reusePortSupported = true;
        try {
            Method supportedOptions = DatagramSocket.class.getMethod("supportedOptions");
            Set options = (Set) supportedOptions.invoke(socket);
            Object reusePort = StandardSocketOptions.class.getField("SO_REUSEPORT").get(null);
            reusePortSupported = options.contains(reusePort);
            if (reusePortSupported) {
                Method setOption = DatagramSocket.class.getMethod(
                        "setOption", SocketOption.class, Object.class);
                Method getOption = DatagramSocket.class.getMethod(
                        "getOption", SocketOption.class);
                setOption.invoke(socket, reusePort, Boolean.TRUE);
                reusePortSupported = Boolean.TRUE.equals(getOption.invoke(socket, reusePort));
            }
        } catch (NoSuchMethodException expectedOnJava8) {
            // SO_REUSEPORT and the generic option methods were introduced after Java 8.
        }
        System.out.println("udp-reuseport:" + reusePortSupported);
        socket.close();
    }

    private static void closeUnblocksReceive() throws Exception {
        final DatagramSocket socket = new DatagramSocket(0, InetAddress.getByName("127.0.0.1"));
        final boolean[] unblocked = {false};
        Thread reader = new Thread(new Runnable() {
            public void run() {
                try {
                    socket.receive(new DatagramPacket(new byte[1], 1));
                } catch (SocketException expected) {
                    unblocked[0] = true;
                } catch (Exception error) {
                    throw new RuntimeException(error);
                }
            }
        });
        reader.start();
        Thread.sleep(25);
        socket.close();
        reader.join(2000);
        System.out.println("udp-close-unblocks:" + unblocked[0]);
    }

    private static void receiveTimeout() throws Exception {
        DatagramSocket socket = new DatagramSocket(0, InetAddress.getByName("127.0.0.1"));
        socket.setSoTimeout(40);
        boolean timedOut = false;
        try {
            socket.receive(new DatagramPacket(new byte[1], 1));
        } catch (java.net.SocketTimeoutException expected) {
            timedOut = true;
        }
        socket.close();
        System.out.println("udp-receive-timeout:" + timedOut);
    }
}
