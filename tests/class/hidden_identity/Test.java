import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

/** Hidden definitions with the same class-file name must retain distinct methods. */
public class Test {
    static byte[] bytes(int value) throws Exception {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buffer);
        out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
        out.writeShort(13);
        out.writeByte(1); out.writeUTF("Generated"); // 1
        out.writeByte(7); out.writeShort(1); // 2
        out.writeByte(1); out.writeUTF("java/lang/Object"); // 3
        out.writeByte(7); out.writeShort(3); // 4
        out.writeByte(1); out.writeUTF("value"); // 5
        out.writeByte(1); out.writeUTF("()I"); // 6
        out.writeByte(1); out.writeUTF("Code"); // 7
        out.writeByte(1); out.writeUTF("self"); // 8
        out.writeByte(1); out.writeUTF("()Ljava/lang/Class;"); // 9
        out.writeByte(8); out.writeShort(1); // 10: literal shares the class-name UTF-8
        out.writeByte(1); out.writeUTF("literal"); // 11
        out.writeByte(1); out.writeUTF("()Ljava/lang/String;"); // 12
        out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
        out.writeShort(0); out.writeShort(0); out.writeShort(3);
        method(out, 5, 6, new byte[]{0x10, (byte) value, (byte) 0xac});
        method(out, 8, 9, new byte[]{0x12, 2, (byte) 0xb0});
        method(out, 11, 12, new byte[]{0x12, 10, (byte) 0xb0});
        out.writeShort(0);
        return buffer.toByteArray();
    }

    static void method(DataOutputStream out, int name, int descriptor, byte[] code) throws Exception {
        out.writeShort(9); out.writeShort(name); out.writeShort(descriptor);
        out.writeShort(1); out.writeShort(7); out.writeInt(12 + code.length);
        out.writeShort(1); out.writeShort(0); out.writeInt(code.length); out.write(code);
        out.writeShort(0); out.writeShort(0);
    }

    static void check(MethodHandles.Lookup lookup, Class<?> type, int expected) throws Throwable {
        MethodHandle value = lookup.findStatic(type, "value", MethodType.methodType(int.class));
        int actual = (int) value.invokeExact();
        if (actual != expected) throw new AssertionError("wrong definition: " + actual);
        MethodHandle self = lookup.findStatic(type, "self", MethodType.methodType(Class.class));
        if ((Class<?>) self.invokeExact() != type) throw new AssertionError("wrong self reference");
        MethodHandle literal = lookup.findStatic(type, "literal", MethodType.methodType(String.class));
        String name = (String) literal.invokeExact();
        if (!name.equals("Generated")) throw new AssertionError("renamed string literal: " + name);
        System.out.println("literal: " + name);
        System.out.println("value: " + actual);
    }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        Class<?> first = lookup.defineHiddenClass(bytes(11), false).lookupClass();
        Class<?> second = lookup.defineHiddenClass(bytes(22), false).lookupClass();
        if (first == second || first.getName().equals(second.getName())
                || !first.isHidden() || !second.isHidden()) {
            throw new AssertionError("hidden definitions must have unique identities");
        }
        check(lookup, first, 11);
        check(lookup, second, 22);
        Class<?> normal = lookup.defineClass(bytes(33));
        if (normal.isHidden()) throw new AssertionError("ordinary definition marked hidden");
        check(lookup, normal, 33);
        check(lookup, first, 11);
        check(lookup, second, 22);
    }
}
