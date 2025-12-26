/**
 * Tests that instance fields are zero-initialized before constructors run.
 *
 * Per JLS §12.5, when an object is created:
 * 1. Memory is allocated for the new object
 * 2. All instance fields are set to their default zero values:
 *    - int, short, byte, char: 0
 *    - long: 0L
 *    - float: 0.0f
 *    - double: 0.0d
 *    - boolean: false
 *    - Object references: null
 * 3. Then the constructor chain executes
 *
 * This test uses an instance initializer block to verify fields are zero
 * before field initializers run.
 */
public class Test {
    static class ZeroCheck {
        // Fields that will be checked and then assigned
        int intField;
        long longField;
        float floatField;
        double doubleField;
        boolean boolField;
        Object objField;
        short shortField;
        byte byteField;
        char charField;

        // Instance initializer runs BEFORE field initializers
        {
            System.out.println("Instance initializer - checking zero values:");
            System.out.println("intField = " + intField + " (expected 0)");
            System.out.println("longField = " + longField + " (expected 0)");
            System.out.println("floatField = " + floatField + " (expected 0.0)");
            System.out.println("doubleField = " + doubleField + " (expected 0.0)");
            System.out.println("boolField = " + boolField + " (expected false)");
            System.out.println("objField = " + objField + " (expected null)");
            System.out.println("shortField = " + shortField + " (expected 0)");
            System.out.println("byteField = " + byteField + " (expected 0)");
            int charValue = charField;
            System.out.println("charField = " + charValue + " (expected 0)");
        }

        ZeroCheck() {
            // Now assign actual values
            intField = 42;
            longField = 100L;
            floatField = 3.14f;
            doubleField = 2.718;
            boolField = true;
            objField = "hello";
            shortField = 10;
            byteField = 5;
            charField = 'X';
            System.out.println("Constructor completed");
        }
    }

    public static void main(String[] args) {
        System.out.println("Creating ZeroCheck object:");
        ZeroCheck obj = new ZeroCheck();

        System.out.println();
        System.out.println("Final field values:");
        System.out.println("intField = " + obj.intField);
        System.out.println("longField = " + obj.longField);
        System.out.println("floatField = " + obj.floatField);
        System.out.println("doubleField = " + obj.doubleField);
        System.out.println("boolField = " + obj.boolField);
        System.out.println("objField = " + obj.objField);
        System.out.println("shortField = " + obj.shortField);
        System.out.println("byteField = " + obj.byteField);
        System.out.println("charField = " + obj.charField);
    }
}

