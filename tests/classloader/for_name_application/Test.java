/** Caller-based application lookup must remain distinct from bootstrap lookup. */
public class Test {
    public static class Reflected {}

    public Reflected reflected() { return null; }

    static void bootstrapAbsent(String name, boolean initialize) throws Exception {
        try {
            Class.forName(name, initialize, null);
            throw new AssertionError("bootstrap found application class: " + name);
        } catch (ClassNotFoundException expected) {
            System.out.println("bootstrap absent: " + name + ", initialize=" + initialize);
        }
    }

    public static void main(String[] args) throws Exception {
        // Exercise the caller-based overload before asking for any loader explicitly.
        Class<?> caller = Class.forName("Test");
        ClassLoader loader = caller.getClassLoader();
        System.out.println("caller identity: " + (caller == Test.class));
        System.out.println("application loader present: " + (loader != null));
        System.out.println("system loader identity: " + (loader == ClassLoader.getSystemClassLoader()));
        Class<?> reflected = Test.class.getMethod("reflected").getReturnType();
        System.out.println("reflected loader identity: " + (reflected.getClassLoader() == loader));
        System.out.println("reflected lookup: " + (Class.forName("Test$Reflected") == reflected));
        System.out.println("array identity: " + (Class.forName("[LTest;") == Test[].class));
        System.out.println("nested array identity: " + (Class.forName("[[LTest;") == Test[][].class));
        System.out.println("array loader identity: " + (Test[][].class.getClassLoader() == loader));
        for (boolean initialize : new boolean[] {false, true}) {
            System.out.println("explicit application lookup: "
                    + (Class.forName("Test", initialize, loader) == Test.class));
            for (String name : new String[] {"Test", "Test$Reflected", "[LTest;", "[[LTest;"}) {
                bootstrapAbsent(name, initialize);
            }
        }
        System.out.println("bootstrap class: " + (Class.forName("java.lang.String", false, null) == String.class));
        System.out.println("bootstrap array: " + (Class.forName("[Ljava.lang.String;", false, null) == String[].class));
        System.out.println("bootstrap loader: " + (String.class.getClassLoader() == null));
    }
}
