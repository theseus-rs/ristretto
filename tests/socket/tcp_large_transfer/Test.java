import java.net.ServerSocket;
import java.net.Socket;
import java.io.InputStream;
import java.io.OutputStream;

/**
 * Test transferring larger data (64KB) over TCP.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== TCP Large Transfer Test ===");

        int dataSize = 65536; // 64KB
        byte[] sendData = new byte[dataSize];
        for (int i = 0; i < dataSize; i++) {
            sendData[i] = (byte) (i % 256);
        }

        ServerSocket serverSocket = new ServerSocket(0);
        int port = serverSocket.getLocalPort();

        // Capture server results for deterministic printing
        String[] serverResults = new String[2];

        Thread serverThread = new Thread(() -> {
            try {
                Socket client = serverSocket.accept();
                InputStream in = client.getInputStream();
                byte[] received = new byte[dataSize];
                int totalRead = 0;
                while (totalRead < dataSize) {
                    int bytesRead = in.read(received, totalRead, dataSize - totalRead);
                    if (bytesRead == -1) break;
                    totalRead += bytesRead;
                }
                serverResults[0] = "Server: received " + totalRead + " bytes";

                // Verify data integrity
                boolean match = true;
                for (int i = 0; i < dataSize; i++) {
                    if (received[i] != (byte) (i % 256)) {
                        match = false;
                        break;
                    }
                }
                serverResults[1] = "Server: data integrity: " + match;

                // Echo back the size
                OutputStream out = client.getOutputStream();
                out.write(String.valueOf(totalRead).getBytes());
                out.flush();

                client.close();
                serverSocket.close();
            } catch (Exception e) {
                serverResults[0] = "Server error: " + e.getMessage();
            }
        });
        serverThread.start();

        Thread.sleep(100);

        Socket socket = new Socket("127.0.0.1", port);
        OutputStream out = socket.getOutputStream();
        out.write(sendData);
        out.flush();

        InputStream in = socket.getInputStream();
        byte[] buf = new byte[1024];
        int bytesRead = in.read(buf);
        String response = new String(buf, 0, bytesRead);

        socket.close();
        serverThread.join();

        // Print results in deterministic order
        for (String r : serverResults) {
            if (r != null) System.out.println(r);
        }
        System.out.println("Client: sent " + dataSize + " bytes");
        System.out.println("Client: server confirmed " + response + " bytes");

        System.out.println("=== TCP Large Transfer Test Complete ===");
    }
}
