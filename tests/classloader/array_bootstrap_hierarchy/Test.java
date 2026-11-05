import java.io.InputStream;
import java.io.Serializable;
import java.lang.reflect.Array;

/** Array supertypes must bypass the component loader's delegation policy. */
public class Test {
    static class Loader extends ClassLoader {
        final String rejectedName;
        boolean restrict;
        int rejectedRequests;

        Loader(ClassLoader parent, String rejectedName) {
            super(parent);
            this.rejectedName = rejectedName;
        }

        Class<?> define(byte[] bytes) {
            return defineClass("ArrayElement", bytes, 0, bytes.length);
        }

        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            if (restrict && name.equals(rejectedName)) {
                rejectedRequests++;
                throw new ClassNotFoundException(name);
            }
            return super.loadClass(name, resolve);
        }
    }

    static void check(boolean condition, String message) {
        if (!condition) throw new AssertionError(message);
    }

    static void checkArray(Object array, Class<?> component, Loader loader) {
        Class<?> type = array.getClass();
        check(type.getClassLoader() == loader, "array defining loader");
        check(type.getComponentType() == component, "array component identity");
        check(type.getSuperclass() == Object.class, "bootstrap superclass identity");
        Class<?>[] interfaces = type.getInterfaces();
        check(interfaces.length == 2, "array interface count");
        check(interfaces[0] == Cloneable.class, "bootstrap Cloneable identity");
        check(interfaces[1] == Serializable.class, "bootstrap Serializable identity");
        for (Class<?> supertype : new Class<?>[] {Object.class, Cloneable.class, Serializable.class}) {
            check(supertype.getClassLoader() == null, "bootstrap supertype loader");
            check(supertype.isAssignableFrom(type), "array supertype assignability");
            check(supertype.isInstance(array), "array supertype instance");
        }
    }

    public static void main(String[] args) throws Exception {
        byte[] bytes;
        try (InputStream in = Test.class.getResourceAsStream("/ArrayElement.class")) {
            if (in == null) throw new AssertionError("missing array component fixture");
            bytes = in.readAllBytes();
        }
        for (ClassLoader parent : new ClassLoader[] {null, Test.class.getClassLoader()}) {
            for (String rejected : new String[] {
                    "java.lang.Cloneable", "java.io.Serializable", "java.lang.Object"}) {
                Loader loader = new Loader(parent, rejected);
                Class<?> component = loader.define(bytes);
                check(component.getClassLoader() == loader, "component defining loader");
                // Resolve the component's own hierarchy before restricting subsequent loads.
                check(component.getSuperclass() == Object.class, "component superclass");
                loader.restrict = true;
                try {
                    loader.loadClass(rejected);
                    throw new AssertionError("loader did not reject " + rejected);
                } catch (ClassNotFoundException expected) {}
                loader.rejectedRequests = 0;

                Object array = Array.newInstance(component, 1);
                checkArray(array, component, loader);
                Object matrix = Array.newInstance(component, new int[] {1, 2});
                checkArray(matrix, array.getClass(), loader);
                checkArray(Array.get(matrix, 0), component, loader);
                check(Array.newInstance(array.getClass(), 1).getClass() == matrix.getClass(),
                        "nested array identity");
                check(Class.forName("[LArrayElement;", false, loader) == array.getClass(),
                        "array lookup identity");
                check(Class.forName("[[LArrayElement;", false, loader) == matrix.getClass(),
                        "nested array lookup identity");
                check(loader.rejectedRequests == 0, "array hierarchy consulted component loader");
                System.out.println("array supertypes bypass rejection of " + rejected);
            }
        }
    }
}

class ArrayElement {}
