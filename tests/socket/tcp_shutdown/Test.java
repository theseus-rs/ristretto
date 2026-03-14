import java.net.ServerSocket;
import java.net.Socket;
import java.io.InputStream;
import java.io.OutputStream;

/**
 * Test socket half-close (shutdown) operations.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== TCP Shutdown Test ===");
        System.out.flush();

        ServerSocket serverSocket = new ServerSocket(0);
        int port = serverSocket.getLocalPort();

        String[] serverResults = new String[2];

        Thread serverThread = new Thread(() -> {
            try {
                Socket client = serverSocket.accept();
                InputStream in = client.getInputStream();
                OutputStream out = client.getOutputStream();

                byte[] buf = new byte[1024];
                int bytesRead = in.read(buf);
                String received = new String(buf, 0, bytesRead);
                serverResults[0] = "Server received: " + received;

                out.write("ServerReply".getBytes());
                out.flush();
                client.shutdownOutput();
                serverResults[1] = "Server: isOutputShutdown: " + client.isOutputShutdown();

                client.close();
            } catch (Throwable e) {
                serverResults[0] = "Server error: " + e.getClass().getSimpleName() + ": " + e.getMessage();
            }
        });
        serverThread.start();

        Thread.sleep(100);

        Socket socket = new Socket("127.0.0.1", port);

        OutputStream out = socket.getOutputStream();
        out.write("ClientData".getBytes());
        out.flush();

        Thread.sleep(200);

        InputStream in = socket.getInputStream();
        byte[] buf = new byte[1024];
        int bytesRead = in.read(buf);
        String response = new String(buf, 0, bytesRead);

        int nextRead = in.read(buf);

        socket.close();

        serverThread.join(5000);

        for (String r : serverResults) {
            if (r != null) System.out.println(r);
        }
        System.out.println("Client received: " + response);
        System.out.println("Client: read after server shutdown: " + nextRead);
        System.out.println("Client: isClosed: " + socket.isClosed());

        System.out.println("=== TCP Shutdown Test Complete ===");
    }
}
