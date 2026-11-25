import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;

public class Test {
    public static class Parent {
        public int value = 3;
    }

    public static class Child extends Parent {
        public int value = 5;
        public final int fixed;
        public float amount = 1.5f;
        public String text;

        Child(int fixed) { this.fixed = fixed; }
    }

    public static void main(String[] args) throws Exception {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        Child first = new Child(7);
        Child second = new Child(9);
        VarHandle parent = lookup.findVarHandle(Parent.class, "value", int.class);
        VarHandle child = lookup.findVarHandle(Child.class, "value", int.class);
        parent.set(first, 11);
        child.set(second, 13);
        check("hidden fields", ((Parent) first).value == 11 && first.value == 5
                && ((Parent) second).value == 3 && second.value == 13);
        first.value = 19;
        check("direct field write", (int) child.getVolatile(first) == 19);

        VarHandle text = lookup.findVarHandle(Child.class, "text", String.class);
        try {
            text.set((Child) null, (Object) 42);
            throw new AssertionError("accepted a null receiver");
        } catch (NullPointerException expected) {}
        String original = new String("same");
        String equal = new String("same");
        check("CAS from null", text.compareAndSet(first, null, original));
        check("distinct expected reference", !text.compareAndSet(first, equal, null));
        check("mismatch witness identity", text.compareAndExchange(first, equal, null) == original);
        check("matching witness identity", text.compareAndExchange(first, original, null) == original);
        check("reference cleared", first.text == null);

        VarHandle amount = lookup.findVarHandle(Child.class, "amount", float.class);
        check("fractional addition", (float) amount.getAndAdd(first, 0.25f) == 1.5f && first.amount == 1.75f);
        first.amount = -0.0f;
        check("opposite signed zeros", !amount.compareAndSet(first, 0.0f, 1.0f));
        float nan = Float.intBitsToFloat(0x7fc01234);
        float other = Float.intBitsToFloat(0x7fc05678);
        check("swap signed zero", amount.compareAndSet(first, -0.0f, nan));
        check("NaN mismatch witness", Float.floatToRawIntBits((float) amount.compareAndExchange(first, other, 1.0f)) == 0x7fc01234);
        check("matching NaN payload", amount.compareAndSet(first, nan, 1.0f) && first.amount == 1.0f);

        VarHandle fixed = lookup.findVarHandle(Child.class, "fixed", int.class);
        try {
            fixed.set(first, 11);
            throw new AssertionError("accepted write to final instance field");
        } catch (UnsupportedOperationException expected) {
            check("final field", first.fixed == 7 && (int) fixed.get(first) == 7);
        }
        System.out.println("Instance field access passed");
    }

    static void check(String label, boolean condition) {
        if (!condition) throw new AssertionError(label);
    }
}
