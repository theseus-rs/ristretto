import java.util.zip.CheckedOutputStream;
import java.util.zip.CheckedInputStream;
import java.util.zip.CRC32;
import java.util.zip.Adler32;
import java.io.ByteArrayOutputStream;
import java.io.ByteArrayInputStream;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing CheckedOutputStream and CheckedInputStream");

        String data = "This is test data for checksum verification";
        byte[] dataBytes = data.getBytes();

        try {
            // Test CheckedOutputStream with CRC32
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            CRC32 crc32Out = new CRC32();
            CheckedOutputStream cos = new CheckedOutputStream(baos, crc32Out);

            cos.write(dataBytes);
            cos.close();

            System.out.println("CRC32 checksum (output): " + cos.getChecksum().getValue());

            // Test CheckedInputStream with CRC32
            ByteArrayInputStream bais = new ByteArrayInputStream(dataBytes);
            CRC32 crc32In = new CRC32();
            CheckedInputStream cis = new CheckedInputStream(bais, crc32In);

            byte[] buffer = new byte[1024];
            while (cis.read(buffer) != -1) {
                // Read all data
            }
            cis.close();

            System.out.println("CRC32 checksum (input): " + cis.getChecksum().getValue());
            System.out.println("Checksums match: " + (cos.getChecksum().getValue() == cis.getChecksum().getValue()));

            // Test with Adler32
            ByteArrayOutputStream baos2 = new ByteArrayOutputStream();
            Adler32 adler32Out = new Adler32();
            CheckedOutputStream cos2 = new CheckedOutputStream(baos2, adler32Out);

            cos2.write(dataBytes);
            cos2.close();

            System.out.println("Adler32 checksum (output): " + cos2.getChecksum().getValue());

            ByteArrayInputStream bais2 = new ByteArrayInputStream(dataBytes);
            Adler32 adler32In = new Adler32();
            CheckedInputStream cis2 = new CheckedInputStream(bais2, adler32In);

            while (cis2.read(buffer) != -1) {
                // Read all data
            }
            cis2.close();

            System.out.println("Adler32 checksum (input): " + cis2.getChecksum().getValue());
            System.out.println("Adler32 checksums match: " + (cos2.getChecksum().getValue() == cis2.getChecksum().getValue()));

        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
            e.printStackTrace();
        }

        System.out.println("Test completed successfully");
    }
}
