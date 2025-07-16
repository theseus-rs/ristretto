/** Test the char primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        // Test char primitive class type
        System.out.println("=== char.class attributes ===");
        System.out.println("Name: " + char.class.getName());
        System.out.println("Simple name: " + char.class.getSimpleName());
        System.out.println("Canonical name: " + char.class.getCanonicalName());
        System.out.println("Is primitive: " + char.class.isPrimitive());
        System.out.println("Is array: " + char.class.isArray());
        System.out.println("Is interface: " + char.class.isInterface());
        System.out.println("Component type: " + char.class.getComponentType());
        System.out.println("Package: " + char.class.getPackage());
        System.out.println("Modifiers: " + char.class.getModifiers());
        System.out.println("Superclass: " + char.class.getSuperclass());
        System.out.println("toString: " + char.class.toString());

        // Test Character wrapper class
        System.out.println("=== Character wrapper tests ===");
        System.out.println(Character.MIN_HIGH_SURROGATE);
        System.out.println(Character.MAX_HIGH_SURROGATE);
        System.out.println(Character.MIN_LOW_SURROGATE);
        System.out.println(Character.MAX_LOW_SURROGATE);

        for (int c = 0; c < 127; c++) {
            System.out.print(Character.toChars(c));
        }
    }
}
