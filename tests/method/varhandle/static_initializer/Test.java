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

    public static void main(String[] args) {
        System.out.println("compareAndSet: " + Holder.SWAPPED);
        System.out.println("value: " + Holder.value);
    }
}
