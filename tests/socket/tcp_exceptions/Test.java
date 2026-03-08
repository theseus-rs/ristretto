import java.net.ConnectException;
import java.net.BindException;
import java.net.ServerSocket;
import java.net.Socket;
import java.net.SocketException;

/**
 * Test socket exception handling.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== TCP Exceptions Test ===");

        // Test 1: ConnectException - connect to a port with no listener
        testConnectRefused();

        // Test 2: BindException - bind to already-bound port
        testBindConflict();

        // Test 3: SocketException - read from closed socket
        testReadFromClosed();

        System.out.println("=== TCP Exceptions Test Complete ===");
    }

    static void testConnectRefused() {
        System.out.println("--- Connect Refused ---");
        try {
            // Use a port that is very unlikely to have a listener
            // Port 1 typically requires root and has no service
            Socket socket = new Socket("127.0.0.1", 1);
            socket.close();
            System.out.println("Connect: should not reach here");
        } catch (ConnectException e) {
            System.out.println("ConnectException caught: true");
        } catch (Exception e) {
            System.out.println("Other exception: " + e.getClass().getSimpleName());
        }
    }

    static void testBindConflict() throws Exception {
        System.out.println("--- Bind Conflict ---");
        ServerSocket ss1 = new ServerSocket(0);
        int port = ss1.getLocalPort();

        try {
            ServerSocket ss2 = new ServerSocket(port);
            ss2.close();
            System.out.println("Bind: should not reach here");
        } catch (BindException e) {
            System.out.println("BindException caught: true");
        } catch (Exception e) {
            System.out.println("Other exception: " + e.getClass().getSimpleName());
        }

        ss1.close();
    }

    static void testReadFromClosed() throws Exception {
        System.out.println("--- Read from Closed ---");
        ServerSocket serverSocket = new ServerSocket(0);
        int port = serverSocket.getLocalPort();

        Thread serverThread = new Thread(() -> {
            try {
                Socket client = serverSocket.accept();
                Thread.sleep(100);
                client.close();
                serverSocket.close();
            } catch (Exception e) {
                // expected
            }
        });
        serverThread.start();

        Thread.sleep(50);

        Socket socket = new Socket("127.0.0.1", port);
        socket.close();

        try {
            socket.getInputStream().read();
            System.out.println("Read: should not reach here");
        } catch (SocketException e) {
            System.out.println("SocketException caught: true");
        }

        serverThread.join();
    }
}
