import java.io.*;
import java.nio.ByteBuffer;
import java.nio.channels.FileChannel;
import java.nio.charset.StandardCharsets;

/** Test interop between java.io and java.nio file channels. */
public class Test {
    public static void main(String[] args) throws IOException {
        String filename = "nio_interop_test_pos.txt";
        File file = new File(filename);

        // 1. Write using java.io.FileOutputStream
        try (FileOutputStream fos = new FileOutputStream(file)) {
            fos.write("Hello from java.io!\n".getBytes(StandardCharsets.UTF_8));
            
            // 2. Write using java.nio.channels.FileChannel from FileOutputStream
            FileChannel channel = fos.getChannel();
            ByteBuffer buffer = ByteBuffer.wrap("Hello from java.nio!\n".getBytes(StandardCharsets.UTF_8));
            channel.write(buffer);
            
            // 3. Positional write
            buffer.clear();
            buffer.put("POS\n".getBytes(StandardCharsets.UTF_8));
            buffer.flip();
            channel.write(buffer, 0); // Overwrite start
        }

        // 4. Read using java.nio.channels.FileChannel from FileInputStream
        try (FileInputStream fis = new FileInputStream(file)) {
            FileChannel channel = fis.getChannel();
            
            // Positional read
            ByteBuffer posBuffer = ByteBuffer.allocate(4);
            channel.read(posBuffer, 0);
            posBuffer.flip();
            System.out.println("Positional read: " + new String(posBuffer.array(), StandardCharsets.UTF_8).trim());
            
            ByteBuffer buffer = ByteBuffer.allocate(15);
            int bytesRead = channel.read(buffer);
            System.out.println("NIO read " + bytesRead + " bytes:");
            buffer.flip();
            byte[] bytes = new byte[buffer.remaining()];
            buffer.get(bytes);
            System.out.print(new String(bytes, StandardCharsets.UTF_8));

            // 5. Read remaining using java.io.FileInputStream
            System.out.println("IO read remaining:");
            BufferedReader reader = new BufferedReader(new InputStreamReader(fis, StandardCharsets.UTF_8));
            String line;
            while ((line = reader.readLine()) != null) {
                System.out.println(line);
            }
        }

        System.out.println("File length: " + file.length());

        if (file.delete()) {
            System.out.println("File deleted successfully");
        } else {
            System.out.println("Failed to delete file");
        }
    }
}
