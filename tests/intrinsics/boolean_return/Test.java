/**
 * A boolean intrinsic must deliver a boolean value to its caller: {@code sun.net.spi.DefaultProxySelector} calls the native {@code init()Z} from its static initializer when {@code java.net.useSystemProxies} is set, so loading the class exercises a boolean intrinsic result flowing into a {@code putstatic}.
 */
public class Test {

    public static void main(String[] args) throws Exception {
        System.setProperty("java.net.useSystemProxies", "true");
        Class<?> selector = Class.forName("sun.net.spi.DefaultProxySelector");
        System.out.println("loaded: " + selector.getName());
    }
}
