import java.math.BigInteger;
import java.util.Random;

public class Test {
    private static final Random random = new Random();

    public static class RSAKeyPair {
        BigInteger n, e, d;

        RSAKeyPair(BigInteger n, BigInteger e, BigInteger d) {
            this.n = n;
            this.e = e;
            this.d = d;
        }
    }

    public static RSAKeyPair generateKeyPair(int bitLength) {
        // Generate two large prime numbers
        BigInteger p = BigInteger.probablePrime(bitLength / 2, random);
        BigInteger q = BigInteger.probablePrime(bitLength / 2, random);

        // Calculate n = p * q
        BigInteger n = p.multiply(q);

        // Calculate phi(n) = (p-1) * (q-1)
        BigInteger phi = p.subtract(BigInteger.ONE).multiply(q.subtract(BigInteger.ONE));

        // Choose e such that 1 < e < phi(n) and gcd(e, phi(n)) = 1
        BigInteger e = BigInteger.valueOf(65537); // Common choice
        while (phi.gcd(e).compareTo(BigInteger.ONE) > 0) {
            e = e.add(BigInteger.valueOf(2));
        }

        // Calculate d = e^(-1) mod phi(n)
        BigInteger d = e.modInverse(phi);

        return new RSAKeyPair(n, e, d);
    }

    public static BigInteger encrypt(BigInteger message, BigInteger e, BigInteger n) {
        return message.modPow(e, n);
    }

    public static BigInteger decrypt(BigInteger ciphertext, BigInteger d, BigInteger n) {
        return ciphertext.modPow(d, n);
    }

    public static void main(String[] args) {
        System.out.println("RSA Algorithm Implementation");

        // Generate RSA key pair
        RSAKeyPair keyPair = generateKeyPair(512);

        System.out.println("Public Key (n, e): (" + keyPair.n + ", " + keyPair.e + ")");
        System.out.println("Private Key (d): " + keyPair.d);

        // Message to encrypt
        BigInteger message = new BigInteger("123456789");
        System.out.println("\nOriginal message: " + message);

        // Encrypt
        BigInteger ciphertext = encrypt(message, keyPair.e, keyPair.n);
        System.out.println("Encrypted message: " + ciphertext);

        // Decrypt
        BigInteger decrypted = decrypt(ciphertext, keyPair.d, keyPair.n);
        System.out.println("Decrypted message: " + decrypted);

        System.out.println("Encryption/Decryption successful: " + message.equals(decrypted));
    }
}
