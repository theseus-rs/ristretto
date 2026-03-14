import java.net.ServerSocket;
import java.net.Socket;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.DataInputStream;
import java.io.DataOutputStream;

/**
 * Test stream wrappers over socket I/O.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== TCP Streams Test ===");

        // Test 1: BufferedReader / PrintWriter (line-oriented)
        testLineOriented();

        // Test 2: DataInputStream / DataOutputStream (typed data)
        testTypedData();

        System.out.println("=== TCP Streams Test Complete ===");
    }

    static void testLineOriented() throws Exception {
        System.out.println("--- Line-oriented I/O ---");
        ServerSocket serverSocket = new ServerSocket(0);
        int port = serverSocket.getLocalPort();

        Thread serverThread = new Thread(() -> {
            try {
                Socket client = serverSocket.accept();
                BufferedReader reader = new BufferedReader(
                    new InputStreamReader(client.getInputStream()));
                PrintWriter writer = new PrintWriter(client.getOutputStream(), true);

                String line1 = reader.readLine();
                String line2 = reader.readLine();
                System.out.println("Server line 1: " + line1);
                System.out.println("Server line 2: " + line2);

                writer.println("Reply 1");
                writer.println("Reply 2");

                client.close();
                serverSocket.close();
            } catch (Exception e) {
                System.out.println("Server error: " + e.getMessage());
            }
        });
        serverThread.start();

        Thread.sleep(100);

        Socket socket = new Socket("127.0.0.1", port);
        PrintWriter writer = new PrintWriter(socket.getOutputStream(), true);
        BufferedReader reader = new BufferedReader(
            new InputStreamReader(socket.getInputStream()));

        writer.println("Hello World");
        writer.println("Second Line");

        String reply1 = reader.readLine();
        String reply2 = reader.readLine();
        System.out.println("Client reply 1: " + reply1);
        System.out.println("Client reply 2: " + reply2);

        socket.close();
        serverThread.join();
    }

    static void testTypedData() throws Exception {
        System.out.println("--- Typed data I/O ---");
        ServerSocket serverSocket = new ServerSocket(0);
        int port = serverSocket.getLocalPort();

        Thread serverThread = new Thread(() -> {
            try {
                Socket client = serverSocket.accept();
                DataInputStream dis = new DataInputStream(client.getInputStream());
                DataOutputStream dos = new DataOutputStream(client.getOutputStream());

                int intVal = dis.readInt();
                long longVal = dis.readLong();
                boolean boolVal = dis.readBoolean();
                String strVal = dis.readUTF();

                System.out.println("Server int: " + intVal);
                System.out.println("Server long: " + longVal);
                System.out.println("Server boolean: " + boolVal);
                System.out.println("Server string: " + strVal);

                dos.writeInt(intVal * 2);
                dos.writeUTF("Echo: " + strVal);
                dos.flush();

                client.close();
                serverSocket.close();
            } catch (Exception e) {
                System.out.println("Server error: " + e.getMessage());
            }
        });
        serverThread.start();

        Thread.sleep(100);

        Socket socket = new Socket("127.0.0.1", port);
        DataOutputStream dos = new DataOutputStream(socket.getOutputStream());
        DataInputStream dis = new DataInputStream(socket.getInputStream());

        dos.writeInt(42);
        dos.writeLong(123456789L);
        dos.writeBoolean(true);
        dos.writeUTF("TestData");
        dos.flush();

        int replyInt = dis.readInt();
        String replyStr = dis.readUTF();
        System.out.println("Client reply int: " + replyInt);
        System.out.println("Client reply string: " + replyStr);

        socket.close();
        serverThread.join();
    }
}
