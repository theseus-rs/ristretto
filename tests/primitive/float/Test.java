/** Test the float primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        // Test float primitive class type
        System.out.println("=== float.class attributes ===");
        System.out.println("Name: " + float.class.getName());
        System.out.println("Simple name: " + float.class.getSimpleName());
        System.out.println("Canonical name: " + float.class.getCanonicalName());
        System.out.println("Is primitive: " + float.class.isPrimitive());
        System.out.println("Is array: " + float.class.isArray());
        System.out.println("Is interface: " + float.class.isInterface());
        System.out.println("Component type: " + float.class.getComponentType());
        System.out.println("Package: " + float.class.getPackage());
        System.out.println("Modifiers: " + float.class.getModifiers());
        System.out.println("Superclass: " + float.class.getSuperclass());
        System.out.println("toString: " + float.class.toString());

        // Test Float wrapper class and operations
        System.out.println("=== Float wrapper tests ===");
        System.out.println(0.0f);
        System.out.println(1.0f);
        System.out.println(Float.MIN_VALUE);
        System.out.println(Float.MAX_VALUE);
        System.out.println(Float.valueOf(0.0f).hashCode());
        System.out.println(Float.valueOf(Float.MIN_VALUE).hashCode());
        System.out.println(Float.valueOf(Float.MAX_VALUE).hashCode());
        System.out.println(1.0f + 4.0f);
        System.out.println(12.0f - 7.0f);
        System.out.println(5.0f * 7.0f);
        System.out.println(4.0f / 2.0f);
        System.out.println(5.0f % 2.0f);
        float value = 8.0f;
        System.out.println(++value);
        value++;
        System.out.println(value);
        System.out.println(--value);
        value--;
        System.out.println(value);
    }
}
