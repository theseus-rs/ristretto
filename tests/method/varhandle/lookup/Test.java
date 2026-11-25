import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.lang.invoke.MethodType;

public class Test {
    private int number = 7;
    private static int count = 11;

    static class Nestmate {
        private static void hidden() {}
    }

    interface Action {
        void run() throws ReflectiveOperationException;
    }

    public static void main(String[] args) throws ReflectiveOperationException {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        VarHandle field = lookup.unreflectVarHandle(Test.class.getDeclaredField("number"));
        VarHandle staticField = lookup.unreflectVarHandle(Test.class.getDeclaredField("count"));
        Test receiver = new Test();
        if ((int) field.get(receiver) != receiver.number || (int) staticField.get() != count) {
            throw new AssertionError("private fields through unreflectVarHandle");
        }
        expect(IllegalAccessException.class, () -> MethodHandles.publicLookup().findVarHandle(Test.class, "number", int.class));
        expect(IllegalAccessException.class, () -> MethodHandles.publicLookup().unreflectVarHandle(Test.class.getDeclaredField("number")));
        java.lang.reflect.Field accessible = Test.class.getDeclaredField("number");
        accessible.setAccessible(true);
        expect(IllegalAccessException.class, () -> MethodHandles.publicLookup().unreflectVarHandle(accessible));
        expect(NoSuchFieldException.class, () -> lookup.findVarHandle(Test.class, "absent", int.class));
        expect(NoSuchFieldException.class, () -> lookup.findVarHandle(Test.class, "number", long.class));
        expect(IllegalAccessException.class, () -> lookup.findVarHandle(Test.class, "count", int.class));
        expect(IllegalAccessException.class, () -> lookup.findStaticVarHandle(Test.class, "number", int.class));

        lookup.findStatic(Nestmate.class, "hidden", MethodType.methodType(void.class));
        try {
            lookup.findStatic(PrivateTarget.class, "hidden", MethodType.methodType(void.class));
            throw new AssertionError("private instance method resolved as static");
        } catch (IllegalAccessException failure) {
            if (!(failure.getCause() instanceof IllegalAccessError)) throw failure;
        }

        System.out.println("VarHandle lookup passed");
    }

    static void expect(Class<? extends ReflectiveOperationException> type, Action action) throws ReflectiveOperationException {
        try {
            action.run();
        } catch (ReflectiveOperationException failure) {
            if (failure.getClass() == type) return;
            throw failure;
        }
        throw new AssertionError("expected " + type.getName());
    }
}

class PrivateTarget {
    private void hidden() {}
}
