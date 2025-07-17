/** Test the integer primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        // Test int primitive class type
        System.out.println("=== int.class attributes ===");
        System.out.println("Name: " + int.class.getName());
        System.out.println("Simple name: " + int.class.getSimpleName());
        System.out.println("Canonical name: " + int.class.getCanonicalName());
        System.out.println("Is primitive: " + int.class.isPrimitive());
        System.out.println("Is array: " + int.class.isArray());
        System.out.println("Is interface: " + int.class.isInterface());
        System.out.println("Component type: " + int.class.getComponentType());
        System.out.println("Package: " + int.class.getPackage());
        System.out.println("Modifiers: " + int.class.getModifiers());
        System.out.println("Superclass: " + int.class.getSuperclass());
        System.out.println("toString: " + int.class.toString());

        // Test Integer wrapper class and operations
        System.out.println("=== Integer wrapper tests ===");
        System.out.println(-1);
        System.out.println(0);
        System.out.println(1);
        System.out.println(2);
        System.out.println(3);
        System.out.println(4);
        System.out.println(5);
        System.out.println(Integer.MIN_VALUE);
        System.out.println(Integer.MAX_VALUE);
        System.out.println(Integer.valueOf(0).hashCode());
        System.out.println(Integer.valueOf(Integer.MIN_VALUE).hashCode());
        System.out.println(Integer.valueOf(Integer.MAX_VALUE).hashCode());
        System.out.println(1 + 4);
        System.out.println(12 - 7);
        System.out.println(5 * 7);
        System.out.println(4 / 2);
        System.out.println(5 % 2);
        int value = 8;
        System.out.println(++value);
        value++;
        System.out.println(value);
        System.out.println(--value);
        value--;
        System.out.println(value);
    }
}
