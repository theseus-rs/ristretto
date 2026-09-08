public class Test {
    public static class Reflected {}

    public Reflected reflected() { return null; }

    public static void main(String[] args) throws Exception {
        ClassLoader system = ClassLoader.getSystemClassLoader();
        Class<?> reflected = Test.class.getMethod("reflected").getReturnType();
        for (Class<?> type : new Class<?>[] {Test.class, reflected}) {
            ClassLoader loader = type.getClassLoader();
            System.out.println("system loader: " + (loader == system));
            System.out.println(loader.loadClass("java.nio.file.LinkPermission").getName());
        }
    }
}
