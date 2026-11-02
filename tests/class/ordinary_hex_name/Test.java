import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.reflect.Array;

/** A legal internal name ending in /0x plus hexadecimal digits is not a hidden alias. */
public class Test {
    static final String INTERNAL_NAME = "ordinary/0x000000000000002a";
    static final String BINARY_NAME = "ordinary.0x000000000000002a";

    static byte[] bytes() throws Exception {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buffer);
        out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
        out.writeShort(8);
        out.writeByte(1); out.writeUTF(INTERNAL_NAME); // 1
        out.writeByte(7); out.writeShort(1); // 2
        out.writeByte(1); out.writeUTF("java/lang/Object"); // 3
        out.writeByte(7); out.writeShort(3); // 4
        out.writeByte(1); out.writeUTF("self"); // 5
        out.writeByte(1); out.writeUTF("()Ljava/lang/Class;"); // 6
        out.writeByte(1); out.writeUTF("Code"); // 7
        out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
        out.writeShort(0); out.writeShort(0); out.writeShort(1);
        // public static Class<?> self() { return <this class>.class; }
        out.writeShort(9); out.writeShort(5); out.writeShort(6);
        out.writeShort(1); out.writeShort(7); out.writeInt(15);
        out.writeShort(1); out.writeShort(0); out.writeInt(3);
        out.write(new byte[] {0x12, 2, (byte) 0xb0});
        out.writeShort(0); out.writeShort(0); out.writeShort(0);
        return buffer.toByteArray();
    }

    static class Loader extends ClassLoader {
        Class<?> define() throws Exception {
            byte[] code = bytes();
            return defineClass(BINARY_NAME, code, 0, code.length);
        }
    }

    public static void main(String[] args) throws Exception {
        Loader loader = new Loader();
        Class<?> defined = loader.define();
        System.out.println("name: " + defined.getName());
        System.out.println("ordinary definition: " + !defined.isHidden());
        System.out.println("binary lookup: " + (Class.forName(BINARY_NAME, false, loader) == defined));

        Object single = Array.newInstance(defined, 1);
        Object dimensions = Array.newInstance(defined, new int[] {1});
        Class<?> array = single.getClass();
        System.out.println("array name: " + array.getName());
        System.out.println("array component: " + (array.getComponentType() == defined));
        System.out.println("array overload identity: " + (array == dimensions.getClass()));
        System.out.println("array length: " + Array.getLength(single));
        System.out.println("symbolic self: " + (defined.getMethod("self").invoke(null) == defined));
    }
}
