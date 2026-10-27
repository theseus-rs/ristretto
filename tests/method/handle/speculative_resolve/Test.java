import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

public class Test {

    interface Greeter {
        default String greet() {
            return "hi";
        }
    }

    static class Impl implements Greeter {}

    interface Action {
        void run() throws Throwable;
    }

    static boolean throwsExpected(Action action, Class<? extends Throwable> expected) {
        try {
            action.run();
            return false;
        } catch (Throwable throwable) {
            return throwable.getClass() == expected;
        }
    }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodType voidType = MethodType.methodType(void.class);
        System.out.println(throwsExpected(() -> lookup.findVirtual(Impl.class, "absent", voidType), NoSuchMethodException.class));
        System.out.println(throwsExpected(() -> lookup.findStatic(Impl.class, "absent", voidType), NoSuchMethodException.class));
        System.out.println(throwsExpected(() -> lookup.findGetter(Impl.class, "absent", int.class), NoSuchFieldException.class));
        System.out.println(throwsExpected(() -> lookup.findSetter(Impl.class, "absent", int.class), NoSuchFieldException.class));

        MethodHandle greet = lookup.findVirtual(Impl.class, "greet", MethodType.methodType(String.class));
        System.out.println("hi".equals((String) greet.invokeExact(new Impl())));
    }
}
