/** Test lambda serialization and edge cases. */
import java.io.*;
import java.util.function.*;

public class Test {
    // Serializable functional interface
    @FunctionalInterface
    interface SerializableFunction<T, R> extends Function<T, R>, Serializable {
    }

    @FunctionalInterface
    interface SerializableRunnable extends Runnable, Serializable {
    }

    public static void main(String[] args) {
        System.out.println("=== Lambda Serialization Tests ===");

        // Serializable lambda
        System.out.println("--- Serializable Lambda ---");
        SerializableFunction<String, Integer> serLen = s -> s.length();
        System.out.println("Before serialization: " + serLen.apply("hello"));

        // Serialize and deserialize
        try {
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ObjectOutputStream oos = new ObjectOutputStream(baos);
            oos.writeObject(serLen);
            oos.close();

            ByteArrayInputStream bais = new ByteArrayInputStream(baos.toByteArray());
            ObjectInputStream ois = new ObjectInputStream(bais);
            @SuppressWarnings("unchecked")
            SerializableFunction<String, Integer> restored =
                    (SerializableFunction<String, Integer>) ois.readObject();
            ois.close();

            System.out.println("After serialization: " + restored.apply("hello"));
        } catch (Exception e) {
            System.out.println("Serialization exception: " + e.getClass().getSimpleName());
        }

        // Non-serializable lambda
        System.out.println("--- Non-Serializable Lambda ---");
        Function<String, Integer> nonSerLen = s -> s.length();
        try {
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ObjectOutputStream oos = new ObjectOutputStream(baos);
            oos.writeObject(nonSerLen);
            oos.close();
            System.out.println("Should not reach here");
        } catch (Exception e) {
            System.out.println("Expected exception: " + e.getClass().getSimpleName());
        }

        // Lambda capturing non-serializable variable
        System.out.println("--- Capturing Non-Serializable ---");
        Object nonSerializable = new Object();
        SerializableRunnable capturingNonSer = () -> System.out.println(nonSerializable);
        try {
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ObjectOutputStream oos = new ObjectOutputStream(baos);
            oos.writeObject(capturingNonSer);
            oos.close();
            System.out.println("Should not reach here");
        } catch (Exception e) {
            System.out.println("Expected exception: " + e.getClass().getSimpleName());
        }

        // Lambda capturing serializable variable
        System.out.println("--- Capturing Serializable ---");
        String capturedStr = "captured";
        SerializableFunction<String, String> capturingStr = s -> capturedStr + s;
        try {
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ObjectOutputStream oos = new ObjectOutputStream(baos);
            oos.writeObject(capturingStr);
            oos.close();

            ByteArrayInputStream bais = new ByteArrayInputStream(baos.toByteArray());
            ObjectInputStream ois = new ObjectInputStream(bais);
            @SuppressWarnings("unchecked")
            SerializableFunction<String, String> restored =
                    (SerializableFunction<String, String>) ois.readObject();
            ois.close();

            System.out.println("Restored with capture: " + restored.apply("!"));
        } catch (Exception e) {
            System.out.println("Exception: " + e.getMessage());
        }

        // Lambda identity and equality
        System.out.println("--- Lambda Identity ---");
        Runnable r1 = () -> {};
        Runnable r2 = () -> {};
        Runnable r3 = r1;

        System.out.println("r1 == r2: " + (r1 == r2));
        System.out.println("r1 == r3: " + (r1 == r3));
        System.out.println("r1.equals(r2): " + r1.equals(r2));
        System.out.println("r1.equals(r3): " + r1.equals(r3));

        // Lambda class information
        System.out.println("--- Lambda Class Info ---");
        Function<String, Integer> lambda = s -> s.length();
        Class<?> lambdaClass = lambda.getClass();
        System.out.println("Lambda class name contains '$': " + lambdaClass.getName().contains("$"));
        System.out.println("Is synthetic: " + lambdaClass.isSynthetic());
        System.out.println("Is anonymous: " + lambdaClass.isAnonymousClass());
        System.out.println("Interfaces: " + lambdaClass.getInterfaces().length);

        // Lambda in static context
        System.out.println("--- Static Context ---");
        Function<String, String> staticLambda = getStaticLambda();
        System.out.println("Static lambda result: " + staticLambda.apply("test"));

        // Lambda in instance context
        System.out.println("--- Instance Context ---");
        Test instance = new Test();
        Function<String, String> instanceLambda = instance.getInstanceLambda();
        System.out.println("Instance lambda result: " + instanceLambda.apply("test"));

        // Recursive lambda (via holder)
        System.out.println("--- Recursive Lambda ---");
        final Function<Integer, Integer>[] factorial = new Function[1];
        factorial[0] = n -> n <= 1 ? 1 : n * factorial[0].apply(n - 1);
        System.out.println("Factorial(5): " + factorial[0].apply(5));
        System.out.println("Factorial(0): " + factorial[0].apply(0));

        // Lambda with varargs
        System.out.println("--- Varargs Lambda ---");
        @FunctionalInterface
        interface VarargsFunction {
            String join(String... args);
        }
        VarargsFunction joiner = varArgs -> String.join(", ", varArgs);
        System.out.println("Joined: " + joiner.join("a", "b", "c"));

        System.out.println("=== End Lambda Serialization Tests ===");
    }

    private static Function<String, String> getStaticLambda() {
        return s -> "Static: " + s;
    }

    private Function<String, String> getInstanceLambda() {
        return s -> "Instance: " + s;
    }
}
