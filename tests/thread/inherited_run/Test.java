import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;

/** Thread.start() must select the nearest inherited, non-private run() implementation. */
public class Test {
    public static class BaseThread extends Thread {
        public int result;

        @Override
        public void run() {
            result = 1;
        }
    }

    public static class MiddleThread extends BaseThread {}

    public static class LeafThread extends MiddleThread {}

    public static class OverrideThread extends BaseThread {
        @Override
        public void run() {
            result = 2;
        }
    }

    public static class OverrideLeafThread extends OverrideThread {}

    public static void main(String[] args) throws Exception {
        checkRun("inherited run", new LeafThread(), 1);
        checkRun("direct override", new OverrideThread(), 2);
        checkRun("inherited override", new OverrideLeafThread(), 2);

        FixtureLoader loader = new FixtureLoader();
        Class<?> privateThread = loader.defineThread("PrivateRunThread", "Test$BaseThread", true);
        Class<?> privateLeaf = loader.defineThread("PrivateRunLeafThread", "PrivateRunThread", false);
        checkRun("private receiver method", (BaseThread) privateThread.getConstructor().newInstance(), 1);
        checkRun("private ancestor method", (BaseThread) privateLeaf.getConstructor().newInstance(), 1);
    }

    private static void checkRun(String label, BaseThread thread, int expected) throws Exception {
        thread.start();
        thread.join(5000);
        if (thread.isAlive() || thread.getState() != Thread.State.TERMINATED) {
            throw new AssertionError(label + ": thread did not terminate");
        }
        if (thread.result != expected) {
            throw new AssertionError(label + ": expected " + expected + ", got " + thread.result);
        }
        System.out.println(label + ": " + thread.result);
    }

    private static class FixtureLoader extends ClassLoader {
        // Java source rejects a private run() in a Thread subclass. Generate valid Java 8
        // bytecode with a private run() that writes -1, so executing it fails checkRun().
        Class<?> defineThread(String name, String parent, boolean privateRun) throws Exception {
            ByteArrayOutputStream buffer = new ByteArrayOutputStream();
            DataOutputStream out = new DataOutputStream(buffer);
            out.writeInt(0xCAFEBABE);
            out.writeShort(0);
            out.writeShort(52); // Java 8
            out.writeShort(17); // constant_pool_count
            out.writeByte(1); out.writeUTF(name);                // #1 Utf8 name
            out.writeByte(7); out.writeShort(1);                 // #2 Class name
            out.writeByte(1); out.writeUTF(parent);              // #3 Utf8 parent
            out.writeByte(7); out.writeShort(3);                 // #4 Class parent
            out.writeByte(1); out.writeUTF("<init>");            // #5
            out.writeByte(1); out.writeUTF("()V");               // #6
            out.writeByte(12); out.writeShort(5); out.writeShort(6); // #7 NameAndType <init>()V
            out.writeByte(10); out.writeShort(4); out.writeShort(7); // #8 Methodref parent.<init>
            out.writeByte(1); out.writeUTF("Code");              // #9
            out.writeByte(1); out.writeUTF("run");               // #10
            out.writeByte(1); out.writeUTF("Test$BaseThread");   // #11
            out.writeByte(7); out.writeShort(11);                // #12 Class BaseThread
            out.writeByte(1); out.writeUTF("result");            // #13
            out.writeByte(1); out.writeUTF("I");                 // #14
            out.writeByte(12); out.writeShort(13); out.writeShort(14); // #15 NameAndType result:I
            out.writeByte(9); out.writeShort(12); out.writeShort(15);  // #16 Fieldref BaseThread.result
            out.writeShort(0x0021); // ACC_PUBLIC | ACC_SUPER
            out.writeShort(2); // this_class
            out.writeShort(4); // super_class
            out.writeShort(0); // interfaces_count
            out.writeShort(0); // fields_count
            out.writeShort(privateRun ? 2 : 1); // methods_count
            // public <init>() { super(); }
            writeMethod(out, 0x0001, 5, 1, new byte[] {0x2a, (byte) 0xb7, 0, 8, (byte) 0xb1});
            if (privateRun) {
                // private run() { this.result = -1; }
                writeMethod(out, 0x0002, 10, 2, new byte[] {0x2a, 0x02, (byte) 0xb5, 0, 16, (byte) 0xb1});
            }
            out.writeShort(0); // class attributes_count
            out.flush();
            byte[] bytes = buffer.toByteArray();
            return defineClass(name, bytes, 0, bytes.length);
        }

        private static void writeMethod(DataOutputStream out, int flags, int name, int maxStack,
                                        byte[] code) throws Exception {
            out.writeShort(flags);
            out.writeShort(name);
            out.writeShort(6); // descriptor: ()V
            out.writeShort(1); // attributes_count
            out.writeShort(9); // Code
            out.writeInt(12 + code.length);
            out.writeShort(maxStack);
            out.writeShort(1); // max_locals: this
            out.writeInt(code.length);
            out.write(code);
            out.writeShort(0); // exception_table_length
            out.writeShort(0); // Code attributes_count
        }
    }
}
