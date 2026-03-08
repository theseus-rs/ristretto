import java.net.ServerSocket;
import java.net.Socket;
import java.io.InputStream;
import java.io.OutputStream;

/**
 * Test server accepting multiple sequential clients.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== TCP Multi-Client Test ===");

        ServerSocket serverSocket = new ServerSocket(0);
        int port = serverSocket.getLocalPort();
        int clientCount = 3;

        Thread serverThread = new Thread(() -> {
            try {
                for (int i = 0; i < clientCount; i++) {
                    Socket client = serverSocket.accept();
                    InputStream in = client.getInputStream();
                    byte[] buf = new byte[1024];
                    int bytesRead = in.read(buf);
                    String received = new String(buf, 0, bytesRead);
                    System.out.println("Server received from client " + i + ": " + received);

                    OutputStream out = client.getOutputStream();
                    out.write(("Reply " + i).getBytes());
                    out.flush();
                    client.close();
                }
                serverSocket.close();
            } catch (Exception e) {
                System.out.println("Server error: " + e.getMessage());
            }
        });
        serverThread.start();

        Thread.sleep(100);

        // Connect clients sequentially for deterministic output
        for (int i = 0; i < clientCount; i++) {
            Socket socket = new Socket("127.0.0.1", port);
            OutputStream out = socket.getOutputStream();
            out.write(("Client " + i).getBytes());
            out.flush();

            InputStream in = socket.getInputStream();
            byte[] buf = new byte[1024];
            int bytesRead = in.read(buf);
            String response = new String(buf, 0, bytesRead);
            System.out.println("Client " + i + " received: " + response);
            socket.close();
        }

        serverThread.join();
        System.out.println("=== TCP Multi-Client Test Complete ===");
    }
}
