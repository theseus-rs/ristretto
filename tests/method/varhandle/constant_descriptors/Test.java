import java.io.InputStream;
import java.lang.constant.ClassDesc;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.lang.invoke.VarHandle.VarHandleDesc;
import java.nio.ByteOrder;

public class Test {
    int number;
    static String text;

    public static class HiddenTarget {
        public int value = 13;
    }

    public static void main(String[] args) throws Exception {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        Test receiver = new Test();
        int[] array = {3};
        ClassDesc owner = Test.class.describeConstable().get();
        VarHandle field = resolve(lookup.findVarHandle(Test.class, "number", int.class),
                VarHandleDesc.ofField(owner, "number", int.class.describeConstable().get()), lookup);
        VarHandle staticField = resolve(lookup.findStaticVarHandle(Test.class, "text", String.class),
                VarHandleDesc.ofStaticField(owner, "text", String.class.describeConstable().get()), lookup);
        VarHandleDesc arrayDesc = VarHandleDesc.ofArray(int[].class.describeConstable().get());
        if (!arrayDesc.varType().equals(int.class.describeConstable().get())) throw new AssertionError("array descriptor type");
        VarHandle element = resolve(MethodHandles.arrayElementVarHandle(int[].class),
                arrayDesc, lookup);
        field.set(receiver, 7);
        staticField.set("resolved");
        int witness = (int) element.getAndAdd(array, 0, 2);
        if (receiver.number != 7 || !"resolved".equals(text) || witness != 3 || array[0] != 5) {
            throw new AssertionError("resolved handles must access original storage");
        }
        VarHandle view = MethodHandles.byteArrayViewVarHandle(int[].class, ByteOrder.BIG_ENDIAN);
        if (view.describeConstable().isPresent()) throw new AssertionError("unexpected byte-array view descriptor");
        ClassDesc integer = int.class.describeConstable().get();
        try {
            VarHandleDesc.ofField(owner, "missing", integer).resolveConstantDesc(lookup);
            throw new AssertionError("resolved a missing field");
        } catch (NoSuchFieldException expected) {}
        try {
            VarHandleDesc.ofStaticField(owner, "number", integer).resolveConstantDesc(lookup);
            throw new AssertionError("resolved an instance field as static");
        } catch (IllegalAccessException expected) {}
        hiddenField(lookup);
        System.out.println("VarHandle descriptors passed");
    }

    static void hiddenField(MethodHandles.Lookup lookup) throws Exception {
        MethodHandles.Lookup hidden;
        try (InputStream in = HiddenTarget.class.getResourceAsStream("Test$HiddenTarget.class")) {
            if (in == null) throw new AssertionError("missing hidden class template");
            hidden = lookup.defineHiddenClass(in.readAllBytes(), true);
        }
        Class<?> owner = hidden.lookupClass();
        Object target = owner.getConstructor().newInstance();
        VarHandle field = hidden.findVarHandle(owner, "value", int.class);
        int previous = (int) field.getAndAdd(target, 4);
        if (previous != 13 || owner.getField("value").getInt(target) != 17 || field.describeConstable().isPresent()) {
            throw new AssertionError("hidden field access or descriptor");
        }
    }

    static VarHandle resolve(VarHandle original, VarHandleDesc expected, MethodHandles.Lookup lookup)
            throws ReflectiveOperationException {
        VarHandleDesc description = original.describeConstable().get();
        if (!description.equals(expected)) throw new AssertionError(description);
        return description.resolveConstantDesc(lookup);
    }
}
