import java.io.InputStream;

/** Custom delegation must win even when a native parent supplies the same name. */
public class Test {
    public static class Element {
        public static Class<?> tag() { return Element.class; }
    }

    public static class Factory {
        public static Class<?> tag() { return Element.tag(); }
        public static Class<?> literal() { return Element[].class; }
        public static Object array() { return new Element[1]; }
        public static Object matrix() { return new Element[1][2]; }
    }

    static class Loader extends ClassLoader {
        final Loader sibling;
        int elementLoads;

        Loader(Loader sibling) { this.sibling = sibling; }

        Class<?> loaded(String name) { return findLoadedClass(name); }

        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            Class<?> loaded = findLoadedClass(name);
            if (loaded != null) return loaded;
            if (name.equals("Test$Element")) {
                elementLoads++;
                if (sibling != null) return sibling.loadClass(name);
            } else if (!name.equals("Test$Factory")) {
                return super.loadClass(name, resolve);
            }
            try (InputStream in = Test.class.getResourceAsStream("/" + name + ".class")) {
                if (in == null) throw new ClassNotFoundException(name);
                byte[] bytes = in.readAllBytes();
                return defineClass(name, bytes, 0, bytes.length);
            } catch (java.io.IOException error) {
                throw new ClassNotFoundException(name, error);
            }
        }
    }

    static void check(boolean condition) {
        if (!condition) throw new AssertionError("parent bypassed custom delegation");
    }

    static void check(boolean preload, String first) throws Exception {
        Class<?> parent = Element.class;
        Loader sibling = new Loader(null);
        Loader initiating = new Loader(sibling);
        Class<?> element = sibling.loadClass("Test$Element");
        check(element != parent);
        if (preload) check(Class.forName("Test$Element", false, initiating) == element);
        Class<?> factory = initiating.loadClass("Test$Factory");
        Object result = factory.getMethod(first).invoke(null);
        if (first.equals("tag")) check(result == element);
        if (first.equals("literal")) check(((Class<?>) result).getComponentType() == element);
        if (first.equals("array")) check(result.getClass().getComponentType() == element);
        if (first.equals("matrix")) {
            check(result.getClass().getComponentType().getComponentType() == element);
        }
        check(initiating.loaded("Test$Element") == element);
        int loads = initiating.elementLoads;
        check(factory.getMethod("tag").invoke(null) == element);
        check(((Class<?>) factory.getMethod("literal").invoke(null)).getComponentType() == element);
        check(factory.getMethod("array").invoke(null).getClass().getComponentType() == element);
        check(factory.getMethod("matrix").invoke(null).getClass()
                .getComponentType().getComponentType() == element);
        check(initiating.elementLoads == loads);
        System.out.println("sibling wins: " + preload + ", " + first);
    }

    public static void main(String[] args) throws Exception {
        for (String first : new String[] {"tag", "literal", "array", "matrix"}) {
            check(false, first);
            check(true, first);
        }
    }
}
