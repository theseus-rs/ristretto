import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.util.Arrays;

/** Compare digest results across update boundaries, cloning, and reset. */
public class Test {
    public static void main(String[] args) throws Exception {
        String[][] vectors = {
            {"", "d41d8cd98f00b204e9800998ecf8427e"},
            {"a", "0cc175b9c0f1b6a831c399e269772661"},
            {"abc", "900150983cd24fb0d6963f7d28e17f72"},
            {"message digest", "f96b697d7cb7938d525a2f31aaf161d0"},
            {"abcdefghijklmnopqrstuvwxyz", "c3fcd3d76192e4007dfb496cca67e13b"},
            {"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", "d174ab98d277d9f5a5611c2c9f419d9f"},
            {"12345678901234567890123456789012345678901234567890123456789012345678901234567890", "57edf4a22be3c955ac49da2e2107b67a"}
        };
        for (String[] vector : vectors) {
            check(hex(MessageDigest.getInstance("MD5").digest(vector[0].getBytes(StandardCharsets.US_ASCII))).equals(vector[1]));
        }
        for (String algorithm : new String[]{"MD5", "SHA-1", "SHA-256", "SHA-512"}) {
            for (int length : new int[]{0, 1, 55, 56, 63, 64, 65, 119, 120, 127, 128, 129, 4097}) {
                byte[] input = new byte[length + 7];
                for (int i = 0; i < input.length; i++) input[i] = (byte) (i * 37 + 131);
                MessageDigest digest = MessageDigest.getInstance(algorithm);
                digest.update(input, 3, length);
                byte[] expected = digest.digest();
                for (int pos = 0; pos < length; pos += 7) {
                    digest.update(input, pos + 3, Math.min(7, length - pos));
                }
                check(Arrays.equals(expected, digest.digest()));
                for (int i = 0; i < length; i++) digest.update(input[i + 3]);
                check(Arrays.equals(expected, digest.digest()));

                int split = length / 2;
                digest.update(input, 3, split);
                MessageDigest clone = (MessageDigest) digest.clone();
                digest.update(input, split + 3, length - split);
                clone.update(input, split + 3, length - split);
                check(Arrays.equals(expected, digest.digest()));
                check(Arrays.equals(expected, clone.digest()));
                digest.update(input);
                digest.reset();
                digest.update(ByteBuffer.wrap(input, 3, length));
                byte[] output = new byte[expected.length + 9];
                int written = digest.digest(output, 4, expected.length);
                check(written == expected.length);
                check(Arrays.equals(expected, Arrays.copyOfRange(output, 4, 4 + written)));
                check(output[3] == 0 && output[4 + written] == 0);
                System.out.println(algorithm + " " + length + " " + hex(expected));
            }
        }
        byte[] million = new byte[1000000];
        Arrays.fill(million, (byte) 'a');
        check(hex(MessageDigest.getInstance("MD5").digest(million)).equals("7707d6ae4e027c70eea2a935c2296f21"));
        System.out.println("MessageDigest checks passed");
    }

    static String hex(byte[] bytes) {
        StringBuilder result = new StringBuilder();
        for (byte b : bytes) {
            result.append(Character.forDigit((b >>> 4) & 15, 16));
            result.append(Character.forDigit(b & 15, 16));
        }
        return result.toString();
    }

    static void check(boolean condition) {
        if (!condition) throw new AssertionError("incorrect digest");
    }
}
