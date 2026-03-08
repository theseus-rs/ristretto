import java.net.ServerSocket;
import java.net.Socket;
import java.io.InputStream;
import java.io.OutputStream;

/**
 * Test basic TCP socket communication using loopback.
 * Creates a server and client in separate threads.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== TCP Basic Test ===");

        ServerSocket serverSocket = new ServerSocket(0); // ephemeral port
        int port = serverSocket.getLocalPort();
        System.out.println("Server listening on port: " + (port > 0));

        // Capture server results for deterministic output
        String[] serverResults = new String[3];

        Thread serverThread = new Thread(() -> {
            try {
                Socket client = serverSocket.accept();
                serverResults[0] = "Server: client connected";

                InputStream in = client.getInputStream();
                byte[] buf = new byte[1024];
                int bytesRead = in.read(buf);
                String received = new String(buf, 0, bytesRead);
                serverResults[1] = "Server received: " + received;

                OutputStream out = client.getOutputStream();
                out.write(("Echo: " + received).getBytes());
                out.flush();
                serverResults[2] = "Server: sent echo";

                client.close();
                serverSocket.close();
            } catch (Exception e) {
                serverResults[0] = "Server error: " + e.getMessage();
            }
        });
        serverThread.start();

        // Give server time to start accepting
        Thread.sleep(100);

        Socket socket = new Socket("127.0.0.1", port);
        boolean localPortBound = socket.getLocalPort() > 0;
        boolean remotePortMatch = socket.getPort() == port;

        OutputStream out = socket.getOutputStream();
        out.write("Hello".getBytes());
        out.flush();

        InputStream in = socket.getInputStream();
        byte[] buf = new byte[1024];
        int bytesRead = in.read(buf);
        String response = new String(buf, 0, bytesRead);

        socket.close();
        serverThread.join();

        // Print all results in deterministic order
        for (String r : serverResults) {
            if (r != null) System.out.println(r);
        }
        System.out.println("Client: connected to server");
        System.out.println("Client: local port bound: " + localPortBound);
        System.out.println("Client: remote port: " + remotePortMatch);
        System.out.println("Client: sent Hello");
        System.out.println("Client received: " + response);
        System.out.println("Client: socket closed");

        System.out.println("=== TCP Basic Test Complete ===");
    }
}
