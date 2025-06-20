import java.io.*;

public class Test {
    public static void main(String[] args) throws IOException {
        String filename = "test.txt";
        File file = new File(filename);
        String content = "Hello, world!\nThis is a test file.";

        try (BufferedWriter writer = new BufferedWriter(new FileWriter(file))) {
            writer.write(content);
        }

        StringBuilder sb = new StringBuilder();
        try (BufferedReader reader = new BufferedReader(new FileReader(file))) {
            String line;
            while ((line = reader.readLine()) != null) {
                sb.append(line).append('\n');
            }
        }
        System.out.println("Read content:");
        System.out.println(sb);

        file.delete();
        System.out.print("File deleted: ");
        System.out.println(!file.exists());
    }
}
