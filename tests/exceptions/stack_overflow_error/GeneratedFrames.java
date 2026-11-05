import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.reflect.Method;

/** Valid bytecode for frame shapes that javac cannot express directly. */
final class GeneratedFrames extends ClassLoader {
    static Method method(String mode) throws Exception {
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(bytes);
        out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
        out.writeShort(24);
        utf(out, "Generated");                         // 1
        entry(out, 7, 1);                              // 2 Class
        utf(out, "java/lang/Object");                  // 3
        entry(out, 7, 3);                              // 4 Class
        utf(out, "run"); utf(out, "()V");              // 5, 6
        utf(out, "Code");                              // 7
        pair(out, 12, 5, 6);                           // 8 NameAndType
        pair(out, 10, 2, 8);                           // 9 Methodref Generated.run
        utf(out, "Test"); entry(out, 7, 10);           // 10, 11
        utf(out, "enter"); pair(out, 12, 12, 6);       // 12, 13
        pair(out, 10, 11, 13);                         // 14 Test.enter
        utf(out, "BootstrapMethods");                 // 15
        utf(out, "bootstrap");                        // 16
        utf(out, "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/CallSite;"); // 17
        pair(out, 12, 16, 17); pair(out, 10, 11, 18);  // 18, 19
        out.writeByte(15); out.writeByte(6); out.writeShort(19); // 20 REF_invokeStatic
        pair(out, 18, 0, 8);                           // 21 InvokeDynamic
        utf(out, "unused"); utf(out, "I");             // 22, 23
        out.writeShort(0x21); out.writeShort(2); out.writeShort(4);
        out.writeShort(0); out.writeShort(0); out.writeShort(1); // interfaces, fields, methods
        out.writeShort(9); out.writeShort(5); out.writeShort(6); // public static run()V
        out.writeShort(1); out.writeShort(7);
        ByteArrayOutputStream codeBytes = new ByteArrayOutputStream();
        DataOutputStream code = new DataOutputStream(codeBytes);
        code.writeByte(0xb8); code.writeShort(14); // Test.enter
        int operands = mode.equals("operand-stack") ? 256 : 0;
        for (int i = 0; i < operands; i++) code.writeByte(3); // iconst_0, live across recursive call
        if (mode.equals("invokedynamic")) {
            code.writeByte(0xba); code.writeShort(21); code.writeShort(0);
        } else {
            code.writeByte(0xb8); code.writeShort(9);
        }
        for (int i = 0; i < operands; i++) code.writeByte(0x57); // pop
        code.writeByte(0xb1); // return
        byte[] instructions = codeBytes.toByteArray();
        out.writeInt(12 + instructions.length);
        out.writeShort(operands);
        out.writeShort(mode.equals("large-frame") ? 65535 : 0);
        out.writeInt(instructions.length); out.write(instructions);
        out.writeShort(0); out.writeShort(0); // exception table, code attributes
        out.writeShort(1); out.writeShort(15); // class attributes: BootstrapMethods
        out.writeInt(6); out.writeShort(1); out.writeShort(20); out.writeShort(0);
        byte[] classFile = bytes.toByteArray();
        Class<?> type = new GeneratedFrames().defineClass(null, classFile, 0, classFile.length);
        return type.getMethod("run");
    }

    private static void utf(DataOutputStream out, String value) throws Exception {
        out.writeByte(1); out.writeUTF(value);
    }

    private static void entry(DataOutputStream out, int tag, int index) throws Exception {
        out.writeByte(tag); out.writeShort(index);
    }

    private static void pair(DataOutputStream out, int tag, int first, int second) throws Exception {
        out.writeByte(tag); out.writeShort(first); out.writeShort(second);
    }
}
