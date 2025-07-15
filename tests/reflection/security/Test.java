/** Test security and access control in reflection operations. */
public class Test {
    public static class PublicClass {
        public String publicField = "public";
        protected String protectedField = "protected";
        String packageField = "package";
        private String privateField = "private";

        public void publicMethod() {}
        protected void protectedMethod() {}
        void packageMethod() {}
        private void privateMethod() {}
    }

    private static class PrivateClass {
        private String value = "private class";
        private void privateMethod() {}
    }

    public static final class FinalClass {
        public String value = "final class";
    }

    public static void main(String[] args) throws Exception {
        // Test access to public class
        System.out.println("=== Public Class Access ===");
        Class<?> publicClass = PublicClass.class;
        PublicClass publicInstance = new PublicClass();

        // Public field access
        java.lang.reflect.Field publicField = publicClass.getField("publicField");
        System.out.println("Public field value: " + publicField.get(publicInstance));
        publicField.set(publicInstance, "modified public");
        System.out.println("Modified public field: " + publicField.get(publicInstance));

        // Public method access
        java.lang.reflect.Method publicMethod = publicClass.getMethod("publicMethod");
        publicMethod.invoke(publicInstance);
        System.out.println("Public method invoked successfully");

        // Test access to protected members
        System.out.println("\n=== Protected Member Access ===");
        try {
            java.lang.reflect.Field protectedField = publicClass.getField("protectedField");
            System.out.println("ERROR: Should not find protected field via getField()");
        } catch (NoSuchFieldException e) {
            System.out.println("Correctly failed to find protected field via getField()");
        }

        java.lang.reflect.Field protectedField = publicClass.getDeclaredField("protectedField");
        try {
            protectedField.get(publicInstance);
            System.out.println("ERROR: Should have thrown IllegalAccessException");
        } catch (IllegalAccessException e) {
            System.out.println("Correctly caught IllegalAccessException for protected field");
        }

        protectedField.setAccessible(true);
        System.out.println("Protected field after setAccessible: " + protectedField.get(publicInstance));

        // Test access to package-private members
        System.out.println("\n=== Package-Private Member Access ===");
        java.lang.reflect.Field packageField = publicClass.getDeclaredField("packageField");
        try {
            packageField.get(publicInstance);
            System.out.println("ERROR: Should have thrown IllegalAccessException");
        } catch (IllegalAccessException e) {
            System.out.println("Correctly caught IllegalAccessException for package field");
        }

        packageField.setAccessible(true);
        System.out.println("Package field after setAccessible: " + packageField.get(publicInstance));

        // Test access to private members
        System.out.println("\n=== Private Member Access ===");
        java.lang.reflect.Field privateField = publicClass.getDeclaredField("privateField");
        try {
            privateField.get(publicInstance);
            System.out.println("ERROR: Should have thrown IllegalAccessException");
        } catch (IllegalAccessException e) {
            System.out.println("Correctly caught IllegalAccessException for private field");
        }

        privateField.setAccessible(true);
        System.out.println("Private field after setAccessible: " + privateField.get(publicInstance));

        // Test method access patterns
        System.out.println("\n=== Method Access Patterns ===");
        java.lang.reflect.Method protectedMethod = publicClass.getDeclaredMethod("protectedMethod");
        try {
            protectedMethod.invoke(publicInstance);
            System.out.println("ERROR: Should have thrown IllegalAccessException");
        } catch (IllegalAccessException e) {
            System.out.println("Correctly caught IllegalAccessException for protected method");
        }

        protectedMethod.setAccessible(true);
        protectedMethod.invoke(publicInstance);
        System.out.println("Protected method invoked after setAccessible");

        // Test access to private class
        System.out.println("\n=== Private Class Access ===");
        Class<?> privateClass = PrivateClass.class;
        System.out.println("Private class accessible via reflection: " + (privateClass != null));

        try {
            Object privateInstance = privateClass.newInstance();
            System.out.println("ERROR: Should have thrown IllegalAccessException");
        } catch (IllegalAccessException e) {
            System.out.println("Correctly caught IllegalAccessException for private class instantiation");
        } catch (InstantiationException e) {
            System.out.println("Caught InstantiationException: " + e.getMessage());
        }

        // Try with constructor access
        java.lang.reflect.Constructor<?> privateConstructor = privateClass.getDeclaredConstructor();
        try {
            privateConstructor.newInstance();
            System.out.println("ERROR: Should have thrown IllegalAccessException");
        } catch (IllegalAccessException e) {
            System.out.println("Correctly caught IllegalAccessException for private constructor");
        }

        privateConstructor.setAccessible(true);
        Object privateInstance = privateConstructor.newInstance();
        System.out.println("Private class instantiated after setAccessible");

        // Test final class
        System.out.println("\n=== Final Class Access ===");
        Class<?> finalClass = FinalClass.class;
        System.out.println("Final class modifiers: " + java.lang.reflect.Modifier.toString(finalClass.getModifiers()));
        System.out.println("Is final: " + java.lang.reflect.Modifier.isFinal(finalClass.getModifiers()));

        FinalClass finalInstance = new FinalClass();
        java.lang.reflect.Field finalField = finalClass.getField("value");
        System.out.println("Final class field value: " + finalField.get(finalInstance));

        // Test accessibility queries
        System.out.println("\n=== Accessibility Queries ===");
        System.out.println("Public field accessible: " + publicField.isAccessible());
        System.out.println("Private field accessible: " + privateField.isAccessible());
        System.out.println("Protected method accessible: " + protectedMethod.isAccessible());

        // Test modifier checks
        System.out.println("\n=== Modifier Checks ===");
        System.out.println("Public field modifiers: " + java.lang.reflect.Modifier.toString(publicField.getModifiers()));
        System.out.println("Protected field modifiers: " + java.lang.reflect.Modifier.toString(protectedField.getModifiers()));
        System.out.println("Private field modifiers: " + java.lang.reflect.Modifier.toString(privateField.getModifiers()));

        System.out.println("Public field is public: " + java.lang.reflect.Modifier.isPublic(publicField.getModifiers()));
        System.out.println("Protected field is protected: " + java.lang.reflect.Modifier.isProtected(protectedField.getModifiers()));
        System.out.println("Private field is private: " + java.lang.reflect.Modifier.isPrivate(privateField.getModifiers()));

        // Test security manager interactions (if applicable)
        System.out.println("\n=== Security Manager ===");
        SecurityManager sm = System.getSecurityManager();
        System.out.println("Security manager present: " + (sm != null));

        // Test access from different contexts
        System.out.println("\n=== Cross-Package Access Simulation ===");
        // This simulates what would happen with cross-package access
        java.lang.reflect.Method packageMethod = publicClass.getDeclaredMethod("packageMethod");
        System.out.println("Package method modifiers: " + java.lang.reflect.Modifier.toString(packageMethod.getModifiers()));

        // Test reflection on system classes
        System.out.println("\n=== System Class Access ===");
        Class<?> stringClass = String.class;
        java.lang.reflect.Field[] stringFields = stringClass.getDeclaredFields();
        System.out.println("String class declared fields: " + stringFields.length);

        // Try to access a private field in String
        boolean foundPrivateField = false;
        for (java.lang.reflect.Field field : stringFields) {
            if (java.lang.reflect.Modifier.isPrivate(field.getModifiers())) {
                foundPrivateField = true;
                try {
                    field.setAccessible(true);
                    System.out.println("Successfully made String private field accessible: " + field.getName());
                    break;
                } catch (Exception e) {
                    System.out.println("Failed to make String private field accessible: " + e.getClass().getSimpleName());
                    break;
                }
            }
        }

        if (!foundPrivateField) {
            System.out.println("No private fields found in String class");
        }

        // Test reflection suppression
        System.out.println("\n=== Reflection Suppression ===");
        try {
            java.lang.reflect.Field suppressField = publicClass.getDeclaredField("nonExistentField");
            System.out.println("ERROR: Should have thrown NoSuchFieldException");
        } catch (NoSuchFieldException e) {
            System.out.println("Correctly threw NoSuchFieldException for non-existent field");
        }
    }
}

