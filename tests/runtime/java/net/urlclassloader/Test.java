import java.io.InputStream;
import java.net.URL;
import java.net.URLClassLoader;
import java.util.Enumeration;

/**
 * Tests java.net.URLClassLoader functionality explicitly.
 *
 * Exercises URLClassLoader construction, class loading, resource lookup,
 * parent delegation, URL management, and close behavior.
 */
public class Test {
    public static void main(String[] args) throws Exception {
        testConstructors();
        testLoadClass();
        testResourceLookup();
        testClassNotFound();
        testParentDelegation();
        testGetURLs();
        testClose();
    }

    private static void testConstructors() throws Exception {
        System.out.println("=== URLClassLoader Constructors ===");

        // Construct with empty URL array and explicit parent
        URLClassLoader loader1 = new URLClassLoader(new URL[0], ClassLoader.getSystemClassLoader());
        System.out.println("Created URLClassLoader with parent: " + (loader1 != null));
        System.out.println("Parent is system class loader: " + (loader1.getParent() == ClassLoader.getSystemClassLoader()));
        loader1.close();

        // Construct with empty URL array (default parent)
        URLClassLoader loader2 = new URLClassLoader(new URL[0]);
        System.out.println("Created URLClassLoader with default parent: " + (loader2 != null));
        System.out.println("Default parent is not null: " + (loader2.getParent() != null));
        loader2.close();

        // Construct with a URL pointing to a directory
        URL codeSourceUrl = Test.class.getProtectionDomain().getCodeSource().getLocation();
        URLClassLoader loader3 = new URLClassLoader(new URL[]{ codeSourceUrl });
        System.out.println("Created URLClassLoader with code source URL: " + (loader3 != null));
        loader3.close();
    }

    private static void testLoadClass() throws Exception {
        System.out.println("\n=== URLClassLoader Load Class ===");

        // Create a URLClassLoader that can find classes on the classpath
        URL codeSourceUrl = Test.class.getProtectionDomain().getCodeSource().getLocation();
        URLClassLoader loader = new URLClassLoader(new URL[]{ codeSourceUrl }, ClassLoader.getSystemClassLoader());

        // Load a bootstrap class through the URLClassLoader (delegates to parent)
        Class<?> stringClass = loader.loadClass("java.lang.String");
        System.out.println("Loaded String class: " + stringClass.getName());
        System.out.println("String class identity preserved: " + (stringClass == String.class));

        // Load java.lang.Object through the URLClassLoader
        Class<?> objectClass = loader.loadClass("java.lang.Object");
        System.out.println("Loaded Object class: " + objectClass.getName());
        System.out.println("Object class identity preserved: " + (objectClass == Object.class));

        loader.close();
    }

    private static void testResourceLookup() throws Exception {
        System.out.println("\n=== URLClassLoader Resource Lookup ===");

        URL codeSourceUrl = Test.class.getProtectionDomain().getCodeSource().getLocation();
        URLClassLoader loader = new URLClassLoader(new URL[]{ codeSourceUrl }, ClassLoader.getSystemClassLoader());

        // Look up a resource that should exist (the Test class file itself)
        URL testClassUrl = loader.getResource("Test.class");
        System.out.println("Found Test.class resource: " + (testClassUrl != null));

        // Look up a resource that does not exist
        URL missingUrl = loader.getResource("NonExistentResource.xyz");
        System.out.println("Missing resource is null: " + (missingUrl == null));

        // getResourceAsStream for an existing resource
        InputStream stream = loader.getResourceAsStream("Test.class");
        System.out.println("getResourceAsStream not null: " + (stream != null));
        if (stream != null) {
            int bytesAvailable = stream.available();
            System.out.println("Resource stream has bytes: " + (bytesAvailable > 0));
            stream.close();
        }

        // getResources (returns Enumeration)
        Enumeration<URL> resources = loader.getResources("Test.class");
        System.out.println("getResources has elements: " + resources.hasMoreElements());

        loader.close();
    }

    private static void testClassNotFound() throws Exception {
        System.out.println("\n=== URLClassLoader Class Not Found ===");

        // Create a URLClassLoader with no URLs and no parent to ensure class loading fails
        URLClassLoader emptyLoader = new URLClassLoader(new URL[0], null);

        // Attempt to load a class that does not exist
        try {
            emptyLoader.loadClass("com.nonexistent.FakeClass");
            System.out.println("ERROR: Should have thrown ClassNotFoundException");
        } catch (ClassNotFoundException e) {
            System.out.println("Correctly threw ClassNotFoundException: " + e.getMessage());
        }

        // Attempt to load another non-existent class
        try {
            emptyLoader.loadClass("does.not.Exist");
            System.out.println("ERROR: Should have thrown ClassNotFoundException");
        } catch (ClassNotFoundException e) {
            System.out.println("Correctly threw ClassNotFoundException: " + e.getMessage());
        }

        // Even bootstrap classes should not be found with null parent and no URLs
        try {
            emptyLoader.loadClass("java.lang.String");
            // Note: bootstrap classes may still load via the bootstrap loader
            System.out.println("Bootstrap class loaded despite null parent (expected)");
        } catch (ClassNotFoundException e) {
            System.out.println("Bootstrap class not found with null parent");
        }

        emptyLoader.close();
    }

    private static void testParentDelegation() throws Exception {
        System.out.println("\n=== URLClassLoader Parent Delegation ===");

        // Create two URLClassLoaders with the same parent
        URL codeSourceUrl = Test.class.getProtectionDomain().getCodeSource().getLocation();
        ClassLoader parent = ClassLoader.getSystemClassLoader();
        URLClassLoader loader1 = new URLClassLoader(new URL[]{ codeSourceUrl }, parent);
        URLClassLoader loader2 = new URLClassLoader(new URL[]{ codeSourceUrl }, parent);

        // Loading the same bootstrap class from different URLClassLoaders should return the same Class
        Class<?> objectFromLoader1 = loader1.loadClass("java.lang.Object");
        Class<?> objectFromLoader2 = loader2.loadClass("java.lang.Object");
        System.out.println("Same Object class from different loaders: " + (objectFromLoader1 == objectFromLoader2));

        Class<?> integerFromLoader1 = loader1.loadClass("java.lang.Integer");
        Class<?> integerFromLoader2 = loader2.loadClass("java.lang.Integer");
        System.out.println("Same Integer class from different loaders: " + (integerFromLoader1 == integerFromLoader2));

        loader1.close();
        loader2.close();
    }

    private static void testGetURLs() throws Exception {
        System.out.println("\n=== URLClassLoader getURLs ===");

        // Empty URL array
        URLClassLoader emptyLoader = new URLClassLoader(new URL[0]);
        URL[] emptyUrls = emptyLoader.getURLs();
        System.out.println("Empty loader URL count: " + emptyUrls.length);
        emptyLoader.close();

        // Single URL
        URL url1 = new URL("file:///tmp/classes/");
        URLClassLoader singleLoader = new URLClassLoader(new URL[]{ url1 });
        URL[] singleUrls = singleLoader.getURLs();
        System.out.println("Single loader URL count: " + singleUrls.length);
        System.out.println("URL matches: " + url1.equals(singleUrls[0]));
        singleLoader.close();

        // Multiple URLs
        URL url2 = new URL("file:///tmp/lib.jar");
        URLClassLoader multiLoader = new URLClassLoader(new URL[]{ url1, url2 });
        URL[] multiUrls = multiLoader.getURLs();
        System.out.println("Multi loader URL count: " + multiUrls.length);
        multiLoader.close();
    }

    private static void testClose() throws Exception {
        System.out.println("\n=== URLClassLoader Close ===");

        URL codeSourceUrl = Test.class.getProtectionDomain().getCodeSource().getLocation();
        URLClassLoader loader = new URLClassLoader(new URL[]{ codeSourceUrl }, ClassLoader.getSystemClassLoader());

        // Load a class before closing
        Class<?> loaded = loader.loadClass("java.lang.String");
        System.out.println("Class loaded before close: " + loaded.getName());

        // Close the loader
        loader.close();
        System.out.println("URLClassLoader closed successfully");

        // Closing again should be a no-op (idempotent)
        loader.close();
        System.out.println("Second close succeeded (idempotent)");
    }
}
