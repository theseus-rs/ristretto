import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.lang.invoke.*;

public class Test {
    public static int value = 17;
    static int resolutions;

    public static VarHandle bootstrap(MethodHandles.Lookup caller, String name, Class<?> type) {
        resolutions++;
        return ConstantBootstraps.staticFieldVarHandle(caller, name, VarHandle.class, Test.class, int.class);
    }

    public static void main(String[] args) throws Throwable {
        MethodHandle read = reader("GoodConstant", "value");
        VarHandle first = (VarHandle) read.invokeExact();
        if ((VarHandle) read.invokeExact() != first || (int) first.getAndAdd(3) != 17 || value != 20 || resolutions != 1) {
            throw new AssertionError("resolved constant identity, storage or bootstrap count");
        }
        MethodHandle missing = reader("BadConstant", "absent");
        for (int i = 0; i < 2; i++) {
            try {
                VarHandle unexpected = (VarHandle) missing.invokeExact();
                throw new AssertionError(unexpected);
            } catch (NoSuchFieldError expected) {}
        }
        if (resolutions != 2) throw new AssertionError("retried a failed dynamic constant");
        System.out.println("Java 11 dynamic VarHandle constants passed");
    }

    // A version-55 class with one static method: ldc Dynamic(fieldName); areturn.
    static MethodHandle reader(String className, String fieldName) throws ReflectiveOperationException, IOException {
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(bytes);
        out.writeInt(0xcafebabe);
        out.writeShort(0);
        out.writeShort(55);
        out.writeShort(20);
        utf(out, className);                           // 1
        entry(out, 7, 1);                              // 2 Class
        utf(out, "java/lang/Object");                  // 3
        entry(out, 7, 3);                              // 4 Class
        utf(out, "get");                               // 5
        utf(out, "()Ljava/lang/invoke/VarHandle;");     // 6
        utf(out, "Code");                              // 7
        utf(out, "BootstrapMethods");                  // 8
        utf(out, "Test");                              // 9
        entry(out, 7, 9);                              // 10 Class
        utf(out, "bootstrap");                         // 11
        utf(out, "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/invoke/VarHandle;");
        entry(out, 12, 11, 12);                        // 13 NameAndType
        entry(out, 10, 10, 13);                        // 14 Methodref
        out.writeByte(15); out.writeByte(6); out.writeShort(14); // 15 REF_invokeStatic
        utf(out, fieldName);                           // 16
        utf(out, "Ljava/lang/invoke/VarHandle;");       // 17
        entry(out, 12, 16, 17);                        // 18 NameAndType
        entry(out, 17, 0, 18);                         // 19 Dynamic
        shorts(out, 0x21, 2, 4, 0, 0, 1);             // class header and method count
        shorts(out, 0x09, 5, 6, 1, 7);                // public static get, Code attribute
        out.writeInt(15);
        shorts(out, 1, 0);
        out.writeInt(3);
        out.write(new byte[]{0x12, 19, (byte) 0xb0});
        shorts(out, 0, 0, 1, 8);                       // end Code, BootstrapMethods attribute
        out.writeInt(6);
        shorts(out, 1, 15, 0);
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        Class<?> owner = lookup.defineClass(bytes.toByteArray());
        return lookup.findStatic(owner, "get", MethodType.methodType(VarHandle.class));
    }

    static void utf(DataOutputStream out, String value) throws IOException {
        out.writeByte(1);
        out.writeUTF(value);
    }

    static void entry(DataOutputStream out, int tag, int... operands) throws IOException {
        out.writeByte(tag);
        shorts(out, operands);
    }

    static void shorts(DataOutputStream out, int... values) throws IOException {
        for (int value : values) out.writeShort(value);
    }
}
