import java.net.ServerSocket;
import java.net.Socket;
import java.net.InetAddress;
import java.net.InetSocketAddress;

/**
 * Test ServerSocket creation, binding, and properties.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== Server Socket Test ===");

        // Test 1: Basic creation with ephemeral port
        ServerSocket ss1 = new ServerSocket(0);
        System.out.println("Bound: " + ss1.isBound());
        System.out.println("Closed: " + ss1.isClosed());
        System.out.println("Port > 0: " + (ss1.getLocalPort() > 0));
        System.out.println("Address not null: " + (ss1.getInetAddress() != null));

        int port = ss1.getLocalPort();

        // Test 2: Accept a connection
        Thread serverThread = new Thread(() -> {
            try {
                Socket client = ss1.accept();
                System.out.println("Accepted client: " + (client != null));
                System.out.println("Client connected: " + client.isConnected());
                client.close();
                ss1.close();
            } catch (Exception e) {
                System.out.println("Server error: " + e.getMessage());
            }
        });
        serverThread.start();

        Thread.sleep(100);

        Socket client = new Socket("127.0.0.1", port);
        client.close();
        serverThread.join();

        System.out.println("ServerSocket closed: " + ss1.isClosed());

        // Test 3: Bind with backlog
        ServerSocket ss2 = new ServerSocket(0, 5);
        System.out.println("Backlog server port > 0: " + (ss2.getLocalPort() > 0));
        ss2.close();

        // Test 4: Bind to specific address
        ServerSocket ss3 = new ServerSocket(0, 5, InetAddress.getByName("127.0.0.1"));
        System.out.println("Loopback bind address: " + ss3.getInetAddress().getHostAddress());
        ss3.close();

        // Test 5: Unbound ServerSocket then bind
        ServerSocket ss4 = new ServerSocket();
        System.out.println("Unbound - isBound: " + ss4.isBound());
        ss4.bind(new InetSocketAddress("127.0.0.1", 0));
        System.out.println("After bind - isBound: " + ss4.isBound());
        System.out.println("After bind - port > 0: " + (ss4.getLocalPort() > 0));
        ss4.close();

        System.out.println("=== Server Socket Test Complete ===");
    }
}
