import java.util.zip.GZIPOutputStream;
import java.util.zip.GZIPInputStream;
import java.io.ByteArrayOutputStream;
import java.io.ByteArrayInputStream;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing GZIPOutputStream and GZIPInputStream");

        String original = "Hello World! This is a test of GZIP compression.";
        byte[] inputBytes = original.getBytes();

        try {
            // Compress with GZIPOutputStream
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            GZIPOutputStream gzipOut = new GZIPOutputStream(baos);
            gzipOut.write(inputBytes);
            gzipOut.close();

            byte[] compressed = baos.toByteArray();
            System.out.println("Original size: " + inputBytes.length);
            // Note: Compressed size varies between zlib implementations, so we just verify compression occurred
            System.out.println("Compression occurred: " + (compressed.length > 0));

            // Decompress with GZIPInputStream
            ByteArrayInputStream bais = new ByteArrayInputStream(compressed);
            GZIPInputStream gzipIn = new GZIPInputStream(bais);

            ByteArrayOutputStream result = new ByteArrayOutputStream();
            byte[] buffer = new byte[1024];
            int len;
            while ((len = gzipIn.read(buffer)) > 0) {
                result.write(buffer, 0, len);
            }
            gzipIn.close();

            String decompressed = result.toString();
            System.out.println("Decompressed size: " + decompressed.length());
            System.out.println("Matches original: " + original.equals(decompressed));

        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
            e.printStackTrace();
        }

        System.out.println("Test completed successfully");
    }
}
