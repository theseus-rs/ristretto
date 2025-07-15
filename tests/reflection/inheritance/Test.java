/** Test inheritance and polymorphism in reflection operations. */
public class Test {
    static abstract class AbstractBase {
        protected String baseField = "base";
        public abstract void abstractMethod();
        public void concreteMethod() {
            System.out.println("Base concrete method");
        }
        protected void protectedMethod() {
            System.out.println("Base protected method");
        }
    }

    static class MiddleClass extends AbstractBase {
        protected String middleField = "middle";

        @Override
        public void abstractMethod() {
            System.out.println("Middle abstract implementation");
        }

        @Override
        public void concreteMethod() {
            System.out.println("Middle overridden method");
        }

        public void middleSpecificMethod() {
            System.out.println("Middle specific method");
        }
    }

    static class DerivedClass extends MiddleClass {
        private String derivedField = "derived";

        @Override
        public void concreteMethod() {
            System.out.println("Derived overridden method");
        }

        public void derivedSpecificMethod() {
            System.out.println("Derived specific method");
        }

        // Method hiding (static)
        public static void staticMethod() {
            System.out.println("Derived static method");
        }
    }

    interface InterfaceA {
        void methodA();
        default void defaultMethod() {
            System.out.println("Interface A default method");
        }
    }

    interface InterfaceB {
        void methodB();
        default void defaultMethod() {
            System.out.println("Interface B default method");
        }
    }

    static class MultipleInterfaces implements InterfaceA, InterfaceB {
        @Override
        public void methodA() {
            System.out.println("Method A implementation");
        }

        @Override
        public void methodB() {
            System.out.println("Method B implementation");
        }

        @Override
        public void defaultMethod() {
            System.out.println("Overridden default method");
        }
    }

    public static void main(String[] args) throws Exception {
        // Test inheritance hierarchy
        System.out.println("=== Inheritance Hierarchy ===");
        Class<?> derivedClass = DerivedClass.class;
        Class<?> middleClass = MiddleClass.class;
        Class<?> baseClass = AbstractBase.class;

        System.out.println("Derived superclass: " + derivedClass.getSuperclass().getName());
        System.out.println("Middle superclass: " + middleClass.getSuperclass().getName());
        System.out.println("Base superclass: " + baseClass.getSuperclass().getName());

        // Test assignability
        System.out.println("\n=== Assignability Tests ===");
        System.out.println("Base assignable from Derived: " + baseClass.isAssignableFrom(derivedClass));
        System.out.println("Derived assignable from Base: " + derivedClass.isAssignableFrom(baseClass));
        System.out.println("Middle assignable from Derived: " + middleClass.isAssignableFrom(derivedClass));
        System.out.println("Derived assignable from Middle: " + derivedClass.isAssignableFrom(middleClass));

        // Test instance checking
        System.out.println("\n=== Instance Checking ===");
        DerivedClass derivedInstance = new DerivedClass();
        System.out.println("Derived instance of Base: " + baseClass.isInstance(derivedInstance));
        System.out.println("Derived instance of Middle: " + middleClass.isInstance(derivedInstance));
        System.out.println("Derived instance of Derived: " + derivedClass.isInstance(derivedInstance));

        // Test method inheritance and overriding
        System.out.println("\n=== Method Inheritance ===");
        java.lang.reflect.Method[] derivedMethods = derivedClass.getDeclaredMethods();
        java.lang.reflect.Method[] allMethods = derivedClass.getMethods();

        System.out.println("Derived declared methods: " + derivedMethods.length);
        System.out.println("Derived all methods (including inherited): " + allMethods.length);

        // Test specific method resolution
        java.lang.reflect.Method concreteMethod = derivedClass.getMethod("concreteMethod");
        System.out.println("Concrete method declaring class: " + concreteMethod.getDeclaringClass().getName());

        // Test method invocation with polymorphism
        System.out.println("\n=== Polymorphic Method Invocation ===");
        AbstractBase baseRef = derivedInstance;
        concreteMethod.invoke(baseRef);

        java.lang.reflect.Method abstractMethod = derivedClass.getMethod("abstractMethod");
        abstractMethod.invoke(baseRef);

        // Test field inheritance
        System.out.println("\n=== Field Inheritance ===");
        java.lang.reflect.Field[] derivedFields = derivedClass.getDeclaredFields();
        java.lang.reflect.Field[] allFields = derivedClass.getFields();

        System.out.println("Derived declared fields: " + derivedFields.length);
        System.out.println("Derived all fields (including inherited): " + allFields.length);

        // Access inherited fields
        java.lang.reflect.Field baseField = baseClass.getDeclaredField("baseField");
        baseField.setAccessible(true);
        System.out.println("Base field value from derived: " + baseField.get(derivedInstance));

        java.lang.reflect.Field middleField = middleClass.getDeclaredField("middleField");
        middleField.setAccessible(true);
        System.out.println("Middle field value from derived: " + middleField.get(derivedInstance));

        // Test interface implementation
        System.out.println("\n=== Interface Implementation ===");
        Class<?> multipleInterfacesClass = MultipleInterfaces.class;
        Class<?>[] interfaces = multipleInterfacesClass.getInterfaces();
        System.out.println("Implemented interfaces count: " + interfaces.length);

        for (Class<?> iface : interfaces) {
            System.out.println("Implemented interface: " + iface.getName());
        }

        // Test interface method resolution
        MultipleInterfaces interfaceInstance = new MultipleInterfaces();
        java.lang.reflect.Method methodA = multipleInterfacesClass.getMethod("methodA");
        java.lang.reflect.Method methodB = multipleInterfacesClass.getMethod("methodB");
        java.lang.reflect.Method defaultMethod = multipleInterfacesClass.getMethod("defaultMethod");

        methodA.invoke(interfaceInstance);
        methodB.invoke(interfaceInstance);
        defaultMethod.invoke(interfaceInstance);

        // Test interface assignability
        System.out.println("\n=== Interface Assignability ===");
        System.out.println("InterfaceA assignable from MultipleInterfaces: " +
            InterfaceA.class.isAssignableFrom(multipleInterfacesClass));
        System.out.println("InterfaceB assignable from MultipleInterfaces: " +
            InterfaceB.class.isAssignableFrom(multipleInterfacesClass));

        // Test method hiding vs overriding
        System.out.println("\n=== Method Hiding vs Overriding ===");
        try {
            java.lang.reflect.Method staticMethod = derivedClass.getMethod("staticMethod");
            System.out.println("Static method declaring class: " + staticMethod.getDeclaringClass().getName());
            staticMethod.invoke(null);
        } catch (NoSuchMethodException e) {
            System.out.println("Static method not found (method hiding doesn't affect reflection)");
        }

        // Test access to protected methods
        System.out.println("\n=== Protected Method Access ===");
        java.lang.reflect.Method protectedMethod = baseClass.getDeclaredMethod("protectedMethod");
        protectedMethod.setAccessible(true);
        protectedMethod.invoke(derivedInstance);

        // Test overridden method resolution
        System.out.println("\n=== Overridden Method Resolution ===");
        java.lang.reflect.Method baseConcreteMethod = baseClass.getDeclaredMethod("concreteMethod");
        java.lang.reflect.Method derivedConcreteMethod = derivedClass.getMethod("concreteMethod");

        System.out.println("Base method declaring class: " + baseConcreteMethod.getDeclaringClass().getName());
        System.out.println("Derived method declaring class: " + derivedConcreteMethod.getDeclaringClass().getName());
        System.out.println("Methods are different: " + !baseConcreteMethod.equals(derivedConcreteMethod));

        // Invoke both to show polymorphism
        baseConcreteMethod.invoke(derivedInstance);  // Calls derived implementation
        derivedConcreteMethod.invoke(derivedInstance);  // Also calls derived implementation
    }
}

