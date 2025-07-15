/** Test method reflection operations. */
public class Test {
    static class A {
        public static void staticMethod() {}
        public void publicMethod() {}
        protected void protectedMethod() {}
        private void privateMethod() {}
        void packageMethod() {}

        public String methodWithReturn() { return "test"; }
        public void methodWithParams(int x, String s) {}
        public void methodWithVarargs(String... args) {}

        public synchronized void synchronizedMethod() {}
        public final void finalMethod() {}
        public native void nativeMethod();

        public void overloadedMethod() {}
        public void overloadedMethod(int x) {}
        public void overloadedMethod(String s) {}
    }

    static abstract class AbstractClass {
        public abstract void abstractMethod();
    }

    public static void main(String[] args) throws Exception {
        Class<?> clazz = Class.forName("Test$A");

        // Test getting all methods
        java.lang.reflect.Method[] methods = clazz.getDeclaredMethods();
        System.out.println("Declared methods count: " + methods.length);

        // Test getting public methods (includes inherited)
        java.lang.reflect.Method[] publicMethods = clazz.getMethods();
        System.out.println("Public methods count: " + publicMethods.length);

        // Test getting specific method
        java.lang.reflect.Method publicMethod = clazz.getMethod("publicMethod");
        System.out.println("Found public method: " + publicMethod.getName());

        // Test method with parameters
        java.lang.reflect.Method methodWithParams = clazz.getMethod("methodWithParams", int.class, String.class);
        System.out.println("Method with params: " + methodWithParams.getName());

        // Test method modifiers
        java.lang.reflect.Method staticMethod = clazz.getMethod("staticMethod");
        System.out.println("Static method modifiers: " + java.lang.reflect.Modifier.toString(staticMethod.getModifiers()));

        // Test method return type
        java.lang.reflect.Method methodWithReturn = clazz.getMethod("methodWithReturn");
        System.out.println("Return type: " + methodWithReturn.getReturnType().getName());

        // Test method parameter types
        Class<?>[] paramTypes = methodWithParams.getParameterTypes();
        System.out.println("Parameter types: " + paramTypes[0].getName() + ", " + paramTypes[1].getName());

        // Test method invocation
        A instance = new A();
        Object result = methodWithReturn.invoke(instance);
        System.out.println("Method invocation result: " + result);

        // Test static method invocation
        staticMethod.invoke(null);
        System.out.println("Static method invoked successfully");

        // Test overloaded method resolution
        java.lang.reflect.Method overloaded1 = clazz.getMethod("overloadedMethod");
        java.lang.reflect.Method overloaded2 = clazz.getMethod("overloadedMethod", int.class);
        java.lang.reflect.Method overloaded3 = clazz.getMethod("overloadedMethod", String.class);
        System.out.println("Overloaded methods resolved: " + (overloaded1 != overloaded2 && overloaded2 != overloaded3));
    }
}
