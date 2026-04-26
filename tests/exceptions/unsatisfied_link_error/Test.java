public class Test {
    public static native int nativeStaticMethod(int a, int b);

    public native void nativeInstanceMethod();

    public static void main(String[] args) {
        System.out.println("Start");

        try {
            int result = nativeStaticMethod(1, 2);
            System.out.println("nativeStaticMethod returned: " + result);
        } catch (UnsatisfiedLinkError e) {
            System.out.println("nativeStaticMethod: " + e.getClass().getName());
        }

        try {
            Test t = new Test();
            t.nativeInstanceMethod();
            System.out.println("nativeInstanceMethod returned");
        } catch (UnsatisfiedLinkError e) {
            System.out.println("nativeInstanceMethod: " + e.getClass().getName());
        }

        System.out.println("End");
    }
}
