import fields.Target;
import java.io.InputStream;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;
import java.lang.invoke.WrongMethodTypeException;
import java.util.Collections;

public class Test extends Target {
    static class CopyLoader extends ClassLoader {
        Class<?> define(byte[] bytes) {
            return defineClass("fields.Target", bytes, 0, bytes.length);
        }
    }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        VarHandle inherited = lookup.findVarHandle(Target.class, "inherited", int.class);
        if (!inherited.coordinateTypes().equals(Collections.singletonList(Test.class))) {
            throw new AssertionError("protected receiver type " + inherited.coordinateTypes());
        }
        Test child = new Test();
        inherited.set(child, 11);
        if (child.inherited != 11) throw new AssertionError("protected field write");
        rejectReceiver(inherited, new Target());
        VarHandle secret = MethodHandles.privateLookupIn(Target.class, lookup).findVarHandle(Target.class, "secret", int.class);
        if ((int) secret.getAndSet(child, 13) != 7 || child.secret() != 13) throw new AssertionError("private lookup");

        byte[] bytes;
        try (InputStream in = Target.class.getResourceAsStream("Target.class")) {
            if (in == null) throw new AssertionError("missing Target.class");
            bytes = in.readAllBytes();
        }
        Class<?> left = new CopyLoader().define(bytes);
        Class<?> right = new CopyLoader().define(bytes);
        Object first = left.getConstructor().newInstance();
        Object second = right.getConstructor().newInstance();
        VarHandle leftStatic = lookup.findStaticVarHandle(left, "shared", int.class);
        VarHandle rightStatic = lookup.findStaticVarHandle(right, "shared", int.class);
        VarHandle leftField = lookup.findVarHandle(left, "number", int.class);
        VarHandle rightField = lookup.findVarHandle(right, "number", int.class);
        for (int i = 0; i < 4; i++) {
            setStatic(leftStatic, 10 + i);
            setStatic(rightStatic, 20 + i);
            setField(leftField, first, 30 + i);
            setField(rightField, second, 40 + i);
        }
        rejectReceiver(leftField, second);
        rejectReceiver(rightField, first);
        if (left.getField("shared").getInt(null) != 13 || right.getField("shared").getInt(null) != 23
                || left.getField("number").getInt(first) != 33 || right.getField("number").getInt(second) != 43) {
            throw new AssertionError("storage or receiver shared across defining loaders");
        }
        typedAccess(lookup, left, right, first, second);
        System.out.println("VarHandle lookup and loader identity passed");
    }

    static void typedAccess(MethodHandles.Lookup lookup, Class<?> left, Class<?> right, Object first, Object second)
            throws Throwable {
        VarHandle field = lookup.in(left).findStaticVarHandle(left, "peer", left);
        MethodType get = MethodType.methodType(Object.class, VarHandle.class);
        MethodType set = MethodType.methodType(void.class, VarHandle.class, Object.class);
        MethodHandle readLeft = lookup.findStatic(left, "readPeer", get);
        MethodHandle writeLeft = lookup.findStatic(left, "writePeer", set);
        MethodHandle readRight = lookup.findStatic(right, "readPeer", get);
        MethodHandle writeRight = lookup.findStatic(right, "writePeer", set);

        writeLeft.invokeExact(field, first);
        try {
            writeRight.invokeExact(field, second);
            throw new AssertionError("adapted store accepted another loader's value");
        } catch (ClassCastException expected) {}
        try {
            Object value = (Object) readRight.invokeExact(field);
            throw new AssertionError("adapted load returned " + value);
        } catch (ClassCastException expected) {}
        if (Runtime.version().feature() >= 16) {
            // Resolve the newer API by name so the same fixture compiles on Java 11.
            MethodHandle makeExact = lookup.findVirtual(VarHandle.class, "withInvokeExactBehavior",
                    MethodType.methodType(VarHandle.class));
            VarHandle exact = (VarHandle) makeExact.invokeExact(field);
            writeLeft.invokeExact(exact, first);
            try {
                writeRight.invokeExact(exact, second);
                throw new AssertionError("exact store linked across loaders");
            } catch (WrongMethodTypeException expected) {}
            try {
                Object value = (Object) readRight.invokeExact(exact);
                throw new AssertionError("exact load returned " + value);
            } catch (WrongMethodTypeException expected) {}
            if ((Object) readLeft.invokeExact(exact) != first) throw new AssertionError("exact reference load");
        }
        if ((Object) readLeft.invokeExact(field) != first || right.getField("peer").get(null) != null) {
            throw new AssertionError("reference storage after rejected access");
        }
    }

    static void setStatic(VarHandle field, int value) { field.set(value); }
    static void setField(VarHandle field, Object receiver, int value) { field.set(receiver, value); }

    static void rejectReceiver(VarHandle field, Object receiver) {
        try {
            setField(field, receiver, -1);
            throw new AssertionError("accepted wrong receiver");
        } catch (ClassCastException expected) {}
    }
}
