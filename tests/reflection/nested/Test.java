/** Test nested and inner class reflection operations. */
public class Test {
    // Static nested class
    static class StaticNestedClass {
        private String value = "static nested";

        public void method() {
            System.out.println("Static nested method");
        }
    }

    // Non-static inner class
    class InnerClass {
        private String value = "inner";

        public void method() {
            System.out.println("Inner method, outer value: " + Test.this.outerField);
        }

        public Test getOuter() {
            return Test.this;
        }
    }

    // Local class defined in method
    public Object createLocalClass() {
        final String localVar = "local variable";

        class LocalClass {
            private String value = "local";

            public void method() {
                System.out.println("Local method, local var: " + localVar);
            }
        }

        return new LocalClass();
    }

    // Anonymous class
    public Object createAnonymousClass() {
        return new Object() {
            private String value = "anonymous";

            public void method() {
                System.out.println("Anonymous method");
            }

            @Override
            public String toString() {
                return "Anonymous instance";
            }
        };
    }

    private String outerField = "outer field";

    public static void main(String[] args) throws Exception {
        Test outer = new Test();

        // Test static nested class
        System.out.println("=== Static Nested Class ===");
        Class<?> staticNestedClass = StaticNestedClass.class;
        System.out.println("Class name: " + staticNestedClass.getName());
        System.out.println("Simple name: " + staticNestedClass.getSimpleName());
        System.out.println("Canonical name: " + staticNestedClass.getCanonicalName());

        // Test enclosing class
        Class<?> enclosingClass = staticNestedClass.getEnclosingClass();
        System.out.println("Enclosing class: " + (enclosingClass != null ? enclosingClass.getName() : "null"));

        // Test modifiers
        System.out.println("Is static: " + java.lang.reflect.Modifier.isStatic(staticNestedClass.getModifiers()));
        System.out.println("Is member class: " + staticNestedClass.isMemberClass());
        System.out.println("Is local class: " + staticNestedClass.isLocalClass());
        System.out.println("Is anonymous class: " + staticNestedClass.isAnonymousClass());

        // Create instance of static nested class
        StaticNestedClass staticInstance = new StaticNestedClass();
        staticInstance.method();

        // Test non-static inner class
        System.out.println("\n=== Non-Static Inner Class ===");
        Class<?> innerClass = InnerClass.class;
        System.out.println("Class name: " + innerClass.getName());
        System.out.println("Simple name: " + innerClass.getSimpleName());
        System.out.println("Canonical name: " + innerClass.getCanonicalName());

        System.out.println("Enclosing class: " + innerClass.getEnclosingClass().getName());
        System.out.println("Is static: " + java.lang.reflect.Modifier.isStatic(innerClass.getModifiers()));
        System.out.println("Is member class: " + innerClass.isMemberClass());
        System.out.println("Is local class: " + innerClass.isLocalClass());
        System.out.println("Is anonymous class: " + innerClass.isAnonymousClass());

        // Create instance of inner class using reflection
        java.lang.reflect.Constructor<?> innerConstructor = innerClass.getDeclaredConstructor(Test.class);
        Object innerInstance = innerConstructor.newInstance(outer);

        java.lang.reflect.Method innerMethod = innerClass.getMethod("method");
        innerMethod.invoke(innerInstance);

        // Test getting outer instance
        java.lang.reflect.Method getOuterMethod = innerClass.getMethod("getOuter");
        Object outerFromInner = getOuterMethod.invoke(innerInstance);
        System.out.println("Got outer instance: " + (outerFromInner == outer));

        // Test local class
        System.out.println("\n=== Local Class ===");
        Object localInstance = outer.createLocalClass();
        Class<?> localClass = localInstance.getClass();

        System.out.println("Class name: " + localClass.getName());
        System.out.println("Simple name: " + localClass.getSimpleName());
        System.out.println("Canonical name: " + localClass.getCanonicalName());

        System.out.println("Enclosing class: " + localClass.getEnclosingClass().getName());
        System.out.println("Is member class: " + localClass.isMemberClass());
        System.out.println("Is local class: " + localClass.isLocalClass());
        System.out.println("Is anonymous class: " + localClass.isAnonymousClass());

        // Test enclosing method
        java.lang.reflect.Method enclosingMethod = localClass.getEnclosingMethod();
        System.out.println("Enclosing method: " + (enclosingMethod != null ? enclosingMethod.getName() : "null"));

        java.lang.reflect.Method localMethod = localClass.getMethod("method");
        localMethod.invoke(localInstance);

        // Test anonymous class
        System.out.println("\n=== Anonymous Class ===");
        Object anonymousInstance = outer.createAnonymousClass();
        Class<?> anonymousClass = anonymousInstance.getClass();

        System.out.println("Class name: " + anonymousClass.getName());
        System.out.println("Simple name: " + anonymousClass.getSimpleName());
        System.out.println("Canonical name: " + anonymousClass.getCanonicalName());

        System.out.println("Enclosing class: " + anonymousClass.getEnclosingClass().getName());
        System.out.println("Is member class: " + anonymousClass.isMemberClass());
        System.out.println("Is local class: " + anonymousClass.isLocalClass());
        System.out.println("Is anonymous class: " + anonymousClass.isAnonymousClass());

        // Test enclosing method for anonymous class
        java.lang.reflect.Method anonymousEnclosingMethod = anonymousClass.getEnclosingMethod();
        System.out.println("Enclosing method: " + (anonymousEnclosingMethod != null ? anonymousEnclosingMethod.getName() : "null"));

        java.lang.reflect.Method anonymousMethod = anonymousClass.getMethod("method");
        anonymousMethod.invoke(anonymousInstance);

        // Test declared classes
        System.out.println("\n=== Declared Classes ===");
        Class<?>[] declaredClasses = Test.class.getDeclaredClasses();
        System.out.println("Declared classes count: " + declaredClasses.length);

        for (Class<?> declaredClass : declaredClasses) {
            System.out.println("Declared class: " + declaredClass.getSimpleName());
        }

        // Test access to private members of nested classes
        System.out.println("\n=== Private Member Access ===");
        java.lang.reflect.Field staticNestedField = staticNestedClass.getDeclaredField("value");
        staticNestedField.setAccessible(true);
        String staticValue = (String) staticNestedField.get(staticInstance);
        System.out.println("Static nested private field: " + staticValue);

        java.lang.reflect.Field innerField = innerClass.getDeclaredField("value");
        innerField.setAccessible(true);
        String innerValue = (String) innerField.get(innerInstance);
        System.out.println("Inner private field: " + innerValue);

        // Test synthetic fields and methods
        System.out.println("\n=== Synthetic Members ===");
        java.lang.reflect.Field[] innerFields = innerClass.getDeclaredFields();
        for (java.lang.reflect.Field field : innerFields) {
            if (field.isSynthetic()) {
                System.out.println("Synthetic field: " + field.getName() + " (" + field.getType().getName() + ")");
            }
        }

        java.lang.reflect.Constructor<?>[] innerConstructors = innerClass.getDeclaredConstructors();
        for (java.lang.reflect.Constructor<?> constructor : innerConstructors) {
            System.out.println("Inner constructor parameters: " + constructor.getParameterCount());
            Class<?>[] paramTypes = constructor.getParameterTypes();
            for (int i = 0; i < paramTypes.length; i++) {
                System.out.println("  Parameter " + i + ": " + paramTypes[i].getName());
            }
        }

        // Test nested interface
        System.out.println("\n=== Nested Interface ===");
        interface NestedInterface {
            void interfaceMethod();
        }

        Class<?> nestedInterface = NestedInterface.class;
        System.out.println("Nested interface name: " + nestedInterface.getName());
        System.out.println("Is interface: " + nestedInterface.isInterface());
        System.out.println("Is member class: " + nestedInterface.isMemberClass());

        // Test class hierarchy with nested classes
        System.out.println("\n=== Class Hierarchy ===");
        System.out.println("Static nested superclass: " + staticNestedClass.getSuperclass().getName());
        System.out.println("Inner superclass: " + innerClass.getSuperclass().getName());
        System.out.println("Local superclass: " + localClass.getSuperclass().getName());
        System.out.println("Anonymous superclass: " + anonymousClass.getSuperclass().getName());
    }
}

