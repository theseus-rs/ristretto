import java.io.DataInputStream;
import java.io.InputStream;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import sun.misc.Unsafe;

/** Constant-pool reflection must resolve in the owning definition's namespace. */
public class Test {
    static class Loader extends ClassLoader {
        Class<?> define(String name) throws Exception {
            byte[] bytes;
            try (InputStream in = Test.class.getResourceAsStream("/Fixture.class")) {
                bytes = in.readAllBytes();
            }
            byte[] from = "Fixture".getBytes("UTF-8"), to = name.getBytes("UTF-8");
            for (int i = 0; i <= bytes.length - from.length; i++) {
                boolean match = true;
                for (int j = 0; j < from.length; j++) {
                    if (bytes[i + j] != from[j]) match = false;
                }
                if (match) System.arraycopy(to, 0, bytes, i, to.length);
            }
            return defineClass(name, bytes, 0, bytes.length);
        }
    }

    static void check(boolean condition) {
        if (!condition) throw new AssertionError("constant-pool loader identity");
    }

    static MethodHandles.Lookup trustedLookup() throws Exception {
        Field field = Unsafe.class.getDeclaredField("theUnsafe");
        field.setAccessible(true);
        Unsafe unsafe = (Unsafe) field.get(null);
        Field impl = MethodHandles.Lookup.class.getDeclaredField("IMPL_LOOKUP");
        return (MethodHandles.Lookup) unsafe.getObject(
                unsafe.staticFieldBase(impl), unsafe.staticFieldOffset(impl));
    }

    static int[] constantTags() throws Exception {
        try (DataInputStream in = new DataInputStream(Test.class.getResourceAsStream("/Fixture.class"))) {
            in.readInt(); // magic
            in.readInt(); // version
            int[] tags = new int[in.readUnsignedShort()];
            for (int i = 1; i < tags.length; i++) {
                tags[i] = in.readUnsignedByte();
                switch (tags[i]) {
                    case 1: in.readUTF(); break;
                    case 3: case 4: case 9: case 10: case 11: case 12: case 17: case 18:
                        in.readInt(); break;
                    case 5: case 6: in.readLong(); i++; break;
                    case 7: case 8: case 16: case 19: case 20: in.readUnsignedShort(); break;
                    case 15: in.readUnsignedByte(); in.readUnsignedShort(); break;
                    default: throw new AssertionError("unknown constant tag");
                }
            }
            return tags;
        }
    }

    static void check(MethodHandles.Lookup lookup, Class<?> type) throws Throwable {
        Class<?> poolType = Class.forName("jdk.internal.reflect.ConstantPool");
        Object pool = lookup.findVirtual(Class.class, "getConstantPool",
                MethodType.methodType(poolType)).invoke(type);
        MethodHandle info = lookup.findVirtual(poolType, "getMemberRefInfoAt",
                MethodType.methodType(String[].class, int.class));
        int[] tags = constantTags();
        int checked = 0;
        for (int i = 1; i < tags.length; i++) {
            if (tags[i] == 7) {
                Class<?> resolved = (Class<?>) lookup.findVirtual(poolType, "getClassAt",
                        MethodType.methodType(Class.class, int.class)).invoke(pool, i);
                if (!resolved.getName().equals(type.getName())) continue;
                check(resolved == type);
                check(lookup.findVirtual(poolType, "getClassAtIfLoaded",
                        MethodType.methodType(Class.class, int.class)).invoke(pool, i) == type);
                checked++;
            } else if (tags[i] == 9 || tags[i] == 10) {
                String[] member = (String[]) info.invoke(pool, i);
                if (!member[0].equals(type.getName())) continue;
                for (String suffix : new String[] {"", "IfLoaded"}) {
                    if (tags[i] == 9) {
                        Field field = (Field) lookup.findVirtual(poolType, "getFieldAt" + suffix,
                                MethodType.methodType(Field.class, int.class)).invoke(pool, i);
                        check(field.getDeclaringClass() == type && field.getType() == type);
                    } else {
                        Object method = lookup.findVirtual(poolType, "getMethodAt" + suffix,
                                MethodType.methodType(java.lang.reflect.Member.class, int.class)).invoke(pool, i);
                        if (member[1].equals("<init>")) {
                            Constructor<?> constructor = (Constructor<?>) method;
                            check(constructor.getDeclaringClass() == type);
                            check(constructor.getParameterTypes()[0] == type);
                        } else {
                            Method identity = (Method) method;
                            check(identity.getDeclaringClass() == type && identity.getReturnType() == type);
                            Class<?>[] parameters = identity.getParameterTypes();
                            check(parameters[0] == type && parameters[1].getComponentType() == type);
                            check(parameters[2] == int.class);
                        }
                    }
                }
                checked++;
            }
        }
        check(checked == 4);
        System.out.println("constant-pool identity: " + type.getName());
    }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = trustedLookup();
        // Exercise both a parent-shadowing definition and names absent from the application loader.
        check(lookup, Fixture.class);
        check(lookup, new Loader().define("Fixture"));
        Class<?> first = new Loader().define("Private");
        Class<?> second = new Loader().define("Private");
        check(lookup, first);
        check(lookup, second);
        check(lookup, first);
        try {
            Class.forName("Private", false, Test.class.getClassLoader());
            throw new AssertionError("custom-only definition leaked to application loader");
        } catch (ClassNotFoundException expected) {}
    }
}
