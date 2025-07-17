/** Test the boolean primitive type and its wrapper class. */
class Test {
    public static void main(String[] args) throws Exception {
        // Test boolean primitive class type
        System.out.println("=== boolean.class attributes ===");
        System.out.println("Name: " + boolean.class.getName());
        System.out.println("Simple name: " + boolean.class.getSimpleName());
        System.out.println("Canonical name: " + boolean.class.getCanonicalName());
        System.out.println("Is primitive: " + boolean.class.isPrimitive());
        System.out.println("Is array: " + boolean.class.isArray());
        System.out.println("Is interface: " + boolean.class.isInterface());
        System.out.println("Component type: " + boolean.class.getComponentType());
        System.out.println("Package: " + boolean.class.getPackage());
        System.out.println("Modifiers: " + boolean.class.getModifiers());
        System.out.println("Superclass: " + boolean.class.getSuperclass());
        System.out.println("toString: " + boolean.class.toString());

        // Test Boolean wrapper class
        System.out.println("=== Boolean wrapper tests ===");
        System.out.println(Boolean.TRUE);
        System.out.println(Boolean.FALSE);
        System.out.println(Boolean.valueOf(true));
        System.out.println(Boolean.valueOf(false));
        System.out.println(Boolean.valueOf("true"));
        System.out.println(Boolean.valueOf("false"));
        System.out.println(Boolean.TRUE.hashCode());
        System.out.println(Boolean.FALSE.hashCode());
        System.out.println(Boolean.TRUE.equals(Boolean.TRUE));
        System.out.println(Boolean.TRUE.equals(Boolean.FALSE));
    }
}
