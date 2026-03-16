import java.lang.reflect.Method;

/**
 * Test java.lang.Compiler methods via reflection.
 * java.lang.Compiler was removed in Java 22, so reflection is used to
 * support both old and new Java versions.
 */
public class Test {
    public static void main(String[] args) {
        try {
            Class<?> compilerClass = Class.forName("java.lang.Compiler");

            // Test command(Object) -> returns null (no JIT compiler)
            Method commandMethod = compilerClass.getMethod("command", Object.class);
            Object commandResult = commandMethod.invoke(null, (Object) null);
            System.out.println("command(null): " + commandResult);

            // Test compileClass(Class) -> returns false (no JIT compiler)
            Method compileClassMethod = compilerClass.getMethod("compileClass", Class.class);
            Object compileClassResult = compileClassMethod.invoke(null, Test.class);
            System.out.println("compileClass(Test.class): " + compileClassResult);

            // Test compileClasses(String) -> returns false (no JIT compiler)
            Method compileClassesMethod = compilerClass.getMethod("compileClasses", String.class);
            Object compileClassesResult = compileClassesMethod.invoke(null, "Test");
            System.out.println("compileClasses(\"Test\"): " + compileClassesResult);

            // Test enable() -> void, should not throw
            Method enableMethod = compilerClass.getMethod("enable");
            enableMethod.invoke(null);
            System.out.println("enable(): OK");

            // Test disable() -> void, should not throw
            Method disableMethod = compilerClass.getMethod("disable");
            disableMethod.invoke(null);
            System.out.println("disable(): OK");
        } catch (ClassNotFoundException e) {
            System.out.println("java.lang.Compiler is not available (removed in Java 22)");
        } catch (Exception e) {
            System.out.println("Error: " + e.getMessage());
        }
    }
}
