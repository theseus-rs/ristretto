import java.util.zip.ZipEntry;

public class Test {
    public static void main(String[] args) {
        testZipEntry();
        System.out.println("Test completed successfully");
    }

    private static void testZipEntry() {
        System.out.println("Testing ZipEntry basic functionality");

        // Test constructor with name
        ZipEntry entry = new ZipEntry("test.txt");
        System.out.println("Entry name: " + entry.getName());
        System.out.println("Is directory: " + entry.isDirectory());

        // Test directory entry
        ZipEntry dirEntry = new ZipEntry("testdir/");
        System.out.println("Dir entry name: " + dirEntry.getName());
        System.out.println("Dir is directory: " + dirEntry.isDirectory());

        // Test setting properties
        entry.setSize(100);
        System.out.println("Size: " + entry.getSize());

        entry.setCompressedSize(50);
        System.out.println("Compressed size: " + entry.getCompressedSize());

        entry.setCrc(12345678L);
        System.out.println("CRC: " + entry.getCrc());

        entry.setMethod(ZipEntry.DEFLATED);
        System.out.println("Method: " + entry.getMethod());

        entry.setComment("Test comment");
        System.out.println("Comment: " + entry.getComment());

        // Test extra field
        byte[] extra = {1, 2, 3, 4};
        entry.setExtra(extra);
        byte[] retrievedExtra = entry.getExtra();
        System.out.println("Extra length: " + (retrievedExtra != null ? retrievedExtra.length : 0));

        // Test copy constructor
        ZipEntry copy = new ZipEntry(entry);
        System.out.println("Copy name: " + copy.getName());
        System.out.println("Copy size: " + copy.getSize());
    }
}
