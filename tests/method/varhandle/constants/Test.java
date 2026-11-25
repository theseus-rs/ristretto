import java.lang.invoke.ConstantBootstraps;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;

public class Test {
    private int value = 2;
    private static long total = 3;

    public static void main(String[] args) {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        try {
            MethodHandles.publicLookup().findVarHandle(Test.class, "value", int.class);
            throw new AssertionError("private field accessible to public lookup");
        } catch (IllegalAccessException expected) {
        } catch (NoSuchFieldException failure) {
            throw new AssertionError(failure);
        }
        try {
            ConstantBootstraps.fieldVarHandle(MethodHandles.publicLookup(), "value", VarHandle.class, Test.class, int.class);
            throw new AssertionError("private field accessible to bootstrap");
        } catch (IllegalAccessError expected) {}
        Test receiver = new Test();
        VarHandle field = ConstantBootstraps.fieldVarHandle(lookup, "value", VarHandle.class, Test.class, int.class);
        VarHandle staticField = ConstantBootstraps.staticFieldVarHandle(lookup, "total", VarHandle.class, Test.class, long.class);
        VarHandle element = ConstantBootstraps.arrayVarHandle(lookup, "_", VarHandle.class, String[].class);
        String[] names = {"before"};
        if ((int) field.getAndAdd(receiver, 4) != 2 || receiver.value != 6) throw new AssertionError("field bootstrap");
        if (!staticField.compareAndSet(3L, 9L) || total != 9L) throw new AssertionError("static bootstrap");
        if (!"before".equals((String) element.getAndSet(names, 0, "after")) || !"after".equals(names[0])) {
            throw new AssertionError("array bootstrap");
        }
        System.out.println("VarHandle bootstraps passed");
    }
}
