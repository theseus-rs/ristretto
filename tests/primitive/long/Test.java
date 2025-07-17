/** Test the long primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        // Test long primitive class type
        System.out.println("=== long.class attributes ===");
        System.out.println("Name: " + long.class.getName());
        System.out.println("Simple name: " + long.class.getSimpleName());
        System.out.println("Canonical name: " + long.class.getCanonicalName());
        System.out.println("Is primitive: " + long.class.isPrimitive());
        System.out.println("Is array: " + long.class.isArray());
        System.out.println("Is interface: " + long.class.isInterface());
        System.out.println("Component type: " + long.class.getComponentType());
        System.out.println("Package: " + long.class.getPackage());
        System.out.println("Modifiers: " + long.class.getModifiers());
        System.out.println("Superclass: " + long.class.getSuperclass());
        System.out.println("toString: " + long.class.toString());

        // Test Long wrapper class and operations
        System.out.println("=== Long wrapper tests ===");
        System.out.println(0L);
        System.out.println(1L);
        System.out.println(Long.MIN_VALUE);
        System.out.println(Long.MAX_VALUE);
        System.out.println(Long.valueOf(0L).hashCode());
        System.out.println(Long.valueOf(Long.MIN_VALUE).hashCode());
        System.out.println(Long.valueOf(Long.MAX_VALUE).hashCode());
        System.out.println(1L + 4L);
        System.out.println(12L - 7L);
        System.out.println(5L * 7L);
        System.out.println(4L / 2L);
        System.out.println(5L % 2L);
        long value = 8;
        System.out.println(++value);
        value++;
        System.out.println(value);
        System.out.println(--value);
        value--;
        System.out.println(value);
    }
}
