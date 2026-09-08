import java.lang.invoke.MethodHandles;
public class Test {
    public static void main(String[] args) throws Throwable {
        System.out.println((int) MethodHandles.constant(int.class, 42).invokeExact());
        System.out.println((long) MethodHandles.constant(long.class, 1234567890123L).invokeExact());
        System.out.println((float) MethodHandles.constant(float.class, 1.5f).invokeExact());
        System.out.println((double) MethodHandles.constant(double.class, 2.25).invokeExact());
        System.out.println((boolean) MethodHandles.constant(boolean.class, true).invokeExact());
        System.out.println((String) MethodHandles.constant(String.class, "coffee").invokeExact());
    }
}
