import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;

/** A static field VarHandle created during its class's initialization must update the field. */
public class Test {
    static class Holder {
        static Object value;
        static final boolean SWAPPED;

        static {
            try {
                VarHandle handle = MethodHandles.lookup()
                    .findStaticVarHandle(Holder.class, "value", Object.class);
                SWAPPED = handle.compareAndSet((Object) null, (Object) "updated");
            } catch (ReflectiveOperationException e) {
                throw new ExceptionInInitializerError(e);
            }
        }
    }

    static int initializations;

    public static class Other {
        public static int value = initialize();

        static int initialize() {
            initializations++;
            return 41;
        }
    }

    public static class Broken {
        public static int value = initialize();

        static int initialize() {
            throw new IllegalStateException("initializer failed");
        }
    }

    static class Text {
        static String value;
        static { initializations++; }
    }

    public static void main(String[] args) throws ReflectiveOperationException {
        inheritedStatics();
        System.out.println("compareAndSet: " + Holder.SWAPPED);
        System.out.println("value: " + Holder.value);
        VarHandle external = MethodHandles.lookup().findStaticVarHandle(Other.class, "value", int.class);
        // The reference runtime determines whether lookup or first access initialized Other.
        System.out.println("initializations after lookup: " + initializations);
        if ((int) external.getAndAdd(1) != 41 || Other.value != 42 || initializations != 1) {
            throw new AssertionError("initial value and first update");
        }
        VarHandle text = MethodHandles.lookup().findStaticVarHandle(Text.class, "value", String.class);
        try {
            text.set((Object) Integer.valueOf(1));
            throw new AssertionError("invalid value accepted");
        } catch (ClassCastException expected) {
            if (initializations != 2) throw new AssertionError("value cast before initialization");
        }
        try {
            readBroken();
            throw new AssertionError("missing initialization failure");
        } catch (ExceptionInInitializerError expected) {
            if (!(expected.getCause() instanceof IllegalStateException)) throw expected;
        }
        try {
            readBroken();
            throw new AssertionError("erroneous class was initialized twice");
        } catch (NoClassDefFoundError expected) {
            System.out.println("erroneous class rejected");
        }
    }

    static String initialized = "";
    static int mark(String owner) {
        initialized += owner;
        return owner.equals("I") ? 2 : 1;
    }
    public static class Parent { public static int inherited = mark("P"); }
    public static class Child extends Parent { static int child = mark("C"); }
    public interface Contract { int TOKEN = mark("I"); }
    public static class Impl implements Contract { static int impl = mark("X"); }

    static void inheritedStatics() throws ReflectiveOperationException {
        VarHandle inherited = MethodHandles.lookup().findStaticVarHandle(Child.class, "inherited", int.class);
        if ((int) inherited.getAndAdd(2) != 1 || Parent.inherited != 3 || !initialized.equals(Runtime.version().feature() < 22 ? "PC" : "P")) {
            throw new AssertionError("initialization or inherited field storage: " + initialized);
        }
        VarHandle constant = MethodHandles.lookup().findStaticVarHandle(Impl.class, "TOKEN", int.class);
        int first = (int) constant.get();
        // Earlier JDKs initialize the lookup class. Java 25 initializes the field declarer.
        System.out.println("interface handle first read: " + first + " initialized " + initialized);
        if (constant.isAccessModeSupported(VarHandle.AccessMode.SET) || Contract.TOKEN != 2 || (int) constant.get() != 2) {
            throw new AssertionError("interface constant storage");
        }
    }

    static int readBroken() throws ReflectiveOperationException {
        VarHandle handle = MethodHandles.lookup().findStaticVarHandle(Broken.class, "value", int.class);
        return (int) handle.get();
    }
}
