/** Test basic reflection operations on a class. */
public class Test {
    static class A {}

    public static void main(String[] args) throws Exception {
        Class<?> clazz = Class.forName("Test$A");

        System.out.println("Class name: " + clazz.getName());
        System.out.println("Simple name: " + clazz.getSimpleName());
        System.out.println("Package: " + clazz.getPackage());
        System.out.println("Superclass: " + clazz.getSuperclass().getName());
    }
}
