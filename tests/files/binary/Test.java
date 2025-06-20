import java.io.*;
import java.nio.file.*;

public class Test {
    public static void main(String[] args) throws IOException {
        String filename = "test.bin";
        String content = "Hello, world!\nThis is a test file.";

        try (RandomAccessFile raf = new RandomAccessFile(filename, "rw")) {
            byte[] data = content.getBytes("UTF-8");
            raf.write(data);
            raf.seek(0);
            raf.seek(7); // after "Hello, "
            raf.write("Java".getBytes("UTF-8"));
        }

        byte[] readData;
        try (RandomAccessFile raf = new RandomAccessFile(filename, "r")) {
            readData = new byte[(int) raf.length()];
            raf.seek(0);
            raf.readFully(readData);
        }

        System.out.println("Read content:");
        System.out.println(new String(readData, "UTF-8"));

//         Files.delete(Paths.get(filename));
//         System.out.print("File deleted: ");
//         System.out.println(!Files.exists(Paths.get(filename)));
    }
}
