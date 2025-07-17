/** Test the short primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        // Test short primitive class type
        System.out.println("=== short.class attributes ===");
        System.out.println("Name: " + short.class.getName());
        System.out.println("Simple name: " + short.class.getSimpleName());
        System.out.println("Canonical name: " + short.class.getCanonicalName());
        System.out.println("Is primitive: " + short.class.isPrimitive());
        System.out.println("Is array: " + short.class.isArray());
        System.out.println("Is interface: " + short.class.isInterface());
        System.out.println("Component type: " + short.class.getComponentType());
        System.out.println("Package: " + short.class.getPackage());
        System.out.println("Modifiers: " + short.class.getModifiers());
        System.out.println("Superclass: " + short.class.getSuperclass());
        System.out.println("toString: " + short.class.toString());

        // Test Short wrapper class
        System.out.println("=== Short wrapper tests ===");
        short zero = 0;
        short one = 1;
        System.out.println(zero);
        System.out.println(one);
        System.out.println(Short.MIN_VALUE);
        System.out.println(Short.MAX_VALUE);
        System.out.println(Short.valueOf(zero).hashCode());
        System.out.println(Short.valueOf(Short.MIN_VALUE).hashCode());
        System.out.println(Short.valueOf(Short.MAX_VALUE).hashCode());
    }
}
