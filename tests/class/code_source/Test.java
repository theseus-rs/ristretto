import java.security.CodeSource;
import java.security.ProtectionDomain;
import java.net.URL;

public class Test {
    public static void main(String[] args) throws Exception {
        testCodeSource();
    }

    static void testCodeSource() throws Exception {
        ProtectionDomain pd = Test.class.getProtectionDomain();
        CodeSource cs = pd.getCodeSource();
        if (cs == null) {
            throw new IllegalStateException("Unable to determine code source archive");
        }
        URL location = cs.getLocation();
        if (location == null) {
            throw new IllegalStateException("Unable to determine code source archive");
        }
        System.out.println(location.getProtocol());
    }
}
