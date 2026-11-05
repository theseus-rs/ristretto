import java.util.TimeZone;

/** An invalid timezone reaches the native getSystemGMTOffsetID null-return fallback. */
public class Test {
    public static void main(String[] args) {
        System.setProperty("user.timezone", "Invalid/Ristretto-Time-Zone");
        TimeZone.setDefault(null);
        TimeZone zone = TimeZone.getDefault();
        if (zone == null || zone.getID() == null) {
            throw new AssertionError("missing default timezone");
        }
        // The reference JVM may use the host offset, while Ristretto falls back to GMT.
        System.out.println("default timezone available: true");
    }
}
