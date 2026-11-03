import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/** Class.forName must honor its loader argument, including for array components. */
public class Test {
    static byte[] bytes(String name, boolean probe) throws Exception {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buffer);
        out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
        out.writeShort(20);
        out.writeByte(1); out.writeUTF(name); // 1
        out.writeByte(7); out.writeShort(1); // 2
        out.writeByte(1); out.writeUTF("java/lang/Object"); // 3
        out.writeByte(7); out.writeShort(3); // 4
        out.writeByte(1); out.writeUTF("lookup"); // 5
        out.writeByte(1); out.writeUTF("(Ljava/lang/String;Ljava/lang/Boolean;Ljava/lang/ClassLoader;)Ljava/lang/Class;"); // 6
        out.writeByte(1); out.writeUTF("Code"); // 7
        out.writeByte(1); out.writeUTF("java/lang/Class"); // 8
        out.writeByte(7); out.writeShort(8); // 9
        out.writeByte(1); out.writeUTF("forName"); // 10
        out.writeByte(12); out.writeShort(10); out.writeShort(13); // 11
        out.writeByte(10); out.writeShort(9); out.writeShort(11); // 12
        out.writeByte(1); out.writeUTF("(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;"); // 13
        out.writeByte(1); out.writeUTF("java/lang/Boolean"); // 14
        out.writeByte(7); out.writeShort(14); // 15
        out.writeByte(1); out.writeUTF("booleanValue"); // 16
        out.writeByte(1); out.writeUTF("()Z"); // 17
        out.writeByte(12); out.writeShort(16); out.writeShort(17); // 18
        out.writeByte(10); out.writeShort(15); out.writeShort(18); // 19
        out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
        out.writeShort(0); out.writeShort(0); out.writeShort(probe ? 1 : 0);
        if (probe) {
            byte[] code = {0x2a, 0x2b, (byte) 0xb6, 0, 19, 0x2c, (byte) 0xb8, 0, 12, (byte) 0xb0};
            out.writeShort(9); out.writeShort(5); out.writeShort(6);
            out.writeShort(1); out.writeShort(7); out.writeInt(12 + code.length);
            out.writeShort(3); out.writeShort(3); out.writeInt(code.length); out.write(code);
            out.writeShort(0); out.writeShort(0);
        }
        out.writeShort(0);
        return buffer.toByteArray();
    }

    static class Loader extends ClassLoader {
        final String elementName;
        int finds;

        Loader(String elementName) { this.elementName = elementName; }

        protected Class<?> findClass(String name) throws ClassNotFoundException {
            if (!name.equals(elementName) && !name.equals("LoaderScopeProbe")) {
                throw new ClassNotFoundException(name);
            }
            try {
                byte[] code = bytes(name, name.equals("LoaderScopeProbe"));
                if (name.equals(elementName)) finds++;
                return defineClass(name, code, 0, code.length);
            } catch (Exception error) {
                throw new ClassNotFoundException(name, error);
            }
        }
    }

    static Class<?> lookup(Method probe, String name, boolean initialize, ClassLoader loader)
            throws Exception {
        try {
            return (Class<?>) probe.invoke(null, name, initialize, loader);
        } catch (InvocationTargetException error) {
            if (error.getCause() instanceof ClassNotFoundException) {
                throw (ClassNotFoundException) error.getCause();
            }
            throw error;
        }
    }

    static void absent(Method probe, String name, boolean initialize, ClassLoader loader)
            throws Exception {
        try {
            lookup(probe, name, initialize, loader);
            throw new AssertionError("unexpectedly found " + name);
        } catch (ClassNotFoundException expected) {
            System.out.println("not found: " + name + ", initialize=" + initialize);
        }
    }

    public static void main(String[] args) throws Exception {
        Loader caller = new Loader("OnlyInCaller");
        Method probe = caller.loadClass("LoaderScopeProbe").getMethod(
                "lookup", String.class, Boolean.class, ClassLoader.class);
        for (boolean initialize : new boolean[] {false, true}) {
            absent(probe, "OnlyInCaller", initialize, null);
            absent(probe, "[LOnlyInCaller;", initialize, null);
        }
        System.out.println("caller untouched: " + (caller.finds == 0));
        Class<?> element = caller.loadClass("OnlyInCaller");
        Class<?> array = lookup(probe, "[LOnlyInCaller;", false, caller);
        System.out.println("component: " + (array.getComponentType() == element));
        System.out.println("array loader: " + (array.getClassLoader() == caller));
        for (boolean initialize : new boolean[] {false, true}) {
            absent(probe, "OnlyInCaller", initialize, null);
            absent(probe, "[LOnlyInCaller;", initialize, null);
        }
        // The requested loader differs from every caller on the stack.
        Loader requested = new Loader("OnlyInRequested");
        Class<?> requestedArray = lookup(probe, "[[LOnlyInRequested;", false, requested);
        System.out.println("requested component loader: "
                + (requestedArray.getComponentType().getComponentType().getClassLoader() == requested));
        System.out.println("requested definitions: " + requested.finds);
        System.out.println("requested array loader: " + (requestedArray.getClassLoader() == requested));
        ClassLoader rejecting = new ClassLoader(null) {
            public Class<?> loadClass(String name) throws ClassNotFoundException {
                throw new ClassNotFoundException(name);
            }
        };
        absent(probe, "[LOnlyInCaller;", false, rejecting);
        System.out.println("bootstrap class: " + (lookup(probe, "java.lang.String", false, null) == String.class));
        System.out.println("bootstrap array: " + (lookup(probe, "[[Ljava.lang.String;", false, null) == String[][].class));
        System.out.println("primitive array: " + (lookup(probe, "[[I", false, null) == int[][].class));
    }
}
