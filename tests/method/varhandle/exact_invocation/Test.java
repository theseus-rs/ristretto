import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;
import java.lang.invoke.WrongMethodTypeException;

public class Test {
    int value = 7;
    static long total;
    String text;

    static class Box<T extends Number> {
        T number;
    }

    public static void main(String[] args) throws Throwable {
        Test receiver = new Test();
        VarHandle ordinary = MethodHandles.lookup().findVarHandle(Test.class, "value", int.class);
        VarHandle exact = ordinary.withInvokeExactBehavior();
        VarHandle flexible = exact.withInvokeBehavior();
        if (ordinary.withInvokeBehavior() != ordinary || exact.withInvokeExactBehavior() != exact) {
            throw new AssertionError("reapplying invocation behavior must return the same handle");
        }
        if (ordinary.hasInvokeExactBehavior() || !exact.hasInvokeExactBehavior() || flexible.hasInvokeExactBehavior()) {
            throw new AssertionError("invocation behavior");
        }
        if ((int) exact.get(receiver) != 7) throw new AssertionError("typed exact read");
        expectMismatch(() -> exact.get(receiver));  // The call site's return type is void.
        expectMismatch(() -> { Object v = exact.get(receiver); });
        expectMismatch(() -> { int v = (int) exact.get((Object) receiver); });
        expectMismatch(() -> exact.set(receiver, (short) 9));
        if (receiver.value != 7) throw new AssertionError("rejected write changed value");
        flexible.get(receiver);
        flexible.set((Object) receiver, (short) 11);
        if ((long) flexible.get(receiver) != 11L || (int) exact.get(receiver) != 11) {
            throw new AssertionError("adapting and exact handles must share storage");
        }
        VarHandle view = MethodHandles.byteArrayViewVarHandle(short[].class, java.nio.ByteOrder.nativeOrder())
                .withInvokeExactBehavior();
        expectMismatch(() -> view.getAndAdd(new byte[4], 0, 2L));
        sharedCallSites(MethodHandles.lookup(), receiver);
        adaptingInvoker(ordinary, exact, receiver);
        genericField();
        System.out.println("Exact invocation passed");
    }

    static void adaptingInvoker(VarHandle ordinary, VarHandle exact, Test receiver) throws Throwable {
        MethodHandle read = MethodHandles.varHandleInvoker(VarHandle.AccessMode.GET,
                MethodType.methodType(long.class, Object.class));
        for (VarHandle handle : new VarHandle[]{ordinary, exact, ordinary}) {
            try {
                long value = (long) read.invokeExact(handle, (Object) receiver);
                if (handle == exact || value != receiver.value) throw new AssertionError("adapting invoker result");
            } catch (WrongMethodTypeException failure) {
                if (handle != exact) throw failure;
            }
        }
    }

    static void genericField() throws ReflectiveOperationException {
        Box<Integer> box = new Box<>();
        box.number = 19;
        VarHandle exact = MethodHandles.lookup().findVarHandle(Box.class, "number", Number.class).withInvokeExactBehavior();
        if ((Number) exact.get(box) != box.number) throw new AssertionError("erased return type");
        expectMismatch(() -> { Integer value = (Integer) exact.get(box); });
        expectMismatch(() -> exact.set(box, Integer.valueOf(23)));
        if (box.number != 19) throw new AssertionError("rejected generic write");
        exact.set(box, (Number) Integer.valueOf(29));
        if (box.number != 29) throw new AssertionError("erased parameter type");
    }

    static void sharedCallSites(MethodHandles.Lookup lookup, Test receiver) throws Exception {
        VarHandle wide = lookup.findStaticVarHandle(Test.class, "total", long.class);
        total = 1L << 40;
        expectMismatch(() -> setWide(wide.withInvokeExactBehavior(), 3));
        if (setWide(wide, 3) != (1L << 40) || total != 3) throw new AssertionError("static widening");
        expectMismatch(() -> setWide(wide.withInvokeExactBehavior(), 4));
        if (total != 3) throw new AssertionError("rejected static update");

        VarHandle text = lookup.findVarHandle(Test.class, "text", String.class);
        expectMismatch(() -> setText(text.withInvokeExactBehavior(), receiver, "rejected"));
        if (setText(text, receiver, "accepted") != null || !"accepted".equals(receiver.text)) {
            throw new AssertionError("reference value adaptation");
        }

        VarHandle element = MethodHandles.arrayElementVarHandle(long[].class);
        long[] array = {1L << 44};
        for (VarHandle handle : new VarHandle[]{element, element.withInvokeExactBehavior(), element}) {
            long before = array[0];
            long witness = (long) handle.compareAndExchange(array, 0, before, before + 1);
            if (witness != before || array[0] != before + 1) throw new AssertionError("typed array exchange");
        }
    }

    static long setWide(VarHandle handle, int value) { return (long) handle.getAndSet(value); }
    static String setText(VarHandle handle, Test receiver, Object value) {
        return (String) handle.getAndSet(receiver, value);
    }

    static void expectMismatch(Runnable action) {
        try {
            action.run();
        } catch (WrongMethodTypeException expected) {
            return;
        }
        throw new AssertionError("expected WrongMethodTypeException");
    }
}
