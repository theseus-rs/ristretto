/** Array lookups reuse successful component resolution for the same initiating loader. */
public class Test {
    static class Loader extends ClassLoader {
        int requests;
        boolean reject;

        Loader(boolean reject) {
            super(null);
            this.reject = reject;
        }

        @Override
        public Class<?> loadClass(String name) throws ClassNotFoundException {
            if (!name.equals("java.lang.String")) {
                throw new AssertionError("unexpected loadClass request: " + name);
            }
            requests++;
            if (reject || requests > 1) throw new ClassNotFoundException(name);
            return super.loadClass(name);
        }
    }

    static void check(boolean condition) {
        if (!condition) throw new AssertionError();
    }

    static void absent(Loader loader) throws Exception {
        try {
            Class.forName("[Ljava.lang.String;", false, loader);
            throw new AssertionError("another loader's resolution was reused");
        } catch (ClassNotFoundException expected) {
            check(loader.requests > 0);
        }
    }

    public static void main(String[] args) throws Exception {
        Class<?> expected = Class.forName("[Ljava.lang.String;", false, null);
        Loader first = new Loader(false);
        check(Class.forName("[Ljava.lang.String;", false, first) == expected);
        check(Class.forName("[Ljava.lang.String;", false, first) == expected);
        check(Class.forName("[[Ljava.lang.String;", true, first).getComponentType() == expected);
        check(Class.forName("java.lang.String", false, first) == String.class);
        check(first.requests == 1);
        System.out.println("repeated lookups reuse component resolution");

        Loader second = new Loader(false);
        check(Class.forName("[Ljava.lang.String;", false, second) == expected);
        check(Class.forName("[Ljava.lang.String;", true, second) == expected);
        check(second.requests == 1);
        System.out.println("each loader resolves its own component");

        Loader direct = new Loader(false);
        check(Class.forName("java.lang.String", false, direct) == String.class);
        check(Class.forName("[Ljava.lang.String;", false, direct) == expected);
        check(direct.requests == 1);
        System.out.println("ordinary lookup supplies component resolution");

        Loader rejecting = new Loader(true);
        absent(rejecting);
        absent(rejecting);
        check(rejecting.requests == 2);
        check(Class.forName("[[I", false, rejecting) == int[][].class);
        check(rejecting.requests == 2);
        // A failed resolution must not prevent a later successful attempt.
        rejecting.reject = false;
        rejecting.requests = 0;
        check(Class.forName("[Ljava.lang.String;", false, rejecting) == expected);
        check(Class.forName("[Ljava.lang.String;", false, rejecting) == expected);
        check(rejecting.requests == 1);
        System.out.println("failures remain isolated and retryable");
    }
}
