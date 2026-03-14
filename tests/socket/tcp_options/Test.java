import java.net.ServerSocket;
import java.net.Socket;

/**
 * Test getting and setting socket options.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== TCP Options Test ===");

        // Test ServerSocket options
        ServerSocket serverSocket = new ServerSocket(0);
        serverSocket.setReuseAddress(true);
        System.out.println("ServerSocket reuseAddress: " + serverSocket.getReuseAddress());

        int port = serverSocket.getLocalPort();

        Thread serverThread = new Thread(() -> {
            try {
                Socket client = serverSocket.accept();
                client.close();
                serverSocket.close();
            } catch (Exception e) {
                System.out.println("Server error: " + e.getMessage());
            }
        });
        serverThread.start();

        Thread.sleep(100);

        // Test client Socket options
        Socket socket = new Socket("127.0.0.1", port);

        // TCP_NODELAY
        socket.setTcpNoDelay(true);
        System.out.println("TCP_NODELAY: " + socket.getTcpNoDelay());

        // SO_KEEPALIVE
        socket.setKeepAlive(true);
        System.out.println("SO_KEEPALIVE: " + socket.getKeepAlive());

        // SO_REUSEADDR
        System.out.println("SO_REUSEADDR: " + socket.getReuseAddress());

        // SO_RCVBUF
        int origRcvBuf = socket.getReceiveBufferSize();
        System.out.println("SO_RCVBUF > 0: " + (origRcvBuf > 0));

        // SO_SNDBUF
        int origSndBuf = socket.getSendBufferSize();
        System.out.println("SO_SNDBUF > 0: " + (origSndBuf > 0));

        // SO_LINGER
        socket.setSoLinger(true, 10);
        System.out.println("SO_LINGER: " + socket.getSoLinger());

        // SO_TIMEOUT
        socket.setSoTimeout(5000);
        System.out.println("SO_TIMEOUT: " + socket.getSoTimeout());

        // OOB inline
        socket.setOOBInline(true);
        System.out.println("OOB inline: " + socket.getOOBInline());

        socket.close();
        serverThread.join();
        System.out.println("=== TCP Options Test Complete ===");
    }
}
