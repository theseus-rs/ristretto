public class Test {
    static class A {
        int v;
        public A() { this.v = 5; }
        public A(int x) { this.v = x; }
    }

    public static void main(String[] args) throws Exception {
        Class<?> clazz = Class.forName("Test$A");
        java.lang.reflect.Constructor<?> ctor1 = clazz.getConstructor();
        java.lang.reflect.Constructor<?> ctor2 = clazz.getConstructor(int.class);
        Object o1 = ctor1.newInstance();
        Object o2 = ctor2.newInstance(123);

        System.out.println("A() v: " + ((A) o1).v);
        System.out.println("A(int) v: " + ((A) o2).v);
    }
}
