import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Array;
import java.lang.reflect.Field;
import java.security.ProtectionDomain;
import sun.misc.Unsafe;

/** Unsafe definitions with a null loader and their arrays belong to bootstrap. */
public class Test {
    static int staticValue = 3;
    int instanceValue = 7;
    static int initializations;
    static class Prepared {
        static Object value;
        static double doubleValue;
        static { initializations++; }
    }
    float floatValue;
    static byte[] bytes(String name) throws Exception {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buffer);
        out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
        out.writeShort(5);
        out.writeByte(1); out.writeUTF(name);
        out.writeByte(7); out.writeShort(1);
        out.writeByte(1); out.writeUTF("java/lang/Object");
        out.writeByte(7); out.writeShort(3);
        out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
        out.writeShort(0); out.writeShort(0); out.writeShort(0); out.writeShort(0);
        return buffer.toByteArray();
    }

    public static void main(String[] args) throws Throwable {
        Field unsafeField = Unsafe.class.getDeclaredField("theUnsafe");
        unsafeField.setAccessible(true);
        Unsafe unsafe = (Unsafe) unsafeField.get(null);
        Field prepared = Prepared.class.getDeclaredField("value");
        Object base = unsafe.staticFieldBase(prepared);
        long offset = unsafe.staticFieldOffset(prepared);
        Object marker = new Object();
        if (unsafe.getObject(base, offset) != null || initializations != 0)
            throw new AssertionError("raw read initialized the class");
        unsafe.putObject(base, offset, marker);
        if (unsafe.getObject(base, offset) != marker || initializations != 0)
            throw new AssertionError("raw write initialized the class");
        unsafe.putObject(base, offset, null);
        if (initializations != 0 || !unsafe.compareAndSwapObject(base, offset, null, marker)
                || initializations != 0) throw new AssertionError("raw reference CAS initialized the class");
        Field doubleField = Prepared.class.getDeclaredField("doubleValue");
        Object doubleBase = unsafe.staticFieldBase(doubleField);
        long doubleOffset = unsafe.staticFieldOffset(doubleField);
        long doubleNaN = 0x7ff8000000000042L;
        unsafe.putLong(doubleBase, doubleOffset, Long.MIN_VALUE);
        if (unsafe.compareAndSwapLong(doubleBase, doubleOffset, 0L, doubleNaN)
                || !unsafe.compareAndSwapLong(doubleBase, doubleOffset, Long.MIN_VALUE, doubleNaN)
                || Double.doubleToRawLongBits(unsafe.getDouble(doubleBase, doubleOffset)) != doubleNaN
                || initializations != 0) throw new AssertionError("raw double CAS on prepared field");
        if (Prepared.value != marker || initializations != 1) throw new AssertionError("prepared field value");
        Test receiver = new Test();
        long floatOffset = unsafe.objectFieldOffset(Test.class.getDeclaredField("floatValue"));
        int floatNaN = 0x7fc00042;
        unsafe.putInt(receiver, floatOffset, floatNaN);
        if (Float.floatToRawIntBits(receiver.floatValue) != floatNaN
                || unsafe.compareAndSwapInt(receiver, floatOffset, floatNaN + 1, 0)
                || !unsafe.compareAndSwapInt(receiver, floatOffset, floatNaN, Integer.MIN_VALUE)
                || unsafe.getInt(receiver, floatOffset) != Integer.MIN_VALUE)
            throw new AssertionError("raw float CAS lost NaN payload or sign of zero");
        Field statics = Test.class.getDeclaredField("staticValue");
        Field instance = Test.class.getDeclaredField("instanceValue");
        try {
            unsafe.objectFieldOffset(statics);
            throw new AssertionError("instance offset accepted a static field");
        } catch (IllegalArgumentException expected) {}
        try {
            unsafe.staticFieldOffset(instance);
            throw new AssertionError("static offset accepted an instance field");
        } catch (IllegalArgumentException expected) {}
        if (unsafe.getInt(new Test(), unsafe.objectFieldOffset(instance)) != 7
                || unsafe.getInt(unsafe.staticFieldBase(statics), unsafe.staticFieldOffset(statics)) != 3) {
            throw new AssertionError("field offset selected the wrong storage");
        }
        // Unsafe reads prepared storage. Initialize Lookup before reading its static singleton.
        Class.forName(MethodHandles.Lookup.class.getName(), true, null);
        Field lookupField = MethodHandles.Lookup.class.getDeclaredField("IMPL_LOOKUP");
        MethodHandles.Lookup trusted = (MethodHandles.Lookup) unsafe.getObject(
                unsafe.staticFieldBase(lookupField), unsafe.staticFieldOffset(lookupField));
        Class<?> unsafeClass = Class.forName("jdk.internal.misc.Unsafe");
        Object internalUnsafe = trusted.findStatic(unsafeClass, "getUnsafe",
                MethodType.methodType(unsafeClass)).invoke();
        MethodHandle define = trusted.findVirtual(unsafeClass, "defineClass", MethodType.methodType(
                Class.class, String.class, byte[].class, int.class, int.class,
                ClassLoader.class, ProtectionDomain.class)).bindTo(internalUnsafe);
        for (int order = 0; order < 2; order++) {
            String name = "UnsafeBootstrapElement" + order;
            byte[] code = bytes(name);
            Class<?> component = (Class<?>) define.invoke(name, code, 0, code.length, null, null);
            System.out.println("component bootstrap: " + (component.getClassLoader() == null));
            System.out.println("bootstrap lookup: " + (Class.forName(name, false, null) == component));
            Class<?> first = order == 0 ? Array.newInstance(component, 1).getClass()
                    : Array.newInstance(component, new int[] {1}).getClass();
            Class<?> second = Array.newInstance(component, 2).getClass();
            Class<?> matrix = Array.newInstance(component, new int[] {1, 1}).getClass();
            System.out.println("array bootstrap: " + (first.getClassLoader() == null));
            System.out.println("matrix bootstrap: " + (matrix.getClassLoader() == null));
            System.out.println("component identity: " + (first.getComponentType() == component));
            System.out.println("cached array: " + (first == second));
            System.out.println("matrix component: " + (matrix.getComponentType() == first));
            System.out.println("bootstrap array lookup: " + (Class.forName("[L" + name + ";", false, null) == first));
        }
        for (ClassLoader loader : new ClassLoader[] {Test.class.getClassLoader(), new ClassLoader() {}}) {
            String name = loader == Test.class.getClassLoader() ? "UnsafeAppElement" : "UnsafeCustomElement";
            byte[] code = bytes(name);
            Class<?> component = (Class<?>) define.invoke(name, code, 0, code.length, loader, null);
            Class<?> array = Array.newInstance(component, 1).getClass();
            System.out.println("explicit component loader: " + (component.getClassLoader() == loader));
            System.out.println("explicit array loader: " + (array.getClassLoader() == loader));
        }
    }
}
