import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.InetAddress;

/**
 * Test connected DatagramSocket behavior.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== UDP Connected Test ===");

        DatagramSocket serverSocket = new DatagramSocket(0);
        int port = serverSocket.getLocalPort();

        Thread serverThread = new Thread(() -> {
            try {
                byte[] buf = new byte[1024];
                DatagramPacket packet = new DatagramPacket(buf, buf.length);
                serverSocket.receive(packet);
                String received = new String(packet.getData(), 0, packet.getLength());
                System.out.println("Server received: " + received);

                // Reply
                byte[] replyData = "Connected-Reply".getBytes();
                DatagramPacket replyPacket = new DatagramPacket(
                    replyData, replyData.length,
                    packet.getAddress(), packet.getPort());
                serverSocket.send(replyPacket);

                serverSocket.close();
            } catch (Exception e) {
                System.out.println("Server error: " + e.getMessage());
            }
        });
        serverThread.start();

        Thread.sleep(100);

        DatagramSocket clientSocket = new DatagramSocket();
        InetAddress loopback = InetAddress.getByName("127.0.0.1");

        // Connect to server
        clientSocket.connect(loopback, port);
        System.out.println("Client connected: " + clientSocket.isConnected());
        System.out.println("Client remote port: " + (clientSocket.getPort() == port));

        // Send without specifying address (uses connected address)
        byte[] sendData = "Connected-Hello".getBytes();
        DatagramPacket sendPacket = new DatagramPacket(sendData, sendData.length);
        clientSocket.send(sendPacket);
        System.out.println("Client: sent on connected socket");

        // Receive
        byte[] buf = new byte[1024];
        DatagramPacket recvPacket = new DatagramPacket(buf, buf.length);
        clientSocket.receive(recvPacket);
        String response = new String(recvPacket.getData(), 0, recvPacket.getLength());
        System.out.println("Client received: " + response);

        // Disconnect
        clientSocket.disconnect();
        System.out.println("Client connected after disconnect: " + clientSocket.isConnected());

        clientSocket.close();
        serverThread.join();
        System.out.println("=== UDP Connected Test Complete ===");
    }
}
