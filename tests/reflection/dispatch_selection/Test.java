import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Method;

/** Reflection and method handles must select the same implementation as Java calls. */
public class Test {
    public static class Base {
        private String secret() { return "base private"; }
        public String value() { return "base"; }
        public static String inheritedStatic() { return "static"; }
    }
    public static class Child extends Base {
        public String secret() { return "child public"; }
    }
    public interface Default {
        default String value() { return "default"; }
    }
    public static class ClassWins extends Base implements Default {}
    public interface Root {
        default String value() { return "root"; }
    }
    public interface Specific extends Root {
        default String value() { return "specific"; }
    }
    public static class Parent implements Root {}
    public static class MostSpecific extends Parent implements Specific {}
    public static class Leaf extends MostSpecific {}

    public static void main(String[] args) throws Throwable {
        Method privateMethod = Base.class.getDeclaredMethod("secret");
        privateMethod.setAccessible(true);
        System.out.println("private reflection: " + privateMethod.invoke(new Child()));
        System.out.println("class reflection: " + Default.class.getMethod("value").invoke(new ClassWins()));
        System.out.println("default reflection: " + Root.class.getMethod("value").invoke(new Leaf()));
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodType type = MethodType.methodType(String.class);
        MethodHandle classMethod = lookup.findVirtual(Default.class, "value", type);
        System.out.println("class handle: " + (String) classMethod.invoke(new ClassWins()));
        MethodHandle defaultMethod = lookup.findVirtual(Leaf.class, "value", type);
        System.out.println("default handle: " + (String) defaultMethod.invoke(new Leaf()));
        MethodHandle privateHandle = lookup.unreflect(privateMethod);
        System.out.println("private handle: " + (String) privateHandle.invoke(new Child()));
        MethodHandle staticMethod = lookup.findStatic(Child.class, "inheritedStatic", type);
        System.out.println("static handle: " + (String) staticMethod.invoke());
        privateMethod.setAccessible(false);
        try {
            privateMethod.invoke(new Child());
            throw new AssertionError("private access must still be checked by Method.invoke");
        } catch (IllegalAccessException expected) {
            System.out.println("private access denied");
        }
        MethodHandle toString = lookup.findVirtual(Object.class, "toString", type);
        System.out.println("primitive array handle: " + ((String) toString.invoke(new int[1])).startsWith("[I@"));
        System.out.println("reference array handle: " + ((String) toString.invoke((Object) new String[1])).startsWith("[Ljava.lang.String;@"));
    }
}
