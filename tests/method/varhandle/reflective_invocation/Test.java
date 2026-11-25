import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

public class Test {
    interface IntReader {
        int read(int[] values, int index);
    }

    interface IntWriter { void write(int[] values, int index, int value); }
    interface IntCas { boolean swap(int[] values, int index, int expected, int value); }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        VarHandle element = MethodHandles.arrayElementVarHandle(int[].class);
        int[] values = {7};
        MethodType addType = MethodType.methodType(int.class, int[].class, int.class, int.class);
        MethodHandle add = lookup.findVirtual(VarHandle.class, "getAndAdd", addType);
        int witness = (int) add.invokeExact(element, values, 0, 2);
        IntReader read = element::get;
        if (witness != 7 || values[0] != 9 || read.read(values, 0) != 9) {
            throw new AssertionError("typed invoker or bound method reference");
        }

        IntWriter write = element::set;
        IntCas swap = element::compareAndSet;
        write.write(values, 0, 13);
        if (swap.swap(values, 0, 9, 17) || !swap.swap(values, 0, 13, 17) || values[0] != 17) {
            throw new AssertionError("setter or CAS method reference");
        }

        Method get = VarHandle.class.getDeclaredMethod("get", Object[].class);
        Object[] coordinates = {values, 0};
        MethodHandle unreflected = lookup.unreflect(get);
        try {
            Object result = (Object) unreflected.invokeExact(element, coordinates);
            throw new AssertionError("unreflected placeholder returned " + result);
        } catch (UnsupportedOperationException expected) {
            // unreflect invokes the placeholder with its declared Object[] signature.
        }
        try {
            get.invoke(element, (Object) coordinates);
            throw new AssertionError("reflected placeholder returned normally");
        } catch (InvocationTargetException expected) {
            // The compatibility harness compares the cause with the matching reference JDK.
            System.out.println("reflected get cause: " + expected.getCause());
        }
        try {
            get.invoke(element, (Object) new int[0]);
            throw new AssertionError("accepted a primitive array as Object[]");
        } catch (IllegalArgumentException expected) {
            // Reflection rejects the argument before entering the placeholder.
        }
        for (String name : new String[] {"invoke", "invokeExact"}) {
            Method method = MethodHandle.class.getMethod(name, Object[].class);
            try {
                method.invoke(add, (Object) null);
                throw new AssertionError("reflected MethodHandle." + name + " returned");
            } catch (InvocationTargetException expected) {
                if (!(expected.getCause() instanceof UnsupportedOperationException)) throw expected;
            }
        }
    }
}
