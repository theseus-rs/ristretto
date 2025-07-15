/** Test proxy reflection operations. */
import java.lang.reflect.*;

public class Test {
    interface TestInterface {
        String getValue();
        void setValue(String value);
        int calculate(int a, int b);
    }

    interface AnotherInterface {
        void anotherMethod();
    }

    static class TestInvocationHandler implements InvocationHandler {
        private String value = "initial";

        @Override
        public Object invoke(Object proxy, Method method, Object[] args) throws Throwable {
            String methodName = method.getName();

            if ("getValue".equals(methodName)) {
                return value;
            } else if ("setValue".equals(methodName)) {
                value = (String) args[0];
                return null;
            } else if ("calculate".equals(methodName)) {
                return (Integer) args[0] + (Integer) args[1];
            } else if ("anotherMethod".equals(methodName)) {
                System.out.println("Another method called");
                return null;
            } else if ("toString".equals(methodName)) {
                return "TestProxy[value=" + value + "]";
            } else if ("hashCode".equals(methodName)) {
                return value.hashCode();
            } else if ("equals".equals(methodName)) {
                return proxy == args[0];
            }

            throw new UnsupportedOperationException("Method not supported: " + methodName);
        }
    }

    public static void main(String[] args) throws Exception {
        ClassLoader classLoader = Test.class.getClassLoader();
        Class<?>[] interfaces = {TestInterface.class, AnotherInterface.class};
        InvocationHandler handler = new TestInvocationHandler();

        // Create proxy instance
        Object proxy = Proxy.newProxyInstance(classLoader, interfaces, handler);
        System.out.println("Proxy created: " + proxy.getClass().getName());

        // Test proxy class properties
        Class<?> proxyClass = proxy.getClass();
        System.out.println("Is proxy class: " + Proxy.isProxyClass(proxyClass));
        System.out.println("Proxy class name: " + proxyClass.getName());
        System.out.println("Proxy class simple name: " + proxyClass.getSimpleName());

        // Test proxy superclass
        Class<?> superclass = proxyClass.getSuperclass();
        System.out.println("Proxy superclass: " + superclass.getName());
        System.out.println("Extends Proxy: " + (superclass == Proxy.class));

        // Test proxy interfaces
        Class<?>[] proxyInterfaces = proxyClass.getInterfaces();
        System.out.println("Proxy interfaces count: " + proxyInterfaces.length);
        for (Class<?> iface : proxyInterfaces) {
            System.out.println("Proxy interface: " + iface.getName());
        }

        // Test invocation handler retrieval
        InvocationHandler retrievedHandler = Proxy.getInvocationHandler(proxy);
        System.out.println("Retrieved handler: " + retrievedHandler.getClass().getName());
        System.out.println("Same handler: " + (handler == retrievedHandler));

        // Test proxy method invocation
        TestInterface testProxy = (TestInterface) proxy;
        String value = testProxy.getValue();
        System.out.println("Initial value: " + value);

        testProxy.setValue("modified");
        String newValue = testProxy.getValue();
        System.out.println("Modified value: " + newValue);

        int result = testProxy.calculate(10, 20);
        System.out.println("Calculate result: " + result);

        // Test second interface
        AnotherInterface anotherProxy = (AnotherInterface) proxy;
        anotherProxy.anotherMethod();

        // Test proxy class generation
        Class<?> generatedProxyClass = Proxy.getProxyClass(classLoader, TestInterface.class);
        System.out.println("Generated proxy class: " + generatedProxyClass.getName());
        System.out.println("Is same as instance class: " + (generatedProxyClass == proxyClass));

        // Test proxy constructor
        Constructor<?> proxyConstructor = generatedProxyClass.getConstructor(InvocationHandler.class);
        Object newProxy = proxyConstructor.newInstance(new TestInvocationHandler());
        System.out.println("New proxy created: " + newProxy.getClass().getName());

        // Test proxy method reflection
        Method[] proxyMethods = proxyClass.getDeclaredMethods();
        System.out.println("Proxy declared methods count: " + proxyMethods.length);

        Method[] publicMethods = proxyClass.getMethods();
        System.out.println("Proxy public methods count: " + publicMethods.length);

        // Find and test specific proxy method
        Method getValueMethod = proxyClass.getMethod("getValue");
        Object invokeResult = getValueMethod.invoke(proxy);
        System.out.println("Method invoke result: " + invokeResult);

        // Test proxy with no interfaces (should fail)
        try {
            Proxy.newProxyInstance(classLoader, new Class<?>[0], handler);
            System.out.println("ERROR: Should have failed with no interfaces");
        } catch (IllegalArgumentException e) {
            System.out.println("Correctly failed with no interfaces: " + e.getMessage());
        }

        // Test proxy with non-interface class (should fail)
        try {
            Proxy.newProxyInstance(classLoader, new Class<?>[]{String.class}, handler);
            System.out.println("ERROR: Should have failed with non-interface");
        } catch (IllegalArgumentException e) {
            System.out.println("Correctly failed with non-interface: " + e.getMessage());
        }

        // Test toString, hashCode, equals
        System.out.println("Proxy toString: " + proxy.toString());
        System.out.println("Proxy hashCode: " + proxy.hashCode());
        System.out.println("Proxy equals itself: " + proxy.equals(proxy));
        System.out.println("Proxy equals other: " + proxy.equals(new Object()));
    }
}
