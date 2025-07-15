/** Test native method declarations and JNI behavior. */
public class Test {
    // Native method declarations
    public static native int nativeAdd(int a, int b);
    public static native String nativeGetSystemProperty(String key);
    public native void nativeInstanceMethod(int value);

    // Static block to load native library (commented out as we don't have actual native lib)
    /*
    static {
        System.loadLibrary("testlib");
    }
    */

    // Simulate native behavior with regular methods for testing
    public static int simulatedNativeAdd(int a, int b) {
        System.out.println("Simulated native add: " + a + " + " + b);
        return a + b;
    }

    public static String simulatedNativeGetSystemProperty(String key) {
        System.out.println("Simulated native system property lookup for: " + key);
        return System.getProperty(key, "default_value");
    }

    public void simulatedNativeInstanceMethod(int value) {
        System.out.println("Simulated native instance method called with: " + value);
    }

    // Method that would call native methods
    public static void testNativeMethods() {
        System.out.println("=== Testing Native Method Declarations ===");

        // These would call actual native methods if library was loaded
        try {
            // int result = nativeAdd(5, 3);
            // System.out.println("Native add result: " + result);

            // String prop = nativeGetSystemProperty("java.version");
            // System.out.println("Native system property: " + prop);

            System.out.println("Native methods declared but not implemented (no native library loaded)");
        } catch (UnsatisfiedLinkError e) {
            System.out.println("Native library not found: " + e.getMessage());
        }

        // Use simulated versions instead
        System.out.println("\n=== Using Simulated Native Methods ===");
        int result = simulatedNativeAdd(5, 3);
        System.out.println("Simulated native add result: " + result);

        String prop = simulatedNativeGetSystemProperty("java.version");
        System.out.println("Simulated native system property: " + prop);

        Test instance = new Test();
        instance.simulatedNativeInstanceMethod(42);
    }

    // Method demonstrating JNI parameter types
    public native boolean nativeBooleanMethod(boolean flag);
    public native byte nativeByteMethod(byte b);
    public native char nativeCharMethod(char c);
    public native short nativeShortMethod(short s);
    public native int nativeIntMethod(int i);
    public native long nativeLongMethod(long l);
    public native float nativeFloatMethod(float f);
    public native double nativeDoubleMethod(double d);
    public native String nativeStringMethod(String str);
    public native int[] nativeArrayMethod(int[] array);
    public native Object nativeObjectMethod(Object obj);

    // Simulated versions for demonstration
    public boolean simulatedNativeBooleanMethod(boolean flag) {
        System.out.println("Native boolean method: " + flag);
        return !flag;
    }

    public byte simulatedNativeByteMethod(byte b) {
        System.out.println("Native byte method: " + b);
        return (byte)(b + 1);
    }

    public char simulatedNativeCharMethod(char c) {
        System.out.println("Native char method: " + c);
        return (char)(c + 1);
    }

    public int[] simulatedNativeArrayMethod(int[] array) {
        System.out.println("Native array method with " + array.length + " elements");
        int[] result = new int[array.length];
        for (int i = 0; i < array.length; i++) {
            result[i] = array[i] * 2;
        }
        return result;
    }

    public static void testNativeParameterTypes() {
        System.out.println("\n=== Testing Native Method Parameter Types ===");
        Test test = new Test();

        boolean boolResult = test.simulatedNativeBooleanMethod(true);
        System.out.println("Boolean result: " + boolResult);

        byte byteResult = test.simulatedNativeByteMethod((byte)10);
        System.out.println("Byte result: " + byteResult);

        char charResult = test.simulatedNativeCharMethod('A');
        System.out.println("Char result: " + charResult);

        int[] arrayResult = test.simulatedNativeArrayMethod(new int[]{1, 2, 3, 4});
        System.out.print("Array result: ");
        for (int val : arrayResult) {
            System.out.print(val + " ");
        }
        System.out.println();
    }

    // Method demonstrating synchronized native methods
    public synchronized native void synchronizedNativeMethod();

    public synchronized void simulatedSynchronizedNativeMethod() {
        System.out.println("Synchronized native method called on thread: " + Thread.currentThread().getName());
        try {
            Thread.sleep(100); // Simulate native work
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
    }

    public static void testSynchronizedNative() {
        System.out.println("\n=== Testing Synchronized Native Methods ===");
        Test test = new Test();

        Thread[] threads = new Thread[3];
        for (int i = 0; i < 3; i++) {
            final int threadNum = i;
            threads[i] = new Thread(() -> {
                test.simulatedSynchronizedNativeMethod();
            }, "NativeThread-" + threadNum);
            threads[i].start();
        }

        for (Thread t : threads) {
            try {
                t.join();
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }
    }

    public static void main(String[] args) {
        testNativeMethods();
        testNativeParameterTypes();
        testSynchronizedNative();

        System.out.println("\n=== Native Method Summary ===");
        System.out.println("Native methods are declared with 'native' keyword");
        System.out.println("They must be implemented in native code (C/C++)");
        System.out.println("Native libraries are loaded with System.loadLibrary()");
        System.out.println("JNI provides the bridge between Java and native code");
        System.out.println("Native methods can be static or instance methods");
        System.out.println("Native methods can be synchronized");
    }
}

