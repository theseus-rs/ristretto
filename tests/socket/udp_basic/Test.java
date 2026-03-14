import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.InetAddress;

/**
 * Test basic UDP datagram socket communication.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== UDP Basic Test ===");

        DatagramSocket serverSocket = new DatagramSocket(0);
        int port = serverSocket.getLocalPort();
        System.out.println("Server bound on port: " + (port > 0));

        Thread serverThread = new Thread(() -> {
            try {
                // Receive packet
                byte[] buf = new byte[1024];
                DatagramPacket packet = new DatagramPacket(buf, buf.length);
                serverSocket.receive(packet);
                String received = new String(packet.getData(), 0, packet.getLength());
                System.out.println("Server received: " + received);
                System.out.println("Server: packet length: " + packet.getLength());

                // Send reply
                byte[] replyData = ("Echo: " + received).getBytes();
                DatagramPacket replyPacket = new DatagramPacket(
                    replyData, replyData.length,
                    packet.getAddress(), packet.getPort());
                System.out.println("Server: sent reply");
                serverSocket.send(replyPacket);

                serverSocket.close();
            } catch (Exception e) {
                System.out.println("Server error: " + e.getMessage());
            }
        });
        serverThread.start();

        Thread.sleep(100);

        DatagramSocket clientSocket = new DatagramSocket();
        System.out.println("Client bound on port: " + (clientSocket.getLocalPort() > 0));

        // Send packet
        byte[] sendData = "HelloUDP".getBytes();
        DatagramPacket sendPacket = new DatagramPacket(
            sendData, sendData.length,
            InetAddress.getByName("127.0.0.1"), port);
        clientSocket.send(sendPacket);
        System.out.println("Client: sent packet");

        // Receive reply
        byte[] buf = new byte[1024];
        DatagramPacket recvPacket = new DatagramPacket(buf, buf.length);
        clientSocket.receive(recvPacket);
        String response = new String(recvPacket.getData(), 0, recvPacket.getLength());
        System.out.println("Client received: " + response);

        clientSocket.close();
        serverThread.join();
        System.out.println("=== UDP Basic Test Complete ===");
    }
}
