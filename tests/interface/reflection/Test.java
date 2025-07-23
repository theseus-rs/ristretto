/** Test interface reflection and runtime introspection */
import java.lang.reflect.*;

interface ReflectionTarget {
    void publicMethod();
    default void defaultMethod() {
        System.out.println("ReflectionTarget.defaultMethod");
    }
    static void staticMethod() {
        System.out.println("ReflectionTarget.staticMethod");
    }
    String CONSTANT = "ReflectionConstant";
}

interface GenericReflection<T extends Number> {
    T process(T input);
    Class<T> getType();
}

class ReflectionImpl implements ReflectionTarget, GenericReflection<Integer> {
    public void publicMethod() {
        System.out.println("ReflectionImpl.publicMethod");
    }

    public Integer process(Integer input) {
        System.out.println("Processing integer: " + input);
        return input * 2;
    }

    public Class<Integer> getType() {
        return Integer.class;
    }

    private void privateMethod() {
        System.out.println("ReflectionImpl.privateMethod");
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interface Reflection Test ===");

        ReflectionImpl impl = new ReflectionImpl();
        Class<?> implClass = impl.getClass();

        // Test getting implemented interfaces
        Class<?>[] interfaces = implClass.getInterfaces();
        System.out.println("Implemented interfaces:");
        for (Class<?> iface : interfaces) {
            System.out.println("  " + iface.getName());
            System.out.println("  Is interface: " + iface.isInterface());
            System.out.println("  Modifiers: " + Modifier.toString(iface.getModifiers()));
        }

        // Test interface hierarchy
        Class<?> targetInterface = ReflectionTarget.class;
        System.out.println("\nReflectionTarget interface analysis:");
        System.out.println("Name: " + targetInterface.getName());
        System.out.println("Simple name: " + targetInterface.getSimpleName());
        System.out.println("Is interface: " + targetInterface.isInterface());
        System.out.println("Package: " + targetInterface.getPackage());

        // Test interface methods
        Method[] methods = targetInterface.getMethods();
        System.out.println("\nReflectionTarget methods:");
        for (Method method : methods) {
            System.out.println("  " + method.getName() +
                             " - Modifiers: " + Modifier.toString(method.getModifiers()) +
                             " - Default: " + method.isDefault() +
                             " - Return type: " + method.getReturnType().getSimpleName());
        }

        // Test interface fields (constants)
        Field[] fields = targetInterface.getFields();
        System.out.println("\nReflectionTarget fields:");
        for (Field field : fields) {
            try {
                System.out.println("  " + field.getName() +
                                 " = " + field.get(null) +
                                 " - Modifiers: " + Modifier.toString(field.getModifiers()));
            } catch (IllegalAccessException e) {
                System.out.println("  " + field.getName() + " - Access denied");
            }
        }

        // Test method invocation through reflection
        try {
            Method publicMethod = targetInterface.getMethod("publicMethod");
            System.out.println("\nInvoking publicMethod via reflection:");
            publicMethod.invoke(impl);

            Method defaultMethod = targetInterface.getMethod("defaultMethod");
            System.out.println("Invoking defaultMethod via reflection:");
            defaultMethod.invoke(impl);

            Method staticMethod = targetInterface.getMethod("staticMethod");
            System.out.println("Invoking staticMethod via reflection:");
            staticMethod.invoke(null); // null for static methods

        } catch (Exception e) {
            System.out.println("Reflection invocation error: " + e.getMessage());
        }

        // Test generic interface reflection
        Class<?> genericInterface = GenericReflection.class;
        System.out.println("\nGeneric interface analysis:");
        TypeVariable<?>[] typeParams = genericInterface.getTypeParameters();
        for (TypeVariable<?> typeParam : typeParams) {
            System.out.println("Type parameter: " + typeParam.getName());
            Type[] bounds = typeParam.getBounds();
            for (Type bound : bounds) {
                System.out.println("  Bound: " + bound.getTypeName());
            }
        }

        // Test assignability
        System.out.println("\nAssignability tests:");
        System.out.println("ReflectionTarget.isAssignableFrom(ReflectionImpl): " +
                          ReflectionTarget.class.isAssignableFrom(ReflectionImpl.class));
        System.out.println("ReflectionImpl.isAssignableFrom(ReflectionTarget): " +
                          ReflectionImpl.class.isAssignableFrom(ReflectionTarget.class));
        System.out.println("Object.isAssignableFrom(ReflectionTarget): " +
                          Object.class.isAssignableFrom(ReflectionTarget.class));

        // Test interface instance checks
        System.out.println("\nInterface instance checks:");
        System.out.println("impl.getClass().isInstance(ReflectionTarget): " +
                          ReflectionTarget.class.isInstance(impl));
        System.out.println("GenericReflection.isInstance(impl): " +
                          GenericReflection.class.isInstance(impl));

        // Test getting all interfaces including inherited ones
        Class<?> current = implClass;
        System.out.println("\nAll interfaces in hierarchy:");
        while (current != null) {
            Class<?>[] currentInterfaces = current.getInterfaces();
            for (Class<?> iface : currentInterfaces) {
                System.out.println("  " + current.getSimpleName() + " implements " + iface.getSimpleName());
            }
            current = current.getSuperclass();
        }

        System.out.println("Interface reflection tests completed");
    }
}
