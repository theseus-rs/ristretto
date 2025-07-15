/** Test modifier reflection operations. */
import java.lang.reflect.*;

public class Test {
    // Class modifiers
    public static final class FinalClass {}
    public static abstract class AbstractClass {}
    static class PackageClass {}
    private static class PrivateClass {}
    public interface TestInterface {}
    public @interface TestAnnotation {}
    public enum TestEnum { VALUE }

    // Field modifiers
    public static final String PUBLIC_STATIC_FINAL = "constant";
    public static String publicStatic = "public static";
    public final String publicFinal = "public final";
    public volatile String publicVolatile = "volatile";
    public transient String publicTransient = "transient";
    protected String protectedField = "protected";
    private String privateField = "private";
    String packageField = "package";

    // Method modifiers
    public static void publicStaticMethod() {}
    public final void publicFinalMethod() {}
    public synchronized void synchronizedMethod() {}
    public native void nativeMethod();
    public strictfp void strictfpMethod() {}
    protected void protectedMethod() {}
    private void privateMethod() {}
    void packageMethod() {}

    // Constructor modifiers
    public Test() {}
    protected Test(int x) {}
    private Test(String s) {}

    public static void main(String[] args) throws Exception {
        // Test class modifiers
        System.out.println("=== Class Modifiers ===");
        testClassModifiers("Test", Test.class);
        testClassModifiers("FinalClass", FinalClass.class);
        testClassModifiers("AbstractClass", AbstractClass.class);
        testClassModifiers("PackageClass", PackageClass.class);
        testClassModifiers("PrivateClass", PrivateClass.class);
        testClassModifiers("TestInterface", TestInterface.class);
        testClassModifiers("TestAnnotation", TestAnnotation.class);
        testClassModifiers("TestEnum", TestEnum.class);

        // Test field modifiers
        System.out.println("\n=== Field Modifiers ===");
        Field[] fields = Test.class.getDeclaredFields();
        for (Field field : fields) {
            if (!field.isSynthetic()) { // Skip synthetic fields
                testFieldModifiers(field);
            }
        }

        // Test method modifiers
        System.out.println("\n=== Method Modifiers ===");
        Method[] methods = Test.class.getDeclaredMethods();
        for (Method method : methods) {
            if (!method.isSynthetic() && !method.getName().equals("main")) {
                testMethodModifiers(method);
            }
        }

        // Test constructor modifiers
        System.out.println("\n=== Constructor Modifiers ===");
        Constructor<?>[] constructors = Test.class.getDeclaredConstructors();
        for (Constructor<?> constructor : constructors) {
            testConstructorModifiers(constructor);
        }

        // Test modifier utility methods
        System.out.println("\n=== Modifier Utility Methods ===");
        testModifierUtilities();

        // Test access level checking
        System.out.println("\n=== Access Level Checking ===");
        testAccessLevels();

        // Test modifier combinations
        System.out.println("\n=== Modifier Combinations ===");
        testModifierCombinations();
    }

    private static void testClassModifiers(String name, Class<?> clazz) {
        int modifiers = clazz.getModifiers();
        System.out.println(name + " modifiers: " + Modifier.toString(modifiers));
        System.out.println("  Public: " + Modifier.isPublic(modifiers));
        System.out.println("  Protected: " + Modifier.isProtected(modifiers));
        System.out.println("  Private: " + Modifier.isPrivate(modifiers));
        System.out.println("  Package: " + (!Modifier.isPublic(modifiers) && !Modifier.isProtected(modifiers) && !Modifier.isPrivate(modifiers)));
        System.out.println("  Static: " + Modifier.isStatic(modifiers));
        System.out.println("  Final: " + Modifier.isFinal(modifiers));
        System.out.println("  Abstract: " + Modifier.isAbstract(modifiers));
        System.out.println("  Interface: " + Modifier.isInterface(modifiers));
    }

    private static void testFieldModifiers(Field field) {
        int modifiers = field.getModifiers();
        System.out.println(field.getName() + " modifiers: " + Modifier.toString(modifiers));
        System.out.println("  Public: " + Modifier.isPublic(modifiers));
        System.out.println("  Protected: " + Modifier.isProtected(modifiers));
        System.out.println("  Private: " + Modifier.isPrivate(modifiers));
        System.out.println("  Static: " + Modifier.isStatic(modifiers));
        System.out.println("  Final: " + Modifier.isFinal(modifiers));
        System.out.println("  Volatile: " + Modifier.isVolatile(modifiers));
        System.out.println("  Transient: " + Modifier.isTransient(modifiers));
    }

    private static void testMethodModifiers(Method method) {
        int modifiers = method.getModifiers();
        System.out.println(method.getName() + " modifiers: " + Modifier.toString(modifiers));
        System.out.println("  Public: " + Modifier.isPublic(modifiers));
        System.out.println("  Protected: " + Modifier.isProtected(modifiers));
        System.out.println("  Private: " + Modifier.isPrivate(modifiers));
        System.out.println("  Static: " + Modifier.isStatic(modifiers));
        System.out.println("  Final: " + Modifier.isFinal(modifiers));
        System.out.println("  Synchronized: " + Modifier.isSynchronized(modifiers));
        System.out.println("  Native: " + Modifier.isNative(modifiers));
        System.out.println("  Abstract: " + Modifier.isAbstract(modifiers));
        System.out.println("  Strict: " + Modifier.isStrict(modifiers));
    }

    private static void testConstructorModifiers(Constructor<?> constructor) {
        int modifiers = constructor.getModifiers();
        Class<?>[] paramTypes = constructor.getParameterTypes();
        String paramDesc = paramTypes.length == 0 ? "no-arg" : paramTypes.length + " params";

        System.out.println("Constructor (" + paramDesc + ") modifiers: " + Modifier.toString(modifiers));
        System.out.println("  Public: " + Modifier.isPublic(modifiers));
        System.out.println("  Protected: " + Modifier.isProtected(modifiers));
        System.out.println("  Private: " + Modifier.isPrivate(modifiers));
    }

    private static void testModifierUtilities() {
        // Test modifier constants
        System.out.println("Modifier.PUBLIC: " + Modifier.PUBLIC);
        System.out.println("Modifier.PRIVATE: " + Modifier.PRIVATE);
        System.out.println("Modifier.PROTECTED: " + Modifier.PROTECTED);
        System.out.println("Modifier.STATIC: " + Modifier.STATIC);
        System.out.println("Modifier.FINAL: " + Modifier.FINAL);
        System.out.println("Modifier.SYNCHRONIZED: " + Modifier.SYNCHRONIZED);
        System.out.println("Modifier.VOLATILE: " + Modifier.VOLATILE);
        System.out.println("Modifier.TRANSIENT: " + Modifier.TRANSIENT);
        System.out.println("Modifier.NATIVE: " + Modifier.NATIVE);
        System.out.println("Modifier.INTERFACE: " + Modifier.INTERFACE);
        System.out.println("Modifier.ABSTRACT: " + Modifier.ABSTRACT);
        System.out.println("Modifier.STRICT: " + Modifier.STRICT);

        // Test modifier combinations
        int publicStatic = Modifier.PUBLIC | Modifier.STATIC;
        System.out.println("Public static combination: " + Modifier.toString(publicStatic));

        int privateFinal = Modifier.PRIVATE | Modifier.FINAL;
        System.out.println("Private final combination: " + Modifier.toString(privateFinal));
    }

    private static void testAccessLevels() throws Exception {
        Field publicField = Test.class.getDeclaredField("publicStatic");
        Field protectedField = Test.class.getDeclaredField("protectedField");
        Field privateField = Test.class.getDeclaredField("privateField");
        Field packageField = Test.class.getDeclaredField("packageField");

        System.out.println("Access levels:");
        System.out.println("  Public field: " + getAccessLevel(publicField.getModifiers()));
        System.out.println("  Protected field: " + getAccessLevel(protectedField.getModifiers()));
        System.out.println("  Private field: " + getAccessLevel(privateField.getModifiers()));
        System.out.println("  Package field: " + getAccessLevel(packageField.getModifiers()));
    }

    private static String getAccessLevel(int modifiers) {
        if (Modifier.isPublic(modifiers)) return "public";
        if (Modifier.isProtected(modifiers)) return "protected";
        if (Modifier.isPrivate(modifiers)) return "private";
        return "package";
    }

    private static void testModifierCombinations() throws Exception {
        // Test field with multiple modifiers
        Field constantField = Test.class.getDeclaredField("PUBLIC_STATIC_FINAL");
        int modifiers = constantField.getModifiers();

        System.out.println("Constant field has multiple modifiers:");
        System.out.println("  Is public static final: " +
                (Modifier.isPublic(modifiers) && Modifier.isStatic(modifiers) && Modifier.isFinal(modifiers)));

        // Test invalid modifier combinations (these should not exist)
        System.out.println("Testing impossible combinations:");
        System.out.println("  Abstract and final cannot coexist");
        System.out.println("  Static and abstract for methods");

        // Test interface method modifiers
        Method[] interfaceMethods = TestInterface.class.getDeclaredMethods();
        if (interfaceMethods.length > 0) {
            for (Method method : interfaceMethods) {
                int methodMods = method.getModifiers();
                System.out.println("Interface method modifiers: " + Modifier.toString(methodMods));
            }
        }

        // Test enum field modifiers
        Field[] enumFields = TestEnum.class.getDeclaredFields();
        for (Field field : enumFields) {
            if (field.isEnumConstant()) {
                int enumMods = field.getModifiers();
                System.out.println("Enum constant modifiers: " + Modifier.toString(enumMods));
            }
        }
    }
}
