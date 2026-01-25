import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;

public class Test {
    public static void main(String[] args) throws InterruptedException {
        System.out.println("=== Mixed I/O Test ===");

        final Object lock = new Object();
        final boolean[] fileWritten = { false };

        Thread ioThread = new Thread(() -> {
            synchronized (lock) {
                System.out.println("IO Thread: Acquired lock, writing file...");
                try {
                    File f = new File("io_test.tmp");
                    try (FileOutputStream fos = new FileOutputStream(f)) {
                        String data = "Hello Ristretto IO";
                        // Simulate slow IO
                        for (char c : data.toCharArray()) {
                            fos.write(c);
                            try {
                                Thread.sleep(10);
                            } catch (InterruptedException e) {
                            }
                        }
                    }
                    fileWritten[0] = true;
                    // Notify while holding lock
                    lock.notify();
                    System.out.println("IO Thread: Notified, releasing lock upon exit");
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        });

        Thread waiterThread = new Thread(() -> {
            synchronized (lock) {
                System.out.println("Waiter Thread: Acquired lock, waiting...");
                try {
                    while (!fileWritten[0]) {
                        lock.wait();
                    }
                    System.out.println("Waiter Thread: Woke up, file written: " + fileWritten[0]);
                } catch (InterruptedException e) {
                    System.out.println("Waiter Thread: Interrupted");
                }
            }
        });

        waiterThread.start();
        // Give waiter time to acquire lock and wait
        Thread.sleep(100);
        ioThread.start();

        ioThread.join();
        waiterThread.join();

        // Cleanup
        new File("io_test.tmp").delete();

        System.out.println("Mixed I/O Test PASSED");
    }
}
