import java.nio.*;
import java.nio.channels.*;
import java.io.*;
import java.util.Arrays;

/**
 * Tests for java.nio.channels
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Java NIO Channels Tests ===");

        testFileChannel();
        testReadableByteChannel();
        testWritableByteChannel();
        testByteChannel();
        testChannelProperties();

        // Cleanup any test files
        cleanup();

        System.out.println("=== Channels Tests Complete ===");
    }

    private static void testFileChannel() {
        System.out.println("--- FileChannel Tests ---");

        File testFile = new File("test_channel.txt");
        String testData = "Hello, NIO FileChannel!";

        try {
            // Test writing
            FileOutputStream fos = new FileOutputStream(testFile);
            FileChannel writeChannel = fos.getChannel();

            System.out.println("FileChannel is writable: " + (writeChannel instanceof WritableByteChannel));
            System.out.println("FileChannel initial position: " + writeChannel.position());

            ByteBuffer writeBuffer = ByteBuffer.wrap(testData.getBytes());
            int bytesWritten = writeChannel.write(writeBuffer);
            System.out.println("Bytes written: " + bytesWritten);
            System.out.println("Expected bytes: " + testData.getBytes().length);
            System.out.println("Channel position after write: " + writeChannel.position());

            long size = writeChannel.size();
            System.out.println("Channel size: " + size);

            writeChannel.close();
            fos.close();

            // Test reading
            FileInputStream fis = new FileInputStream(testFile);
            FileChannel readChannel = fis.getChannel();

            System.out.println("FileChannel is readable: " + (readChannel instanceof ReadableByteChannel));
            System.out.println("Read channel size: " + readChannel.size());

            ByteBuffer readBuffer = ByteBuffer.allocate(testData.getBytes().length);
            int bytesRead = readChannel.read(readBuffer);
            System.out.println("Bytes read: " + bytesRead);

            readBuffer.flip();
            String readData = new String(readBuffer.array(), 0, readBuffer.limit());
            System.out.println("Written data: '" + testData + "'");
            System.out.println("Read data: '" + readData + "'");
            System.out.println("Data matches: " + testData.equals(readData));

            readChannel.close();
            fis.close();

            // Test random access
            RandomAccessFile raf = new RandomAccessFile(testFile, "rw");
            FileChannel rafChannel = raf.getChannel();

            // Test position manipulation
            rafChannel.position(6);
            System.out.println("Random access position: " + rafChannel.position());

            ByteBuffer partialRead = ByteBuffer.allocate(3);
            rafChannel.read(partialRead);
            partialRead.flip();
            String partial = new String(partialRead.array(), 0, partialRead.limit());
            System.out.println("Partial read from position 6: '" + partial + "'");

            // Test truncate
            long originalSize = rafChannel.size();
            rafChannel.truncate(5);
            System.out.println("Size before truncate: " + originalSize);
            System.out.println("Size after truncate: " + rafChannel.size());

            rafChannel.close();
            raf.close();

        } catch (Exception e) {
            System.out.println("ERROR in FileChannel test: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void testReadableByteChannel() {
        System.out.println("--- ReadableByteChannel Tests ---");

        File testFile = new File("test_readable.txt");
        String testData = "ReadableByteChannel test data";

        try {
            // Create test file
            FileOutputStream fos = new FileOutputStream(testFile);
            fos.write(testData.getBytes());
            fos.close();

            // Test ReadableByteChannel interface
            FileInputStream fis = new FileInputStream(testFile);
            ReadableByteChannel channel = fis.getChannel();

            System.out.println("Channel is open: " + channel.isOpen());

            ByteBuffer buffer = ByteBuffer.allocate(10);
            int bytesRead = channel.read(buffer);
            System.out.println("First read bytes: " + bytesRead);

            buffer.flip();
            String readData = new String(buffer.array(), 0, buffer.limit());
            System.out.println("First 10 chars: '" + readData + "'");
            System.out.println("Expected first 10: '" + testData.substring(0, 10) + "'");

            // Read remaining data
            buffer.clear();
            bytesRead = channel.read(buffer);
            System.out.println("Remaining bytes read: " + bytesRead);
            System.out.println("Expected remaining: " + (testData.getBytes().length - 10));

            // Test end of stream
            buffer.clear();
            bytesRead = channel.read(buffer);
            System.out.println("End of stream read result: " + bytesRead);

            channel.close();
            System.out.println("Channel closed, is open: " + channel.isOpen());
            fis.close();

        } catch (Exception e) {
            System.out.println("ERROR in ReadableByteChannel test: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void testWritableByteChannel() {
        System.out.println("--- WritableByteChannel Tests ---");

        File testFile = new File("test_writable.txt");
        String testData = "WritableByteChannel test";

        try {
            FileOutputStream fos = new FileOutputStream(testFile);
            WritableByteChannel channel = fos.getChannel();

            System.out.println("Writable channel is open: " + channel.isOpen());

            ByteBuffer buffer = ByteBuffer.wrap(testData.getBytes());
            int bytesWritten = channel.write(buffer);
            System.out.println("Bytes written: " + bytesWritten);
            System.out.println("Expected bytes: " + testData.getBytes().length);

            // Test partial write
            String moreData = " - additional data";
            buffer = ByteBuffer.wrap(moreData.getBytes());
            bytesWritten = channel.write(buffer);
            System.out.println("Additional bytes written: " + bytesWritten);

            channel.close();
            System.out.println("Channel closed, is open: " + channel.isOpen());
            fos.close();

            // Verify written data
            FileInputStream fis = new FileInputStream(testFile);
            byte[] readBytes = new byte[(int) testFile.length()];
            fis.read(readBytes);
            fis.close();

            String fullExpected = testData + moreData;
            String readData = new String(readBytes);
            System.out.println("Expected data: '" + fullExpected + "'");
            System.out.println("Read data: '" + readData + "'");
            System.out.println("Data matches: " + fullExpected.equals(readData));

        } catch (Exception e) {
            System.out.println("ERROR in WritableByteChannel test: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void testByteChannel() {
        System.out.println("--- ByteChannel Tests ---");

        File testFile = new File("test_bytechannel.txt");

        try {
            // Test that FileChannel implements ByteChannel
            RandomAccessFile raf = new RandomAccessFile(testFile, "rw");
            FileChannel channel = raf.getChannel();

            System.out.println("FileChannel is ByteChannel: " + (channel instanceof ByteChannel));
            System.out.println("ByteChannel is readable: " + (channel instanceof ReadableByteChannel));
            System.out.println("ByteChannel is writable: " + (channel instanceof WritableByteChannel));

            // Test read/write operations
            String writeData = "ByteChannel bidirectional test";
            ByteBuffer writeBuffer = ByteBuffer.wrap(writeData.getBytes());
            int written = channel.write(writeBuffer);
            System.out.println("Bytes written: " + written);
            System.out.println("Expected bytes: " + writeData.getBytes().length);

            // Reset position for reading
            channel.position(0);
            ByteBuffer readBuffer = ByteBuffer.allocate(writeData.getBytes().length);
            int read = channel.read(readBuffer);
            System.out.println("Bytes read: " + read);

            readBuffer.flip();
            String readData = new String(readBuffer.array(), 0, readBuffer.limit());
            System.out.println("Written data: '" + writeData + "'");
            System.out.println("Read data: '" + readData + "'");
            System.out.println("Data consistency: " + writeData.equals(readData));

            channel.close();
            raf.close();

        } catch (Exception e) {
            System.out.println("ERROR in ByteChannel test: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void testChannelProperties() {
        System.out.println("--- Channel Properties Tests ---");

        File testFile = new File("test_properties.txt");

        try {
            // Test channel states
            FileOutputStream fos = new FileOutputStream(testFile);
            FileChannel channel = fos.getChannel();

            System.out.println("New channel is open: " + channel.isOpen());

            channel.close();
            System.out.println("Closed channel is open: " + channel.isOpen());

            // Test operations on closed channel
            try {
                channel.size();
                System.out.println("ERROR: Operation on closed channel should throw exception");
            } catch (ClosedChannelException e) {
                System.out.println("Correctly caught ClosedChannelException: " + e.getClass().getSimpleName());
            }

            fos.close();

            // Test force (sync) operation
            RandomAccessFile raf = new RandomAccessFile(testFile, "rw");
            FileChannel writeChannel = raf.getChannel();

            writeChannel.write(ByteBuffer.wrap("Force test".getBytes()));

            try {
                writeChannel.force(false);
                System.out.println("Force without metadata sync: completed");

                writeChannel.force(true);
                System.out.println("Force with metadata sync: completed");
            } catch (Exception e) {
                System.out.println("Force operation error: " + e.getMessage());
            }

            writeChannel.close();
            raf.close();

        } catch (Exception e) {
            System.out.println("ERROR in Channel Properties test: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void cleanup() {
        String[] testFiles = {
            "test_channel.txt",
            "test_readable.txt",
            "test_writable.txt",
            "test_bytechannel.txt",
            "test_properties.txt"
        };

        for (String filename : testFiles) {
            File file = new File(filename);
            if (file.exists()) {
                boolean deleted = file.delete();
                System.out.println("Cleaned up " + filename + ": " + deleted);
            }
        }
    }
}
