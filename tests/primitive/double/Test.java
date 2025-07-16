/** Test the double primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        // Test double primitive class type
        System.out.println("=== double.class attributes ===");
        System.out.println("Name: " + double.class.getName());
        System.out.println("Simple name: " + double.class.getSimpleName());
        System.out.println("Canonical name: " + double.class.getCanonicalName());
        System.out.println("Is primitive: " + double.class.isPrimitive());
        System.out.println("Is array: " + double.class.isArray());
        System.out.println("Is interface: " + double.class.isInterface());
        System.out.println("Component type: " + double.class.getComponentType());
        System.out.println("Package: " + double.class.getPackage());
        System.out.println("Modifiers: " + double.class.getModifiers());
        System.out.println("Superclass: " + double.class.getSuperclass());
        System.out.println("toString: " + double.class.toString());

        // Test Double wrapper class and operations
        System.out.println("=== Double wrapper tests ===");
        System.out.println(0.0d);
        System.out.println(1.0d);
        System.out.println(Double.MIN_VALUE);
        System.out.println(Double.MAX_VALUE);
        System.out.println(Double.valueOf(0.0d).hashCode());
        System.out.println(Double.valueOf(Double.MIN_VALUE).hashCode());
        System.out.println(Double.valueOf(Double.MAX_VALUE).hashCode());
        System.out.println(1.0d + 4.0d);
        System.out.println(12.0d - 7.0d);
        System.out.println(5.0d * 7.0d);
        System.out.println(4.0d / 2.0d);
        System.out.println(5.0d % 2.0d);
        double value = 8.0d;
        System.out.println(++value);
        value++;
        System.out.println(value);
        System.out.println(--value);
        value--;
        System.out.println(value);
    }
}
