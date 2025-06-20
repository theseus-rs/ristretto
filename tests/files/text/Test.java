import java.io.*;
import java.nio.file.*;

public class Test {
    public static void main(String[] args) throws IOException {
        String filename = "test.txt";
        String content = "Hello, world!\nThis is a test file.";

        try (BufferedWriter writer = new BufferedWriter(new FileWriter(filename))) {
            writer.write(content);
        }

        StringBuilder sb = new StringBuilder();
        try (BufferedReader reader = new BufferedReader(new FileReader(filename))) {
            String line;
            while ((line = reader.readLine()) != null) {
                sb.append(line).append('\n');
            }
        }
        System.out.println("Read content:");
        System.out.println(sb);

//         Files.delete(Paths.get(filename));
//         System.out.print("File deleted: ");
//         System.out.println(!Files.exists(Paths.get(filename)));
    }
}
