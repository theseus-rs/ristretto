import java.lang.reflect.Array;
import java.lang.reflect.Proxy;

/** Native proxy definitions must remain discoverable by their defining loader. */
public class Test {
    public static void main(String[] args) throws Exception {
        Class<?> proxy = Proxy.getProxyClass(null, Runnable.class);
        System.out.println("bootstrap proxy: " + (proxy.getClassLoader() == null));
        System.out.println("bootstrap lookup: " + (Class.forName(proxy.getName(), false, null) == proxy));
        Class<?> array = Array.newInstance(proxy, 1).getClass();
        System.out.println("bootstrap array: " + (array.getClassLoader() == null));
        System.out.println("component identity: " + (array.getComponentType() == proxy));
        System.out.println("bootstrap array lookup: "
                + (Class.forName("[L" + proxy.getName() + ";", false, null) == array));
    }
}
