import java.lang.invoke.*;

/** Directly test the StringConcatFactory */
public class Test {
    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodType methodType = MethodType.methodType(
            String.class,
            String.class,
            String.class,
            String.class,
            String.class
        );
        CallSite callSite = StringConcatFactory.makeConcat(lookup, "concat", methodType);
        MethodHandle handle = callSite.dynamicInvoker();
        String result = (String) handle.invokeExact("Hello", " ", "World", "!");
        System.out.println(result);
    }
}
