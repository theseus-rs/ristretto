import java.lang.invoke.*;
import java.lang.reflect.Method;
public class Test {
    public static class Parent { public String value() { return "parent"; } }
    public static class Child extends Parent { }
    public static class Override extends Child { public String value() { return "override"; } }
    public static void main(String[] args) throws Throwable {
        MethodHandle value = MethodHandles.lookup().findVirtual(Child.class, "value", MethodType.methodType(String.class));
        System.out.println((String) value.invokeExact(new Child()));
        System.out.println((String) value.invokeExact((Child) new Override()));
        MethodHandle access = MethodHandles.lookup().findVirtual(Method.class, "canAccess", MethodType.methodType(boolean.class, Object.class));
        System.out.println((boolean) access.invokeExact(Test.class.getMethod("main", String[].class), (Object) null));
    }
}
