import java.lang.invoke.*;

public class Test {
    static int initializations;
    public static class Fresh { public static int value = initialize(); }
    static int initialize() { initializations++; return 41; }

    public static void main(String[] args) throws Throwable {
        VarHandle field = MethodHandles.lookup().findStaticVarHandle(Fresh.class, "value", int.class);
        VarHandle exact = field.withInvokeExactBehavior();
        if (!field.describeConstable().isPresent() || !exact.hasInvokeExactBehavior()
                || exact.withInvokeBehavior().hasInvokeExactBehavior() || field.varType() != int.class
                || !field.coordinateTypes().isEmpty() || !field.isAccessModeSupported(VarHandle.AccessMode.GET)
                || !field.accessModeType(VarHandle.AccessMode.GET).equals(MethodType.methodType(int.class))) {
            throw new AssertionError("metadata");
        }
        MethodHandle read = exact.toMethodHandle(VarHandle.AccessMode.GET);
        if (initializations != 0) throw new AssertionError("metadata initialized the class");
        if ((int) read.invokeExact() != 41 || (int) field.get() != 41 || initializations != 1) {
            throw new AssertionError("initialization on bound first access");
        }
        System.out.println("Lazy static metadata passed");
    }
}
