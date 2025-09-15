import java.nio.*;
import java.nio.channels.*;
import java.io.*;
import java.util.Arrays;

/**
 * Tests for java.nio memory-mapped files and advanced channel operations
 */
public class Test {
    public static void main(String[] args) {
        System.out.println("=== Java NIO Memory-Mapped Files Tests ===");

        testMemoryMappedFiles();
        testFileLocking();
        testScatteringGatheringChannels();
        testChannelTransfer();
        testPipeChannels();
        testChannelPosition();

        // Cleanup any test files
        cleanup();

        System.out.println("=== Memory-Mapped Files Tests Complete ===");
    }

    private static void testMemoryMappedFiles() {
        System.out.println("--- Memory-Mapped Files Tests ---");

        File testFile = new File("test_mmap.txt");
        String testData = "Memory-mapped file test data. This is a longer string to test memory mapping.";

        try {
            // Create and write test file
            RandomAccessFile raf = new RandomAccessFile(testFile, "rw");
            FileChannel channel = raf.getChannel();

            // Write initial data
            ByteBuffer writeBuffer = ByteBuffer.wrap(testData.getBytes());
            channel.write(writeBuffer);
            channel.force(false);

            long fileSize = channel.size();
            System.out.println("File size after write: " + fileSize);
            System.out.println("Expected size: " + testData.getBytes().length);

            // Test read-only memory mapping
            MappedByteBuffer readOnlyMap = channel.map(FileChannel.MapMode.READ_ONLY, 0, fileSize);
            System.out.println("Read-only mapped buffer created: " + (readOnlyMap != null));
            System.out.println("Mapped buffer capacity: " + readOnlyMap.capacity());
            System.out.println("Mapped buffer is direct: " + readOnlyMap.isDirect());

            // Read from mapped buffer
            byte[] readData = new byte[(int) fileSize];
            readOnlyMap.get(readData);
            String mappedReadData = new String(readData);
            System.out.println("Original data: '" + testData + "'");
            System.out.println("Mapped read data: '" + mappedReadData + "'");
            System.out.println("Memory-mapped read matches: " + testData.equals(mappedReadData));

            // Test read-write memory mapping
            MappedByteBuffer readWriteMap = channel.map(FileChannel.MapMode.READ_WRITE, 0, fileSize);
            System.out.println("Read-write mapped buffer created: " + (readWriteMap != null));

            // Modify data through mapped buffer
            readWriteMap.position(0);
            readWriteMap.put("MODIFIED".getBytes());
            readWriteMap.force();

            // Verify modification by reading through channel
            channel.position(0);
            ByteBuffer verifyBuffer = ByteBuffer.allocate(8);
            channel.read(verifyBuffer);
            verifyBuffer.flip();
            String modifiedData = new String(verifyBuffer.array(), 0, verifyBuffer.limit());
            System.out.println("Modified data read from channel: '" + modifiedData + "'");
            System.out.println("Modification successful: " + "MODIFIED".equals(modifiedData));

            // Test private memory mapping (copy-on-write)
            channel.position(0);
            MappedByteBuffer privateMap = channel.map(FileChannel.MapMode.PRIVATE, 0, fileSize);
            System.out.println("Private mapped buffer created: " + (privateMap != null));

            // Modify private map (should not affect file)
            privateMap.position(0);
            privateMap.put("PRIVATE".getBytes());

            // Verify file unchanged by reading original position
            channel.position(0);
            ByteBuffer checkBuffer = ByteBuffer.allocate(8);
            channel.read(checkBuffer);
            checkBuffer.flip();
            String unchangedData = new String(checkBuffer.array(), 0, checkBuffer.limit());
            System.out.println("File data after private modification: '" + unchangedData + "'");
            System.out.println("Private mapping isolation: " + "MODIFIED".equals(unchangedData));

            // Test loading mapped buffer
            try {
                readOnlyMap.load();
                System.out.println("Mapped buffer load operation: completed");
            } catch (Exception e) {
                System.out.println("Mapped buffer load not supported: " + e.getMessage());
            }

            // Test isLoaded
            boolean isLoaded = readOnlyMap.isLoaded();
            System.out.println("Mapped buffer isLoaded: " + isLoaded);

            channel.close();
            raf.close();

        } catch (Exception e) {
            System.out.println("ERROR in memory-mapped files test: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void testFileLocking() {
        System.out.println("--- File Locking Tests ---");

        File testFile = new File("test_lock.txt");
        String testData = "File locking test data";

        try {
            RandomAccessFile raf = new RandomAccessFile(testFile, "rw");
            FileChannel channel = raf.getChannel();

            // Write test data
            channel.write(ByteBuffer.wrap(testData.getBytes()));
            channel.position(0);

            // Test exclusive lock
            FileLock exclusiveLock = channel.lock();
            System.out.println("Exclusive lock acquired: " + (exclusiveLock != null));
            System.out.println("Lock is valid: " + exclusiveLock.isValid());
            System.out.println("Lock is exclusive: " + !exclusiveLock.isShared());
            System.out.println("Lock channel matches: " + (exclusiveLock.channel() == channel));
            System.out.println("Lock position: " + exclusiveLock.position());
            System.out.println("Lock size: " + exclusiveLock.size());
            System.out.println("Channel size: " + channel.size());

            // Test overlaps
            System.out.println("Lock overlaps with itself: " + exclusiveLock.overlaps(0, channel.size()));
            System.out.println("Lock overlaps beyond size: " + exclusiveLock.overlaps(channel.size() + 1, 10));

            // Release lock
            exclusiveLock.release();
            System.out.println("Lock valid after release: " + exclusiveLock.isValid());

            // Test partial lock
            long lockStart = 5;
            long lockSize = 10;
            FileLock partialLock = channel.lock(lockStart, lockSize, false);
            System.out.println("Partial lock acquired: " + (partialLock != null));
            System.out.println("Partial lock position: " + partialLock.position());
            System.out.println("Partial lock size: " + partialLock.size());
            System.out.println("Partial lock is shared: " + partialLock.isShared());

            partialLock.release();

            // Test shared lock
            FileLock sharedLock = channel.lock(0, channel.size(), true);
            System.out.println("Shared lock acquired: " + (sharedLock != null));
            System.out.println("Shared lock is shared: " + sharedLock.isShared());

            sharedLock.release();

            // Test try lock (non-blocking)
            FileLock tryLock = channel.tryLock();
            System.out.println("Try lock successful: " + (tryLock != null));
            System.out.println("Try lock is valid: " + tryLock.isValid());

            tryLock.release();

            // Test try lock with parameters
            FileLock tryPartialLock = channel.tryLock(0, 5, false);
            System.out.println("Try partial lock successful: " + (tryPartialLock != null));

            tryPartialLock.release();

            channel.close();
            raf.close();

        } catch (Exception e) {
            System.out.println("ERROR in file locking test: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void testScatteringGatheringChannels() {
        System.out.println("--- Scattering/Gathering Channels Tests ---");

        File testFile = new File("test_scatter_gather.txt");

        try {
            RandomAccessFile raf = new RandomAccessFile(testFile, "rw");
            FileChannel channel = raf.getChannel();

            // Test gathering write
            ByteBuffer buffer1 = ByteBuffer.wrap("Hello ".getBytes());
            ByteBuffer buffer2 = ByteBuffer.wrap("World ".getBytes());
            ByteBuffer buffer3 = ByteBuffer.wrap("from ".getBytes());
            ByteBuffer buffer4 = ByteBuffer.wrap("NIO!".getBytes());

            ByteBuffer[] writeBuffers = {buffer1, buffer2, buffer3, buffer4};
            long bytesWritten = channel.write(writeBuffers);

            String expectedData = "Hello World from NIO!";
            System.out.println("Expected data: '" + expectedData + "'");
            System.out.println("Bytes written by gathering: " + bytesWritten);
            System.out.println("Expected bytes: " + expectedData.getBytes().length);
            System.out.println("Channel size after write: " + channel.size());

            // Test scattering read
            channel.position(0);
            ByteBuffer readBuffer1 = ByteBuffer.allocate(6);
            ByteBuffer readBuffer2 = ByteBuffer.allocate(6);
            ByteBuffer readBuffer3 = ByteBuffer.allocate(5);
            ByteBuffer readBuffer4 = ByteBuffer.allocate(4);

            ByteBuffer[] readBuffers = {readBuffer1, readBuffer2, readBuffer3, readBuffer4};
            long bytesRead = channel.read(readBuffers);

            System.out.println("Bytes read by scattering: " + bytesRead);

            // Verify scattered data
            readBuffer1.flip();
            readBuffer2.flip();
            readBuffer3.flip();
            readBuffer4.flip();

            String part1 = new String(readBuffer1.array(), 0, readBuffer1.limit());
            String part2 = new String(readBuffer2.array(), 0, readBuffer2.limit());
            String part3 = new String(readBuffer3.array(), 0, readBuffer3.limit());
            String part4 = new String(readBuffer4.array(), 0, readBuffer4.limit());

            System.out.println("Scattered part 1: '" + part1 + "'");
            System.out.println("Scattered part 2: '" + part2 + "'");
            System.out.println("Scattered part 3: '" + part3 + "'");
            System.out.println("Scattered part 4: '" + part4 + "'");

            String reconstructed = part1 + part2 + part3 + part4;
            System.out.println("Reconstructed: '" + reconstructed + "'");
            System.out.println("Scattering/gathering round-trip: " + expectedData.equals(reconstructed));

            // Test gathering write with offset/length
            buffer1.clear();
            buffer2.clear();
            buffer3.clear();
            buffer4.clear();

            buffer1.put("HELLO ".getBytes());
            buffer2.put("WORLD ".getBytes());
            buffer3.put("AGAIN ".getBytes());
            buffer4.put("TEST".getBytes());

            buffer1.flip();
            buffer2.flip();
            buffer3.flip();
            buffer4.flip();

            channel.position(0);
            long bytesWritten2 = channel.write(writeBuffers, 1, 2); // Write only buffer2 and buffer3

            System.out.println("Partial gathering write bytes: " + bytesWritten2);
            System.out.println("Expected partial bytes: " + "WORLD AGAIN ".getBytes().length);

            channel.close();
            raf.close();

        } catch (Exception e) {
            System.out.println("ERROR in scattering/gathering test: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void testChannelTransfer() {
        System.out.println("--- Channel Transfer Tests ---");

        File sourceFile = new File("test_source.txt");
        File targetFile = new File("test_target.txt");
        String testData = "Channel transfer test data. This should be transferred efficiently.";

        try {
            // Create source file
            FileOutputStream fos = new FileOutputStream(sourceFile);
            fos.write(testData.getBytes());
            fos.close();

            // Open channels
            FileInputStream fis = new FileInputStream(sourceFile);
            FileChannel sourceChannel = fis.getChannel();

            RandomAccessFile raf = new RandomAccessFile(targetFile, "rw");
            FileChannel targetChannel = raf.getChannel();

            System.out.println("Test data: '" + testData + "'");
            System.out.println("Source file size: " + sourceChannel.size());

            // Test transferTo
            long transferred = sourceChannel.transferTo(0, sourceChannel.size(), targetChannel);
            System.out.println("TransferTo bytes transferred: " + transferred);
            System.out.println("Target file size after transferTo: " + targetChannel.size());

            // Verify transfer content
            targetChannel.position(0);
            ByteBuffer verifyBuffer = ByteBuffer.allocate((int) targetChannel.size());
            targetChannel.read(verifyBuffer);
            verifyBuffer.flip();
            String transferredData = new String(verifyBuffer.array(), 0, verifyBuffer.limit());
            System.out.println("Transferred data: '" + transferredData + "'");
            System.out.println("TransferTo content matches: " + testData.equals(transferredData));

            // Clear target for transferFrom test
            targetChannel.truncate(0);
            sourceChannel.position(0);

            // Test transferFrom
            long transferredFrom = targetChannel.transferFrom(sourceChannel, 0, sourceChannel.size());
            System.out.println("TransferFrom bytes transferred: " + transferredFrom);
            System.out.println("Target file size after transferFrom: " + targetChannel.size());

            // Verify transferFrom content
            targetChannel.position(0);
            verifyBuffer.clear();
            targetChannel.read(verifyBuffer);
            verifyBuffer.flip();
            String transferredFromData = new String(verifyBuffer.array(), 0, verifyBuffer.limit());
            System.out.println("TransferFrom data: '" + transferredFromData + "'");
            System.out.println("TransferFrom content matches: " + testData.equals(transferredFromData));

            // Test partial transfer
            targetChannel.truncate(0);
            sourceChannel.position(0);
            long partialSize = 10;
            long partialTransferred = targetChannel.transferFrom(sourceChannel, 0, partialSize);
            System.out.println("Partial transfer requested: " + partialSize);
            System.out.println("Partial transfer actual: " + partialTransferred);
            System.out.println("Partial transfer target size: " + targetChannel.size());

            sourceChannel.close();
            targetChannel.close();
            fis.close();
            raf.close();

        } catch (Exception e) {
            System.out.println("ERROR in channel transfer test: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void testPipeChannels() {
        System.out.println("--- Pipe Channels Tests ---");

        try {
            Pipe pipe = Pipe.open();
            System.out.println("Pipe created: " + (pipe != null));

            Pipe.SourceChannel sourceChannel = pipe.source();
            Pipe.SinkChannel sinkChannel = pipe.sink();

            System.out.println("Source channel created: " + (sourceChannel != null));
            System.out.println("Sink channel created: " + (sinkChannel != null));
            System.out.println("Source is readable: " + (sourceChannel instanceof ReadableByteChannel));
            System.out.println("Sink is writable: " + (sinkChannel instanceof WritableByteChannel));

            // Test pipe communication
            String pipeData = "Pipe communication test";
            ByteBuffer writeBuffer = ByteBuffer.wrap(pipeData.getBytes());

            int bytesWritten = sinkChannel.write(writeBuffer);
            System.out.println("Pipe data: '" + pipeData + "'");
            System.out.println("Bytes written to pipe: " + bytesWritten);
            System.out.println("Expected bytes: " + pipeData.getBytes().length);

            ByteBuffer readBuffer = ByteBuffer.allocate(pipeData.getBytes().length);
            int bytesRead = sourceChannel.read(readBuffer);
            System.out.println("Bytes read from pipe: " + bytesRead);

            readBuffer.flip();
            String readData = new String(readBuffer.array(), 0, readBuffer.limit());
            System.out.println("Read data: '" + readData + "'");
            System.out.println("Pipe data integrity: " + pipeData.equals(readData));

            // Test channel states
            System.out.println("Source channel open: " + sourceChannel.isOpen());
            System.out.println("Sink channel open: " + sinkChannel.isOpen());

            // Close channels
            sourceChannel.close();
            sinkChannel.close();

            System.out.println("Source channel closed: " + !sourceChannel.isOpen());
            System.out.println("Sink channel closed: " + !sinkChannel.isOpen());

        } catch (Exception e) {
            System.out.println("ERROR in pipe channels test: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void testChannelPosition() {
        System.out.println("--- Channel Position Tests ---");

        File testFile = new File("test_position.txt");
        String testData = "0123456789ABCDEFGHIJ";

        try {
            RandomAccessFile raf = new RandomAccessFile(testFile, "rw");
            FileChannel channel = raf.getChannel();

            // Write test data
            channel.write(ByteBuffer.wrap(testData.getBytes()));
            System.out.println("Test data: '" + testData + "'");
            System.out.println("Channel size: " + channel.size());
            System.out.println("Channel position after write: " + channel.position());

            // Test position manipulation
            channel.position(5);
            System.out.println("Set position to 5: " + channel.position());

            // Read from position
            ByteBuffer buffer = ByteBuffer.allocate(5);
            int bytesRead = channel.read(buffer);
            System.out.println("Bytes read from position 5: " + bytesRead);
            System.out.println("Position after read: " + channel.position());

            buffer.flip();
            String readData = new String(buffer.array(), 0, buffer.limit());
            System.out.println("Data read from position 5: '" + readData + "'");
            System.out.println("Expected from position 5: '56789'");
            System.out.println("Position read matches: " + "56789".equals(readData));

            // Test write at position
            channel.position(15);
            ByteBuffer writeBuffer = ByteBuffer.wrap("XYZ".getBytes());
            int bytesWritten = channel.write(writeBuffer);
            System.out.println("Bytes written at position 15: " + bytesWritten);
            System.out.println("Position after write: " + channel.position());

            // Verify write
            channel.position(15);
            buffer.clear();
            buffer.limit(3);
            channel.read(buffer);
            buffer.flip();
            String modifiedData = new String(buffer.array(), 0, buffer.limit());
            System.out.println("Modified data at position 15: '" + modifiedData + "'");
            System.out.println("Write at position successful: " + "XYZ".equals(modifiedData));

            // Test truncate
            long originalSize = channel.size();
            channel.truncate(10);
            System.out.println("Size before truncate: " + originalSize);
            System.out.println("Size after truncate: " + channel.size());

            // Position should be adjusted if beyond new size
            channel.position(15); // Try to set beyond truncated size
            long finalPosition = channel.position();
            System.out.println("Position after setting beyond truncated size: " + finalPosition);
            System.out.println("Position adjusted correctly: " + (finalPosition <= 10));

            channel.close();
            raf.close();

        } catch (Exception e) {
            System.out.println("ERROR in channel position test: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void cleanup() {
        String[] testFiles = {
            "test_mmap.txt",
            "test_lock.txt",
            "test_scatter_gather.txt",
            "test_source.txt",
            "test_target.txt",
            "test_position.txt"
        };

        System.out.println("Cleaning up test files...");
        for (String filename : testFiles) {
            File file = new File(filename);
            if (file.exists()) {
                boolean deleted = file.delete();
                System.out.println("Cleaned up " + filename + ": " + deleted);
            }
        }
    }
}
