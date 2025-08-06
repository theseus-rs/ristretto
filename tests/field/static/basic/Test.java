/** Test basic static field initialization in a class. */
public class Test {
    static class Statics {
        static int a = 1;
        static long b = 2;
        static float c = 3.14f;
        static double d = 4.2d;
        static String e = "foo";
    }

    public static void main(String[] args) {
        System.out.println("Statics.a = " + Statics.a);
        System.out.println("Statics.b = " + Statics.b);
        System.out.println("Statics.c = " + Statics.c);
        System.out.println("Statics.d = " + Statics.d);
        System.out.println("Statics.e = " + Statics.e);
    }
}
