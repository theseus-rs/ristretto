import java.net.ServerSocket;
import java.net.Socket;
import java.net.SocketTimeoutException;

/**
 * Test socket read timeout behavior.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== TCP Timeout Test ===");

        ServerSocket serverSocket = new ServerSocket(0);
        int port = serverSocket.getLocalPort();

        // Test ServerSocket accept timeout
        serverSocket.setSoTimeout(200);
        try {
            serverSocket.accept(); // No client connecting, should timeout
            System.out.println("Accept: should not reach here");
        } catch (SocketTimeoutException e) {
            System.out.println("Accept timeout caught: true");
        }

        // Reset for actual use
        serverSocket.setSoTimeout(0);

        Thread serverThread = new Thread(() -> {
            try {
                Socket client = serverSocket.accept();
                // Don't send any data - let client timeout on read
                Thread.sleep(1000);
                client.close();
                serverSocket.close();
            } catch (Exception e) {
                System.out.println("Server error: " + e.getMessage());
            }
        });
        serverThread.start();

        Thread.sleep(100);

        Socket socket = new Socket("127.0.0.1", port);
        socket.setSoTimeout(200);
        System.out.println("Socket timeout set: " + socket.getSoTimeout());

        long startTime = System.currentTimeMillis();
        try {
            socket.getInputStream().read();
            System.out.println("Read: should not reach here");
        } catch (SocketTimeoutException e) {
            long elapsed = System.currentTimeMillis() - startTime;
            System.out.println("Read timeout caught: true");
            System.out.println("Timeout in range: " + (elapsed >= 100 && elapsed <= 2000));
        }

        socket.close();
        serverThread.join();
        System.out.println("=== TCP Timeout Test Complete ===");
    }
}
