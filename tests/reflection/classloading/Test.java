/** Test class loading and forName reflection operations. */
public class Test {
    static class NestedClass {}

    interface NestedInterface {}

    enum NestedEnum { VALUE }

    public static void main(String[] args) throws Exception {
        // Test basic Class.forName operations
        System.out.println("=== Basic Class.forName ===");

        // Load standard classes
        Class<?> stringClass = Class.forName("java.lang.String");
        System.out.println("Loaded String class: " + stringClass.getName());

        Class<?> objectClass = Class.forName("java.lang.Object");
        System.out.println("Loaded Object class: " + objectClass.getName());

        // Load primitive wrapper classes
        Class<?> integerClass = Class.forName("java.lang.Integer");
        System.out.println("Loaded Integer class: " + integerClass.getName());

        // Test array class loading
        System.out.println("\n=== Array Class Loading ===");
        Class<?> stringArrayClass = Class.forName("[Ljava.lang.String;");
        System.out.println("String array class: " + stringArrayClass.getName());
        System.out.println("Is array: " + stringArrayClass.isArray());
        System.out.println("Component type: " + stringArrayClass.getComponentType().getName());

        Class<?> intArrayClass = Class.forName("[I");
        System.out.println("Int array class: " + intArrayClass.getName());
        System.out.println("Component type: " + intArrayClass.getComponentType().getName());

        // Multi-dimensional arrays
        Class<?> stringMatrixClass = Class.forName("[[Ljava.lang.String;");
        System.out.println("String matrix class: " + stringMatrixClass.getName());

        // Test primitive class loading
        System.out.println("\n=== Primitive Class Loading ===");
        try {
            Class.forName("int");
            System.out.println("ERROR: Should not be able to load primitive via forName");
        } catch (ClassNotFoundException e) {
            System.out.println("Correctly failed to load primitive 'int': " + e.getMessage());
        }

        // Access primitive classes through .class
        Class<?> intClass = int.class;
        System.out.println("Int primitive class: " + intClass.getName());
        System.out.println("Is primitive: " + intClass.isPrimitive());

        // Test nested class loading
        System.out.println("\n=== Nested Class Loading ===");
        Class<?> nestedClass = Class.forName("Test$NestedClass");
        System.out.println("Nested class: " + nestedClass.getName());
        System.out.println("Simple name: " + nestedClass.getSimpleName());
        System.out.println("Enclosing class: " + nestedClass.getEnclosingClass().getName());

        Class<?> nestedInterface = Class.forName("Test$NestedInterface");
        System.out.println("Nested interface: " + nestedInterface.getName());
        System.out.println("Is interface: " + nestedInterface.isInterface());

        Class<?> nestedEnum = Class.forName("Test$NestedEnum");
        System.out.println("Nested enum: " + nestedEnum.getName());
        System.out.println("Is enum: " + nestedEnum.isEnum());

        // Test class loading with initialization
        System.out.println("\n=== Class Loading with Initialization ===");
        Class<?> loadedClass = Class.forName("Test", true, Test.class.getClassLoader());
        System.out.println("Loaded with initialization: " + loadedClass.getName());

        Class<?> notInitializedClass = Class.forName("Test", false, Test.class.getClassLoader());
        System.out.println("Loaded without initialization: " + notInitializedClass.getName());

        // Test ClassLoader operations
        System.out.println("\n=== ClassLoader Operations ===");
        ClassLoader systemLoader = ClassLoader.getSystemClassLoader();
        System.out.println("System ClassLoader: " + systemLoader.getClass().getName());

        ClassLoader currentLoader = Test.class.getClassLoader();
        // Print a simplified classloader description to handle implementation differences
        String loaderName = currentLoader != null ? currentLoader.getClass().getSimpleName() : "Bootstrap";
        // Normalize classloader names to avoid implementation-specific differences
        if (loaderName.contains("AppClassLoader") || loaderName.contains("BuiltinClassLoader")) {
            loaderName = "ApplicationClassLoader";
        }
        System.out.println("Current ClassLoader type: " + loaderName);

        // Test class loading from different loaders
        Class<?> viaSystemLoader = Class.forName("java.lang.String", false, systemLoader);
        System.out.println("String via system loader: " + viaSystemLoader.getName());
        System.out.println("Same as direct load: " + (viaSystemLoader == stringClass));

        // Test error cases
        System.out.println("\n=== Error Cases ===");

        // Non-existent class
        try {
            Class.forName("com.nonexistent.Class");
            System.out.println("ERROR: Should have thrown ClassNotFoundException");
        } catch (ClassNotFoundException e) {
            System.out.println("Correctly threw ClassNotFoundException: " + e.getMessage());
        }

        // Invalid array notation
        try {
            Class.forName("[Invalid");
            System.out.println("ERROR: Should have thrown ClassNotFoundException");
        } catch (ClassNotFoundException e) {
            System.out.println("Correctly threw ClassNotFoundException for invalid array: " + e.getMessage());
        }

        // Null class name
        try {
            Class.forName(null);
            System.out.println("ERROR: Should have thrown exception for null");
        } catch (Exception e) {
            System.out.println("Correctly threw exception for null: " + e.getClass().getSimpleName());
        }

        // Empty class name
        try {
            Class.forName("");
            System.out.println("ERROR: Should have thrown ClassNotFoundException");
        } catch (ClassNotFoundException e) {
            System.out.println("Correctly threw ClassNotFoundException for empty string");
        }

        // Test class identity
        System.out.println("\n=== Class Identity ===");
        Class<?> stringClass2 = Class.forName("java.lang.String");
        System.out.println("Same String class instances: " + (stringClass == stringClass2));

        Class<?> testClass1 = Test.class;
        Class<?> testClass2 = Class.forName("Test");
        System.out.println("Same Test class instances: " + (testClass1 == testClass2));

        // Test class metadata after loading
        System.out.println("\n=== Class Metadata After Loading ===");
        Class<?> loadedString = Class.forName("java.lang.String");
        System.out.println("String methods count: " + loadedString.getMethods().length);
        System.out.println("String fields count: " + loadedString.getFields().length);
        System.out.println("String constructors count: " + loadedString.getConstructors().length);

        // Test loading classes from java.lang package (automatically imported)
        System.out.println("\n=== Java.lang Package Classes ===");
        String[] javaLangClasses = {
            "java.lang.Object",
            "java.lang.Class",
            "java.lang.String",
            "java.lang.Integer",
            "java.lang.Thread",
            "java.lang.System"
        };

        for (String className : javaLangClasses) {
            try {
                Class<?> clazz = Class.forName(className);
                System.out.println("Loaded " + className + ": " + clazz.getSimpleName());
            } catch (ClassNotFoundException e) {
                System.out.println("Failed to load " + className + ": " + e.getMessage());
            }
        }

        // Test special array notations
        System.out.println("\n=== Special Array Notations ===");
        String[] arrayNotations = {
            "[Z",  // boolean array
            "[B",  // byte array
            "[C",  // char array
            "[D",  // double array
            "[F",  // float array
            "[I",  // int array
            "[J",  // long array
            "[S"   // short array
        };

        for (String notation : arrayNotations) {
            try {
                Class<?> arrayClass = Class.forName(notation);
                System.out.println(notation + " -> " + arrayClass.getComponentType().getName() + " array");
            } catch (ClassNotFoundException e) {
                System.out.println("Failed to load " + notation + ": " + e.getMessage());
            }
        }
    }
}

