/** Test method invocation on null reference */
public class Test {
    public static void main(String[] args) {
        String nullStr = null;

        // Test null.equals() - should throw NullPointerException
        try {
            boolean result = nullStr.equals("Hello");
            System.out.println("null.equals('Hello'): " + result);
        } catch (NullPointerException e) {
            System.out.println("null.equals('Hello') threw NullPointerException");
        }
    }
}

