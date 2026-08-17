import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.Map;
import java.util.jar.JarEntry;
import java.util.jar.JarInputStream;
import java.util.jar.JarOutputStream;
import java.util.jar.Pack200;
import java.util.zip.CRC32;

/** Integration coverage for com.sun.java.util.jar.pack.NativeUnpack. */
public class Test {
    private static final byte[] PACKED_ARCHIVE = decodeHex(
        "cafed00d079690009300d1c8cadd6702030000000000000000032d00000a1269676e6f7265"
            + "2e74787472756e74696d652e70726f7065727469657302011a4100016a6176612e76657273"
            + "696f6e733d31312c31372c32312c32350afffe520065007100750069007200650073002000"
            + "660075006c006c00200043004c00440052002f00490043005500200073007500700070006f"
            + "0072007400000a00"
    );
    private static final byte[] STORED_CONTENT =
        "java.versions=11,17,21,25\n".getBytes(StandardCharsets.UTF_8);

    public static void main(String[] args) throws Throwable {
        try {
            testPublicInputStreamUnpack();
            System.out.println("Pack200 native unpack integration: passed");
        } catch (Throwable error) {
            System.out.println("Pack200 integration failure: " + describe(error));
            throw error;
        }
    }

    private static String describe(Throwable error) {
        StringBuilder description = new StringBuilder();
        while (error != null) {
            if (description.length() != 0) {
                description.append(" caused by ");
            }
            description
                .append(error.getClass().getName())
                .append(": ")
                .append(error.getMessage());
            Throwable cause;
            if (error instanceof ExceptionInInitializerError) {
                cause = ((ExceptionInInitializerError) error).getException();
            } else {
                cause = error.getCause();
            }
            error = cause;
        }
        return description.toString();
    }

    private static byte[] decodeHex(String hex) {
        byte[] bytes = new byte[hex.length() / 2];
        for (int index = 0; index < bytes.length; index++) {
            int high = hexDigit(hex.charAt(index * 2));
            int low = hexDigit(hex.charAt(index * 2 + 1));
            bytes[index] = (byte) ((high << 4) | low);
        }
        return bytes;
    }

    private static int hexDigit(char value) {
        if (value >= '0' && value <= '9') {
            return value - '0';
        }
        return value - 'a' + 10;
    }

    /** Exercises the public path that lets NativeUnpack pull bytes from its input stream. */
    private static void testPublicInputStreamUnpack() throws Exception {
        CloseTrackingInputStream input = new CloseTrackingInputStream(PACKED_ARCHIVE);
        Pack200.Unpacker unpacker = Pack200.newUnpacker();
        ByteArrayOutputStream jarBytes = new ByteArrayOutputStream();

        try (JarOutputStream output = new JarOutputStream(jarBytes)) {
            unpacker.unpack(input, output);
        }

        check(input.closed, "the unpacker must close its input stream");
        check(
            input.readObserved,
            "the unpacker must read its input stream"
        );
        verifyEntries(readJar(jarBytes.toByteArray()));
    }

    private static Map<String, EntryData> readJar(byte[] jarBytes) throws Exception {
        Map<String, EntryData> entries = new HashMap<String, EntryData>();
        try (JarInputStream input = new JarInputStream(new ByteArrayInputStream(jarBytes))) {
            JarEntry entry;
            while ((entry = input.getNextJarEntry()) != null) {
                ByteArrayOutputStream contents = new ByteArrayOutputStream();
                byte[] buffer = new byte[64];
                int count;
                while ((count = input.read(buffer)) != -1) {
                    contents.write(buffer, 0, count);
                }
                entries.put(
                    entry.getName(),
                    new EntryData(contents.toByteArray(), entry.getMethod() == JarEntry.DEFLATED)
                );
            }
        }
        return entries;
    }

    private static void verifyEntries(Map<String, EntryData> entries) {
        check(entries.size() == 2, "archive entry count");

        EntryData stored = entries.get("runtime.properties");
        check(stored != null, "stored entry must exist");
        check(!stored.deflated, "stored entry compression hint");
        check(java.util.Arrays.equals(STORED_CONTENT, stored.contents), "stored entry contents");

        EntryData deflated = entries.get("ignore.txt");
        check(deflated != null, "deflated entry must exist");
        check(deflated.deflated, "deflated entry compression hint");
        CRC32 crc = new CRC32();
        crc.update(deflated.contents);
        check(deflated.contents.length == 65, "deflated entry size");
        check(crc.getValue() == 0x1b821a1fL, "deflated entry contents");
    }

    private static void check(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }

    private static final class EntryData {
        private final byte[] contents;
        private final boolean deflated;

        private EntryData(byte[] contents, boolean deflated) {
            this.contents = contents;
            this.deflated = deflated;
        }
    }

    private static final class CloseTrackingInputStream extends ByteArrayInputStream {
        private boolean closed;
        private boolean readObserved;

        private CloseTrackingInputStream(byte[] contents) {
            super(contents);
        }

        @Override
        public synchronized int read(byte[] buffer, int offset, int length) {
            readObserved = true;
            return super.read(buffer, offset, Math.min(length, 4));
        }

        @Override
        public synchronized int read() {
            readObserved = true;
            return super.read();
        }

        @Override
        public void close() {
            closed = true;
        }
    }
}
