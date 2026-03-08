import java.net.InetSocketAddress;
import java.net.Socket;
import java.nio.ByteBuffer;
import java.nio.channels.ClosedChannelException;
import java.nio.channels.ServerSocketChannel;
import java.nio.channels.SocketChannel;

/**
 * Test NIO SocketChannel create, connect, close lifecycle.
 * Verifies that after close the file descriptor is fully evicted
 * from internal handle maps by checking channel state, socket state,
 * and confirming post-close operations raise the expected exceptions.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println("=== NIO Channel Close Test ===");

        // Start a server to accept connections
        ServerSocketChannel serverChannel = ServerSocketChannel.open();
        serverChannel.bind(new InetSocketAddress("127.0.0.1", 0));
        int port = ((InetSocketAddress) serverChannel.getLocalAddress()).getPort();
        System.out.println("Server bound: " + (port > 0));

        // Accept in a background thread so connect doesn't block
        String[] serverResults = new String[2];
        Thread serverThread = new Thread(() -> {
            try {
                SocketChannel accepted = serverChannel.accept();
                serverResults[0] = "Server: accepted connection";
                accepted.close();
                serverResults[1] = "Server: accepted channel closed";
            } catch (Exception e) {
                serverResults[0] = "Server error: " + e.getClass().getSimpleName() + ": " + e.getMessage();
            }
        });
        serverThread.start();

        Thread.sleep(100);

        // Open and connect a SocketChannel
        SocketChannel channel = SocketChannel.open();
        System.out.println("Channel open: " + channel.isOpen());
        System.out.println("Channel connected before connect: " + channel.isConnected());

        channel.connect(new InetSocketAddress("127.0.0.1", port));
        System.out.println("Channel connected: " + channel.isConnected());

        // Inspect state before close
        Socket socket = channel.socket();
        System.out.println("Socket closed before close: " + socket.isClosed());
        System.out.println("Socket connected before close: " + socket.isConnected());

        // Close the channel
        channel.close();

        // Verify channel state after close
        System.out.println("Channel open after close: " + channel.isOpen());
        System.out.println("Channel connected after close: " + channel.isConnected());

        // Verify socket state after close - FD should be evicted
        System.out.println("Socket closed after close: " + socket.isClosed());

        // Verify double-close is safe (no exception)
        boolean doubleCloseOk = true;
        try {
            channel.close();
        } catch (Exception e) {
            doubleCloseOk = false;
        }
        System.out.println("Double close safe: " + doubleCloseOk);

        // Verify read on closed channel throws ClosedChannelException
        boolean readThrew = false;
        try {
            channel.read(ByteBuffer.allocate(16));
        } catch (ClosedChannelException e) {
            readThrew = true;
        }
        System.out.println("Read after close threw ClosedChannelException: " + readThrew);

        // Verify write on closed channel throws ClosedChannelException
        boolean writeThrew = false;
        try {
            channel.write(ByteBuffer.wrap(new byte[]{1}));
        } catch (ClosedChannelException e) {
            writeThrew = true;
        }
        System.out.println("Write after close threw ClosedChannelException: " + writeThrew);

        // Verify connect on closed channel throws ClosedChannelException
        boolean connectThrew = false;
        try {
            channel.connect(new InetSocketAddress("127.0.0.1", port));
        } catch (ClosedChannelException e) {
            connectThrew = true;
        }
        System.out.println("Connect after close threw ClosedChannelException: " + connectThrew);

        // Clean up server
        serverThread.join(5000);
        serverChannel.close();

        for (String r : serverResults) {
            if (r != null) System.out.println(r);
        }

        System.out.println("Server channel closed: " + !serverChannel.isOpen());

        System.out.println("=== NIO Channel Close Test Complete ===");
    }
}
