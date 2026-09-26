import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

/** Symbolic resolution must let invocation selection resolve competing defaults. */
public class Test {
    public interface A {
        default int value() { return 1; }
    }
    public interface B {
        default int value() { return 2; }
    }
    public static class Base {
        public int value() { return 4; }
    }
    public interface Invoker {
        int call(Object receiver);
    }
    public interface SpecialInvoker {
        int call();
    }
    public interface Single extends A {}
    public static class SingleSpecial implements Single, SpecialInvoker {
        public int value() { return 3; }
        public int call() { return Single.super.value(); }
    }

    public static class Loader extends ClassLoader {
        Class<?> define(String name, String parent, boolean isInterface, boolean caller,
                        boolean override) throws Exception {
            return define(name, parent, isInterface, caller, override, false);
        }

        Class<?> define(String name, String parent, boolean isInterface, boolean caller,
                        boolean override, boolean special) throws Exception {
            return define(name, parent, isInterface, caller, override, special, null);
        }

        Class<?> define(String name, String parent, boolean isInterface, boolean caller,
                        boolean override, boolean special, String classTarget) throws Exception {
            // These class files model separate compilation: X extends A and B
            // without redeclaring value(), even though both now provide defaults.
            // javac rejects this hierarchy if compiled together from source.
            ByteArrayOutputStream buffer = new ByteArrayOutputStream();
            DataOutputStream out = new DataOutputStream(buffer);
            out.writeInt(0xcafebabe); out.writeShort(0); out.writeShort(52);
            out.writeShort(26);
            out.writeByte(1); out.writeUTF(name); // 1
            out.writeByte(7); out.writeShort(1); // 2: this class
            out.writeByte(1); out.writeUTF(parent); // 3
            out.writeByte(7); out.writeShort(3); // 4: superclass
            out.writeByte(1); out.writeUTF("<init>"); // 5
            out.writeByte(1); out.writeUTF("()V"); // 6
            out.writeByte(12); out.writeShort(5); out.writeShort(6); // 7
            out.writeByte(10); out.writeShort(4); out.writeShort(7); // 8: super()
            out.writeByte(1); out.writeUTF("Code"); // 9
            out.writeByte(1); out.writeUTF("value"); // 10
            out.writeByte(1); out.writeUTF("()I"); // 11
            out.writeByte(12); out.writeShort(10); out.writeShort(11); // 12
            out.writeByte(1); out.writeUTF("Test$A"); // 13
            out.writeByte(7); out.writeShort(13); // 14
            out.writeByte(1); out.writeUTF("Test$B"); // 15
            out.writeByte(7); out.writeShort(15); // 16
            out.writeByte(1); out.writeUTF(classTarget == null ? "X" : classTarget); // 17
            out.writeByte(7); out.writeShort(17); // 18
            out.writeByte(classTarget == null ? 11 : 10);
            out.writeShort(18); out.writeShort(12); // 19: symbolic target's value
            out.writeByte(1); out.writeUTF("call"); // 20
            out.writeByte(1); out.writeUTF("(Ljava/lang/Object;)I"); // 21
            out.writeByte(1); out.writeUTF("Test$Invoker"); // 22
            out.writeByte(7); out.writeShort(22); // 23
            out.writeByte(1); out.writeUTF("Test$SpecialInvoker"); // 24
            out.writeByte(7); out.writeShort(24); // 25
            out.writeShort(isInterface ? 0x601 : 0x21);
            out.writeShort(2); out.writeShort(4);
            out.writeShort(isInterface || (special && classTarget == null) ? 2 : 1);
            if (isInterface) { out.writeShort(14); out.writeShort(16); }
            else if (special) {
                if (classTarget == null) out.writeShort(18);
                out.writeShort(25);
            }
            else out.writeShort(caller ? 23 : 18);
            out.writeShort(0); // fields
            out.writeShort(isInterface ? 0 : 1 + (override ? 1 : 0) + (caller || special ? 1 : 0));
            if (!isInterface) {
                method(out, 5, 6, 1, new byte[]{0x2a, (byte) 0xb7, 0, 8, (byte) 0xb1});
                if (override) method(out, 10, 11, 1, new byte[]{0x06, (byte) 0xac});
                if (caller) {
                    // aload_1; checkcast target; invokeinterface/invokevirtual value()I; ireturn
                    method(out, 20, 21, 2, classTarget == null
                        ? new byte[]{0x2b, (byte) 0xc0, 0, 18, (byte) 0xb9, 0, 19, 1, 0, (byte) 0xac}
                        : new byte[]{0x2b, (byte) 0xc0, 0, 18, (byte) 0xb6, 0, 19, (byte) 0xac});
                }
                if (special) {
                    // aload_0; invokespecial target.value()I; ireturn
                    method(out, 20, 11, 1, new byte[]{0x2a, (byte) 0xb7, 0, 19, (byte) 0xac});
                }
            }
            out.writeShort(0); // class attributes
            byte[] bytes = buffer.toByteArray();
            return defineClass(name, bytes, 0, bytes.length);
        }

        private void method(DataOutputStream out, int name, int descriptor, int locals,
                            byte[] code) throws Exception {
            out.writeShort(1); out.writeShort(name); out.writeShort(descriptor);
            out.writeShort(1); out.writeShort(9); out.writeInt(12 + code.length);
            out.writeShort(1); out.writeShort(locals);
            out.writeInt(code.length); out.write(code);
            out.writeShort(0); out.writeShort(0);
        }
    }

    public static void main(String[] args) throws Throwable {
        Loader loader = new Loader();
        Class<?> x = loader.define("X", "java/lang/Object", true, false, false);
        Object direct = loader.define("Direct", "java/lang/Object", false, false, true)
            .getConstructor().newInstance();
        Object inherited = loader.define("Inherited", "Test$Base", false, false, false)
            .getConstructor().newInstance();
        Object missing = loader.define("Missing", "java/lang/Object", false, false, false)
            .getConstructor().newInstance();
        Invoker invoker = (Invoker) loader.define("Caller", "java/lang/Object", false, true, false)
            .getConstructor().newInstance();
        // Reuse one invokeinterface site across failing and successful selections.
        for (int attempt = 0; attempt < 3; attempt++) {
            for (Object receiver : new Object[]{direct, inherited, missing, direct}) {
                try {
                    int value = invoker.call(receiver);
                    int expected = receiver == direct ? 3 : 4;
                    if (receiver == missing || value != expected) {
                        throw new AssertionError("incorrect receiver selection: " + value);
                    }
                    System.out.println((receiver == direct ? "direct: " : "inherited: ") + value);
                } catch (IncompatibleClassChangeError expected) {
                    if (receiver != missing) throw expected;
                    System.out.println("conflicting defaults rejected");
                }
            }
        }
        Object child = loader.define("OverrideMissing", "Missing", false, false, true)
            .getConstructor().newInstance();
        Invoker virtual = (Invoker) loader.define("VirtualCaller", "java/lang/Object",
            false, true, false, false, "Missing").getConstructor().newInstance();
        for (int attempt = 0; attempt < 3; attempt++) {
            for (Object receiver : new Object[]{child, missing, null, child}) {
                try {
                    int value = virtual.call(receiver);
                    if (receiver != child || value != 3) {
                        throw new AssertionError("incorrect virtual selection: " + value);
                    }
                    System.out.println("virtual override: " + value);
                } catch (IncompatibleClassChangeError expected) {
                    if (receiver != missing) throw expected;
                    System.out.println("conflicting virtual defaults rejected");
                } catch (NullPointerException expected) {
                    if (receiver != null) throw expected;
                    System.out.println("null virtual receiver rejected");
                }
            }
        }
        MethodHandles.Lookup virtualLookup = MethodHandles.lookup();
        MethodHandle virtualHandle = virtualLookup.findVirtual(missing.getClass(), "value", MethodType.methodType(int.class));
        if (virtualLookup.revealDirect(virtualHandle).getDeclaringClass() != missing.getClass()) {
            throw new AssertionError("conflicting virtual handle must retain the symbolic class");
        }
        for (int attempt = 0; attempt < 3; attempt++) {
            if ((int) virtualHandle.invoke(child) != 3) throw new AssertionError("incorrect virtual handle selection");
            System.out.println("virtual method handle override: 3");
            try {
                virtualHandle.invoke(missing);
                throw new AssertionError("virtual method handle must reject conflicting defaults");
            } catch (IncompatibleClassChangeError expected) {
                System.out.println("conflicting virtual method handle rejected");
            }
            try {
                virtualHandle.invoke((Object) null);
                throw new AssertionError("null virtual method handle receiver must fail first");
            } catch (NullPointerException expected) {
                System.out.println("null virtual method handle receiver rejected");
            }
        }
        if (new SingleSpecial().call() != 1) throw new AssertionError("incorrect super selection");
        System.out.println("non-conflicting super default: 1");
        MethodHandles.Lookup singleLookup = MethodHandles.privateLookupIn(SingleSpecial.class, MethodHandles.lookup());
        MethodHandle singleHandle = singleLookup
            .findSpecial(Single.class, "value", MethodType.methodType(int.class), SingleSpecial.class);
        if (singleLookup.revealDirect(singleHandle).getDeclaringClass() != A.class) {
            throw new AssertionError("inherited special handle must retain its declaring interface");
        }
        if ((int) singleHandle.invoke(new SingleSpecial()) != 1) {
            throw new AssertionError("incorrect special method handle selection");
        }
        System.out.println("non-conflicting special method handle: 1");
        for (boolean classTarget : new boolean[]{false, true}) {
            for (boolean override : new boolean[]{false, true}) {
                SpecialInvoker special = (SpecialInvoker) loader.define(
                    "Special" + classTarget + override, classTarget ? "Missing" : "java/lang/Object",
                    false, false, override, true, classTarget ? "Missing" : null)
                    .getConstructor().newInstance();
                for (int attempt = 0; attempt < 3; attempt++) {
                    try {
                        special.call();
                        throw new AssertionError("invokespecial must reject conflicting defaults");
                    } catch (IncompatibleClassChangeError expected) {
                        System.out.println("conflicting super defaults rejected, override=" + override);
                    }
                }
                MethodHandles.Lookup specialLookup = MethodHandles.privateLookupIn(special.getClass(), MethodHandles.lookup());
                Class<?> specialTarget = classTarget ? missing.getClass() : x;
                MethodHandle specialHandle = specialLookup
                    .findSpecial(specialTarget, "value", MethodType.methodType(int.class), special.getClass());
                if (specialLookup.revealDirect(specialHandle).getDeclaringClass() != specialTarget) {
                    throw new AssertionError("conflicting special handle must retain the symbolic target");
                }
                for (int attempt = 0; attempt < 3; attempt++) {
                    try {
                        specialHandle.invoke((Object) null);
                        throw new AssertionError("null special method handle receiver must fail first");
                    } catch (NullPointerException expected) {
                        System.out.println("null special method handle receiver rejected");
                    }
                    try {
                        specialHandle.invoke(special);
                        throw new AssertionError("special method handle must reject conflicting defaults");
                    } catch (IncompatibleClassChangeError expected) {
                        System.out.println("conflicting special method handle rejected, override=" + override);
                    }
                }
            }
        }
    }
}
