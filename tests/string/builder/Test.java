/** Test StringBuilder operations */
public class Test {
    public static void main(String[] args) {
        testStringBuilderBasics();
        testMethodChaining();
        testAppendVariousTypes();
    }

    private static void testStringBuilderBasics() {
        System.out.println("\n=== StringBuilder Tests ===");
        StringBuilder builder = new StringBuilder();
        builder.append("StringBuilder ");
        builder.append("is ");
        builder.append("not ");
        builder.append("synchronized");
        System.out.println("StringBuilder result: '" + builder.toString() + "'");

        builder.insert(13, "generally ");
        System.out.println("After insert: '" + builder.toString() + "'");

        builder.reverse();
        System.out.println("After reverse: '" + builder.toString() + "'");
    }

    private static void testMethodChaining() {
        System.out.println("\n=== Method Chaining Tests ===");
        StringBuilder chained = new StringBuilder()
            .append("Hello")
            .append(" ")
            .append("World")
            .insert(5, ",")
            .append("!");
        System.out.println("Chained operations result: '" + chained.toString() + "'");
    }

    private static void testAppendVariousTypes() {
        System.out.println("\n=== Append Various Types Tests ===");
        StringBuilder types = new StringBuilder();
        types.append("String: ").append("Hello").append(", ");
        types.append("int: ").append(42).append(", ");
        types.append("long: ").append(123L).append(", ");
        types.append("float: ").append(3.14f).append(", ");
        types.append("double: ").append(2.71828).append(", ");
        types.append("boolean: ").append(true).append(", ");
        types.append("char: ").append('X');
        System.out.println("Various types: " + types.toString());
    }
}
