import java.io.InputStream;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;

public class Test {
    public static class Caller {
        public static void set(VarHandle handle, Value value) { handle.set(value); }
    }

    static class CopyLoader extends ClassLoader {
        Class<?> defineCopy(String name) throws Exception {
            try (InputStream in = Test.class.getResourceAsStream(name + ".class")) {
                if (in == null) throw new AssertionError("missing " + name);
                byte[] bytes = in.readAllBytes();
                return defineClass(name, bytes, 0, bytes.length);
            }
        }
    }

    public static void main(String[] args) throws Exception {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        CopyLoader first = new CopyLoader();
        Class<?> value = first.defineCopy("Value");
        Class<?> caller = first.defineCopy("Test$Caller");
        lookup.findStatic(caller, "set", MethodType.methodType(void.class, VarHandle.class, value));

        CopyLoader second = new CopyLoader();
        Class<?> otherValue = second.defineCopy("Value");
        Class<?> otherCaller = second.defineCopy("Test$Caller");
        try {
            lookup.findStatic(otherCaller, "set", MethodType.methodType(void.class, VarHandle.class, otherValue));
            throw new AssertionError("lookup accepted a conflicting definition of the parameter type");
        } catch (IllegalAccessException expected) {
            if (!(expected.getCause() instanceof LinkageError)) throw expected;
        }
        // The failed resolution must preserve the first loader constraint.
        lookup.findStatic(caller, "set", MethodType.methodType(void.class, VarHandle.class, value));
        try {
            Class.forName("Value", false, Test.class.getClassLoader());
            throw new AssertionError("late definition violated the existing constraint");
        } catch (LinkageError expected) {
            lookup.findStatic(caller, "set", MethodType.methodType(void.class, VarHandle.class, value));
        }
        System.out.println("Lookup enforced the parameter loader constraint");
    }
}
