/** Test the byte primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        // Test byte primitive class type
        System.out.println("=== byte.class attributes ===");
        System.out.println("Name: " + byte.class.getName());
        System.out.println("Simple name: " + byte.class.getSimpleName());
        System.out.println("Canonical name: " + byte.class.getCanonicalName());
        System.out.println("Is primitive: " + byte.class.isPrimitive());
        System.out.println("Is array: " + byte.class.isArray());
        System.out.println("Is interface: " + byte.class.isInterface());
        System.out.println("Component type: " + byte.class.getComponentType());
        System.out.println("Package: " + byte.class.getPackage());
        System.out.println("Modifiers: " + byte.class.getModifiers());
        System.out.println("Superclass: " + byte.class.getSuperclass());
        System.out.println("toString: " + byte.class.toString());

        // Test Byte wrapper class
        System.out.println("=== Byte wrapper tests ===");
        byte zero = 0;
        byte one = 1;
        System.out.println(zero);
        System.out.println(one);
        System.out.println(Byte.MIN_VALUE);
        System.out.println(Byte.MAX_VALUE);
        System.out.println(Byte.valueOf(zero).hashCode());
        System.out.println(Byte.valueOf(Byte.MIN_VALUE).hashCode());
        System.out.println(Byte.valueOf(Byte.MAX_VALUE).hashCode());
    }
}
