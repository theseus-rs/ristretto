import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.net.ServerSocket;
import java.net.Socket;
import java.net.SocketException;
import java.net.SocketTimeoutException;
import java.util.Arrays;
import java.util.concurrent.CountDownLatch;

public class Test {
    public static void main(String[] args) throws Exception {
        try {
            transferAndOptions();
            acceptTimeout();
            closeUnblocksRead();
            refusedConnection();
        } catch (Throwable error) {
            error.printStackTrace(System.out);
            throw new RuntimeException(error);
        }
    }

    private static void transferAndOptions() throws Exception {
        final ServerSocket server = new ServerSocket();
        server.setReuseAddress(true);
        server.bind(new InetSocketAddress(InetAddress.getByName("127.0.0.1"), 0));
        server.setSoTimeout(3000);
        final int[] urgent = {-1};
        final Throwable[] workerError = {null};
        final Socket[] acceptedHolder = {null};
        final CountDownLatch urgentReady = new CountDownLatch(1);
        Thread worker = new Thread(new Runnable() {
            public void run() {
                try {
                    Socket accepted = server.accept();
                    acceptedHolder[0] = accepted;
                    accepted.setSoTimeout(3000);
                    accepted.setOOBInline(true);
                    urgentReady.countDown();
                    urgent[0] = accepted.getInputStream().read();
                    accepted.getOutputStream().write(new byte[] {10, 20, 30, 40});
                    accepted.getOutputStream().flush();
                    while (accepted.getInputStream().read() != -1) {
                        // Drain until the client half-closes its output.
                    }
                    accepted.close();
                    server.close();
                } catch (Throwable error) {
                    workerError[0] = error;
                }
            }
        });
        worker.start();

        Socket client = new Socket();
        client.setTcpNoDelay(true);
        client.setKeepAlive(true);
        client.setReceiveBufferSize(8192);
        client.setSendBufferSize(8192);
        client.setSoTimeout(3000);
        client.connect(new InetSocketAddress("127.0.0.1", server.getLocalPort()), 2000);
        urgentReady.await();
        client.sendUrgentData(90);
        InputStream input = client.getInputStream();
        long deadline = System.currentTimeMillis() + 2000;
        while (input.available() == 0 && System.currentTimeMillis() < deadline) {
            Thread.sleep(5);
        }
        byte[] target = new byte[] {-1, -1, -1, -1, -1, -1, -1, -1};
        int read = input.read(target, 2, 4);
        client.shutdownOutput();
        worker.join(5000);
        if (worker.isAlive()) {
            client.close();
            if (acceptedHolder[0] != null) {
                acceptedHolder[0].close();
            }
            server.close();
            throw new RuntimeException("TCP worker did not terminate");
        }
        if (workerError[0] != null) {
            throw new RuntimeException(workerError[0]);
        }
        System.out.println("tcp-transfer:" + read + ":" + Arrays.toString(target));
        System.out.println("tcp-urgent:" + urgent[0]);
        System.out.println("tcp-options:" + client.getTcpNoDelay() + ":" + client.getKeepAlive());
        System.out.println("tcp-addresses:" + client.getLocalAddress().getHostAddress() + ":"
                + client.getLocalAddress().isLoopbackAddress()
                + ":" + client.getInetAddress().isLoopbackAddress());
        client.close();
    }

    private static void acceptTimeout() throws Exception {
        ServerSocket server = new ServerSocket(0, 1, InetAddress.getByName("127.0.0.1"));
        server.setSoTimeout(50);
        boolean timedOut = false;
        try {
            server.accept();
        } catch (SocketTimeoutException expected) {
            timedOut = true;
        }
        server.close();
        System.out.println("tcp-accept-timeout:" + timedOut);
    }

    private static void closeUnblocksRead() throws Exception {
        final ServerSocket server = new ServerSocket(0, 1, InetAddress.getByName("127.0.0.1"));
        final Socket client = new Socket("127.0.0.1", server.getLocalPort());
        final Socket accepted = server.accept();
        final boolean[] unblocked = {false};
        Thread reader = new Thread(new Runnable() {
            public void run() {
                try {
                    client.getInputStream().read();
                } catch (SocketException expected) {
                    unblocked[0] = true;
                } catch (Exception error) {
                    throw new RuntimeException(error);
                }
            }
        });
        reader.start();
        Thread.sleep(25);
        client.close();
        reader.join(2000);
        accepted.close();
        server.close();
        System.out.println("tcp-close-unblocks:" + unblocked[0]);
    }

    private static void refusedConnection() throws Exception {
        ServerSocket reserve = new ServerSocket(0, 1, InetAddress.getByName("127.0.0.1"));
        int port = reserve.getLocalPort();
        reserve.close();
        Socket socket = new Socket();
        boolean refused = false;
        try {
            socket.connect(new InetSocketAddress("127.0.0.1", port), 1000);
        } catch (IOException expected) {
            refused = true;
        } finally {
            socket.close();
        }
        System.out.println("tcp-refused:" + refused);
    }
}
