import java.util.prefs.*;

/**
 * Test java.util.prefs.Preferences API which exercises FileSystemPreferences
 * native methods (chmod, lockFile0, unlockFile0) on Linux/Unix platforms.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        // Use user preferences to avoid needing root access for system prefs
        Preferences prefs = Preferences.userRoot().node("ristretto/test/preferences");

        try {
            // Test putting and getting values
            prefs.put("testKey", "testValue");
            String value = prefs.get("testKey", "default");
            System.out.println("put/get: " + value);

            // Test integer preference
            prefs.putInt("intKey", 42);
            int intValue = prefs.getInt("intKey", 0);
            System.out.println("putInt/getInt: " + intValue);

            // Test boolean preference
            prefs.putBoolean("boolKey", true);
            boolean boolValue = prefs.getBoolean("boolKey", false);
            System.out.println("putBoolean/getBoolean: " + boolValue);

            // Test default value for missing key
            String missing = prefs.get("missingKey", "defaultVal");
            System.out.println("missing key default: " + missing);

            // Test keys listing
            String[] keys = prefs.keys();
            System.out.println("keys count: " + keys.length);

            // Test remove
            prefs.remove("testKey");
            String afterRemove = prefs.get("testKey", "removed");
            System.out.println("after remove: " + afterRemove);

            // Flush to persist (this triggers lockFile0/chmod/unlockFile0 on Unix)
            prefs.flush();
            System.out.println("flush: success");
        } finally {
            // Clean up the test node
            prefs.removeNode();
            System.out.println("cleanup: success");
        }
    }
}
