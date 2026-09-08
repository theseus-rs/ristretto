import java.lang.invoke.*;
public class Test {
    public static void main(String[] args) throws Throwable {
        for (CallSite site : new CallSite[]{
                new MutableCallSite(MethodHandles.constant(String.class, "before")),
                new VolatileCallSite(MethodHandles.constant(String.class, "before"))}) {
            MethodHandle invoker = site.dynamicInvoker();
            System.out.println((String) invoker.invokeExact());
            site.setTarget(MethodHandles.constant(String.class, "after"));
            System.out.println((String) invoker.invokeExact());
        }
    }
}
