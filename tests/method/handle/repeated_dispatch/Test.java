import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

/** Exercise repeated descriptor use without binding dispatch to one target or receiver. */
public class Test {
    public static class Counter {
        int value;
        public Counter(int value) { this.value = value; }
        public int add(int amount) { value += amount; return value; }
    }
    public static class DoubleCounter extends Counter {
        public DoubleCounter(int value) { super(value); }
        public int add(int amount) { return super.add(amount * 2); }
    }

    public static int increment(int value) { return value + 1; }
    public static int decrement(int value) { return value - 1; }
    public static void fail(int value) { throw new IllegalArgumentException("failure " + value); }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodType unary = MethodType.methodType(int.class, int.class);
        MethodHandle increment = lookup.findStatic(Test.class, "increment", unary);
        MethodHandle decrement = lookup.findStatic(Test.class, "decrement", unary);
        MethodHandle wide = increment.asType(MethodType.methodType(long.class, int.class));
        MethodHandle add = lookup.findVirtual(Counter.class, "add", unary);
        MethodHandle constructor = lookup.findConstructor(Counter.class,
            MethodType.methodType(void.class, int.class));
        MethodHandle failure = lookup.findStatic(Test.class, "fail",
            MethodType.methodType(void.class, int.class));
        Counter first = new Counter(0);
        Counter second = new DoubleCounter(10);
        MethodHandle boundFirst = add.bindTo(first);
        MethodHandle boundSecond = add.bindTo(second);

        for (int round = 0; round < 5; round++) {
            check((int) increment.invokeExact(round) == round + 1);
            check((int) decrement.invokeExact(round) == round - 1);
            check((long) wide.invokeExact(round) == round + 1L);
            check((int) increment.invoke(round) == round + 1);
            check((int) add.invokeExact(first, 1) == round * 2 + 1);
            check((int) add.invokeExact(second, 1) == 10 + round * 4 + 2);
            check((int) boundFirst.invokeExact(1) == round * 2 + 2);
            check((int) boundSecond.invokeExact(1) == 10 + round * 4 + 4);
            check(((Counter) constructor.invokeExact(round)).value == round);
            try {
                failure.invokeExact(round);
                throw new AssertionError("missing target exception");
            } catch (IllegalArgumentException expected) {
                check(expected.getMessage().equals("failure " + round));
            }
            try {
                int ignored = (int) add.invokeExact((Counter) null, 1);
                throw new AssertionError("missing null receiver exception");
            } catch (NullPointerException expected) {
                // A previous successful invocation must not supply its receiver here.
            }
        }
        System.out.println("Repeated method-handle dispatch checks passed");
    }

    static void check(boolean condition) {
        if (!condition) throw new AssertionError("incorrect method-handle result");
    }
}
