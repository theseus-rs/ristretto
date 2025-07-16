import java.util.Random;

public class Test {
    public static void main(String[] args) {
        System.out.println("Testing Random() default constructor");

        Random r1 = new Random();
        Random r2 = new Random();

        // Default constructor should create different sequences
        int val1 = r1.nextInt();
        int val2 = r2.nextInt();

        System.out.println("First random value from r1: " + val1);
        System.out.println("First random value from r2: " + val2);
        System.out.println("Values are different: " + (val1 != val2));

        // Test that subsequent calls produce different values
        int next1 = r1.nextInt();
        int next2 = r1.nextInt();
        System.out.println("Sequential values from same instance are different: " + (next1 != next2));

        System.out.println("Test completed successfully");
    }
}
