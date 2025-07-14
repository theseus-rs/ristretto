/** Test StringBuffer operations */
public class Test {
    public static void main(String[] args) {
        testStringBufferBasics();
        testStringBufferInsert();
        testStringBufferDelete();
        testStringBufferReplace();
        testStringBufferReverse();
        testStringBufferCapacity();
        testStringBufferSetLength();
        testStringBufferCharAccess();
        testStringBufferSubstring();
        testStringBufferIndexOf();
    }

    private static void testStringBufferBasics() {
        System.out.println("=== StringBuffer Basic Tests ===");
        StringBuffer sb = new StringBuffer();
        System.out.println("Initial capacity: " + sb.capacity());
        System.out.println("Initial length: " + sb.length());
        System.out.println("Initial string: '" + sb.toString() + "'");

        sb.append("Hello");
        System.out.println("After append('Hello'): '" + sb.toString() + "'");
        System.out.println("Length: " + sb.length());

        sb.append(" World");
        System.out.println("After append(' World'): '" + sb.toString() + "'");

        sb.append(123);
        System.out.println("After append(123): '" + sb.toString() + "'");

        sb.append(true);
        System.out.println("After append(true): '" + sb.toString() + "'");

        sb.append('!');
        System.out.println("After append('!'): '" + sb.toString() + "'");
    }

    private static void testStringBufferInsert() {
        System.out.println("\n=== StringBuffer Insert Tests ===");
        StringBuffer sb2 = new StringBuffer("Hello World");
        System.out.println("Original: '" + sb2.toString() + "'");

        sb2.insert(5, ",");
        System.out.println("After insert(5, ','): '" + sb2.toString() + "'");

        sb2.insert(0, "Hi ");
        System.out.println("After insert(0, 'Hi '): '" + sb2.toString() + "'");

        sb2.insert(sb2.length(), " End");
        System.out.println("After insert(length(), ' End'): '" + sb2.toString() + "'");
    }

    private static void testStringBufferDelete() {
        System.out.println("\n=== StringBuffer Delete Tests ===");
        StringBuffer sb3 = new StringBuffer("Hello World Test");
        System.out.println("Original: '" + sb3.toString() + "'");

        sb3.delete(5, 11);
        System.out.println("After delete(5, 11): '" + sb3.toString() + "'");

        sb3.deleteCharAt(0);
        System.out.println("After deleteCharAt(0): '" + sb3.toString() + "'");
    }

    private static void testStringBufferReplace() {
        System.out.println("\n=== StringBuffer Replace Tests ===");
        StringBuffer sb4 = new StringBuffer("Hello World Hello");
        System.out.println("Original: '" + sb4.toString() + "'");

        sb4.replace(0, 5, "Hi");
        System.out.println("After replace(0, 5, 'Hi'): '" + sb4.toString() + "'");

        sb4.replace(3, 8, "There");
        System.out.println("After replace(3, 8, 'There'): '" + sb4.toString() + "'");
    }

    private static void testStringBufferReverse() {
        System.out.println("\n=== StringBuffer Reverse Tests ===");
        StringBuffer sb5 = new StringBuffer("Hello World");
        System.out.println("Original: '" + sb5.toString() + "'");
        sb5.reverse();
        System.out.println("After reverse(): '" + sb5.toString() + "'");

        // Test with unicode
        StringBuffer sb6 = new StringBuffer("Hello 世界");
        System.out.println("Unicode original: '" + sb6.toString() + "'");
        sb6.reverse();
        System.out.println("Unicode reversed: '" + sb6.toString() + "'");
    }

    private static void testStringBufferCapacity() {
        System.out.println("\n=== StringBuffer Capacity Tests ===");
        StringBuffer sb7 = new StringBuffer(10);
        System.out.println("Initial capacity (10): " + sb7.capacity());
        System.out.println("Initial length: " + sb7.length());

        sb7.append("Hello World Test");
        System.out.println("After append long string: '" + sb7.toString() + "'");
        System.out.println("Capacity after growth: " + sb7.capacity());
        System.out.println("Length: " + sb7.length());

        sb7.ensureCapacity(50);
        System.out.println("After ensureCapacity(50): " + sb7.capacity());

        sb7.trimToSize();
        System.out.println("After trimToSize(): " + sb7.capacity());
    }

    private static void testStringBufferSetLength() {
        System.out.println("\n=== StringBuffer setLength Tests ===");
        StringBuffer sb8 = new StringBuffer("Hello World");
        System.out.println("Original: '" + sb8.toString() + "'");
        System.out.println("Length: " + sb8.length());

        sb8.setLength(5);
        System.out.println("After setLength(5): '" + sb8.toString() + "'");
        System.out.println("Length: " + sb8.length());

        sb8.setLength(10);
        System.out.println("After setLength(10): '" + sb8.toString() + "'");
        System.out.println("Length: " + sb8.length());
    }

    private static void testStringBufferCharAccess() {
        System.out.println("\n=== StringBuffer charAt/setCharAt Tests ===");
        StringBuffer sb9 = new StringBuffer("Hello");
        System.out.println("Original: '" + sb9.toString() + "'");

        for (int i = 0; i < sb9.length(); i++) {
            System.out.println("charAt(" + i + "): " + sb9.charAt(i));
        }

        sb9.setCharAt(0, 'h');
        sb9.setCharAt(1, 'E');
        System.out.println("After setCharAt modifications: '" + sb9.toString() + "'");
    }

    private static void testStringBufferSubstring() {
        System.out.println("\n=== StringBuffer substring Tests ===");
        StringBuffer sb10 = new StringBuffer("Hello World Test");
        System.out.println("Original: '" + sb10.toString() + "'");
        System.out.println("substring(0, 5): '" + sb10.substring(0, 5) + "'");
        System.out.println("substring(6): '" + sb10.substring(6) + "'");
    }

    private static void testStringBufferIndexOf() {
        System.out.println("\n=== StringBuffer indexOf Tests ===");
        StringBuffer sb11 = new StringBuffer("Hello World Hello");
        System.out.println("Original: '" + sb11.toString() + "'");
        System.out.println("indexOf('l'): " + sb11.indexOf("l"));
        System.out.println("indexOf('l', 3): " + sb11.indexOf("l", 3));
        System.out.println("lastIndexOf('l'): " + sb11.lastIndexOf("l"));
        System.out.println("indexOf('Hello'): " + sb11.indexOf("Hello"));
        System.out.println("lastIndexOf('Hello'): " + sb11.lastIndexOf("Hello"));
    }
}
