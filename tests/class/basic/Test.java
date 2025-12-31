/**
 * Comprehensive Class Get Methods Test
 * Tests all the getter methods available on java.lang.Class
 */
public class Test {
    interface TestInterface {
        void interfaceMethod();
    }

    interface AnotherInterface {
        void anotherMethod();
    }

    static class ParentClass implements TestInterface {
        private int parentField;

        public void interfaceMethod() {}
        public void parentMethod() {}
    }

    static class ChildClass extends ParentClass implements AnotherInterface {
        private String childField;

        public void anotherMethod() {}
        public void childMethod() {}

        static class NestedStaticClass {}
        class InnerClass {}
    }

    static enum TestEnum {
        VALUE1, VALUE2, VALUE3
    }

    @interface TestAnnotation {
        String value() default "test";
    }

    @TestAnnotation("annotated")
    static class AnnotatedClass {}

    public static void main(String[] args) {
        System.out.println("=== Comprehensive Class Get Methods Test ===\n");

        testGetName();
        testGetSimpleName();
        testGetCanonicalName();
        testGetTypeName();
        testGetPackage();
        testGetModule();
        testGetSuperclass();
        testGetInterfaces();
        testGetComponentType();
        testGetDeclaringClass();
        testGetEnclosingClass();
        testGetModifiers();
        testGetClassLoader();
        testGetEnumConstants();
        testGetAnnotations();
        testGetDeclaredClasses();
        testGetFields();
        testGetMethods();
        testGetConstructors();

        System.out.println("\n=== All Class Get Methods Tests Complete ===");
    }

    private static void testGetName() {
        System.out.println("--- Test: getName() ---");
        try {
            // Simple class
            Class<?> cls = ChildClass.class;
            if (cls != null) {
                System.out.println("ChildClass.getName(): " + cls.getName());
            }

            // Array class
            Class<?> arrayClass = String[].class;
            if (arrayClass != null) {
                System.out.println("String[].getName(): " + arrayClass.getName());
            }

            // Primitive class
            Class<?> primitiveClass = int.class;
            if (primitiveClass != null) {
                System.out.println("int.getName(): " + primitiveClass.getName());
            }

            // Inner class
            Class<?> innerClass = ChildClass.InnerClass.class;
            if (innerClass != null) {
                System.out.println("InnerClass.getName(): " + innerClass.getName());
            }
        } catch (Exception e) {
            System.out.println("Error in getName(): " + e.getMessage());
            e.printStackTrace();
        }
        System.out.println();
    }

    private static void testGetSimpleName() {
        System.out.println("--- Test: getSimpleName() ---");
        try {
            Class<?> cls = ChildClass.class;
            System.out.println("ChildClass.getSimpleName(): " + cls.getSimpleName());

            Class<?> arrayClass = String[].class;
            System.out.println("String[].getSimpleName(): " + arrayClass.getSimpleName());

            Class<?> primitiveClass = int.class;
            System.out.println("int.getSimpleName(): " + primitiveClass.getSimpleName());

            Class<?> innerClass = ChildClass.InnerClass.class;
            System.out.println("InnerClass.getSimpleName(): " + innerClass.getSimpleName());
        } catch (Exception e) {
            System.out.println("Error in getSimpleName(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetCanonicalName() {
        System.out.println("--- Test: getCanonicalName() ---");
        try {
            Class<?> cls = ChildClass.class;
            System.out.println("ChildClass.getCanonicalName(): " + cls.getCanonicalName());

            Class<?> arrayClass = String[].class;
            System.out.println("String[].getCanonicalName(): " + arrayClass.getCanonicalName());

            Class<?> innerClass = ChildClass.InnerClass.class;
            System.out.println("InnerClass.getCanonicalName(): " + innerClass.getCanonicalName());
        } catch (Exception e) {
            System.out.println("Error in getCanonicalName(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetTypeName() {
        System.out.println("--- Test: getTypeName() ---");
        try {
            Class<?> cls = ChildClass.class;
            System.out.println("ChildClass.getTypeName(): " + cls.getTypeName());

            Class<?> arrayClass = String[].class;
            System.out.println("String[].getTypeName(): " + arrayClass.getTypeName());

            Class<?> primitiveClass = int.class;
            System.out.println("int.getTypeName(): " + primitiveClass.getTypeName());
        } catch (Exception e) {
            System.out.println("Error in getTypeName(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetPackage() {
        System.out.println("--- Test: getPackage() ---");
        try {
            Class<?> stringClass = String.class;
            Package pkg = stringClass.getPackage();
            System.out.println("String.getPackage(): " + (pkg != null ? pkg.getName() : "null"));

            Class<?> testClass = Test.class;
            Package testPkg = testClass.getPackage();
            System.out.println("Test.getPackage(): " + (testPkg != null ? testPkg.getName() : "null"));

            Class<?> primitiveClass = int.class;
            Package primitivePkg = primitiveClass.getPackage();
            System.out.println("int.getPackage(): " + (primitivePkg != null ? primitivePkg.getName() : "null"));
        } catch (Exception e) {
            System.out.println("Error in getPackage(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetModule() {
        System.out.println("--- Test: getModule() ---");
        try {
            // Test with String class (from java.base module)
            Class<?> stringClass = String.class;
            Module stringModule = stringClass.getModule();
            System.out.println("String.getModule(): " + stringModule);
            System.out.println("String module name: " + stringModule.getName());
            System.out.println("String module isNamed: " + stringModule.isNamed());

            // Test with custom class (likely unnamed module)
            Class<?> testClass = Test.class;
            Module testModule = testClass.getModule();
            System.out.println("Test module name: " + testModule.getName());
            System.out.println("Test module isNamed: " + testModule.isNamed());

            // Test with nested class
            Class<?> childClass = ChildClass.class;
            Module childModule = childClass.getModule();
            System.out.println("ChildClass module name: " + childModule.getName());

            // Test with primitive
            Class<?> primitiveClass = int.class;
            Module primitiveModule = primitiveClass.getModule();
            System.out.println("int.getModule(): " + primitiveModule);
            System.out.println("int module name: " + primitiveModule.getName());

            // Test with array
            Class<?> arrayClass = String[].class;
            Module arrayModule = arrayClass.getModule();
            System.out.println("String[].getModule(): " + arrayModule);
            System.out.println("String[] module name: " + arrayModule.getName());
        } catch (Exception e) {
            System.out.println("Error in getModule(): " + e.getMessage());
            e.printStackTrace();
        }
        System.out.println();
    }

    private static void testGetSuperclass() {
        System.out.println("--- Test: getSuperclass() ---");
        try {
            Class<?> childClass = ChildClass.class;
            Class<?> superclass = childClass.getSuperclass();
            System.out.println("ChildClass.getSuperclass(): " + (superclass != null ? superclass.getName() : "null"));

            Class<?> parentClass = ParentClass.class;
            Class<?> parentSuper = parentClass.getSuperclass();
            System.out.println("ParentClass.getSuperclass(): " + (parentSuper != null ? parentSuper.getName() : "null"));

            Class<?> objectClass = Object.class;
            Class<?> objectSuper = objectClass.getSuperclass();
            System.out.println("Object.getSuperclass(): " + (objectSuper != null ? objectSuper.getName() : "null"));

            Class<?> interfaceClass = TestInterface.class;
            Class<?> interfaceSuper = interfaceClass.getSuperclass();
            System.out.println("TestInterface.getSuperclass(): " + (interfaceSuper != null ? interfaceSuper.getName() : "null"));

            Class<?> primitiveClass = int.class;
            Class<?> primitiveSuper = primitiveClass.getSuperclass();
            System.out.println("int.getSuperclass(): " + (primitiveSuper != null ? primitiveSuper.getName() : "null"));
        } catch (Exception e) {
            System.out.println("Error in getSuperclass(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetInterfaces() {
        System.out.println("--- Test: getInterfaces() ---");
        try {
            Class<?> childClass = ChildClass.class;
            Class<?>[] childInterfaces = childClass.getInterfaces();
            System.out.println("ChildClass.getInterfaces() count: " + childInterfaces.length);
            for (Class<?> iface : childInterfaces) {
                System.out.println("  - " + iface.getName());
            }

            Class<?> parentClass = ParentClass.class;
            Class<?>[] parentInterfaces = parentClass.getInterfaces();
            System.out.println("ParentClass.getInterfaces() count: " + parentInterfaces.length);
            for (Class<?> iface : parentInterfaces) {
                System.out.println("  - " + iface.getName());
            }

            Class<?> stringClass = String.class;
            Class<?>[] stringInterfaces = stringClass.getInterfaces();
            System.out.println("String.getInterfaces() count: " + stringInterfaces.length);
            for (Class<?> iface : stringInterfaces) {
                System.out.println("  - " + iface.getSimpleName());
            }
        } catch (Exception e) {
            System.out.println("Error in getInterfaces(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetComponentType() {
        System.out.println("--- Test: getComponentType() ---");
        try {
            Class<?> arrayClass = String[].class;
            Class<?> componentType = arrayClass.getComponentType();
            System.out.println("String[].getComponentType(): " + (componentType != null ? componentType.getName() : "null"));

            Class<?> intArrayClass = int[].class;
            Class<?> intComponentType = intArrayClass.getComponentType();
            System.out.println("int[].getComponentType(): " + (intComponentType != null ? intComponentType.getName() : "null"));

            Class<?> multiArrayClass = String[][].class;
            Class<?> multiComponentType = multiArrayClass.getComponentType();
            System.out.println("String[][].getComponentType(): " + (multiComponentType != null ? multiComponentType.getName() : "null"));

            Class<?> nonArrayClass = String.class;
            Class<?> nonArrayComponent = nonArrayClass.getComponentType();
            System.out.println("String.getComponentType(): " + (nonArrayComponent != null ? nonArrayComponent.getName() : "null"));
        } catch (Exception e) {
            System.out.println("Error in getComponentType(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetDeclaringClass() {
        System.out.println("--- Test: getDeclaringClass() ---");
        try {
            Class<?> nestedClass = ChildClass.NestedStaticClass.class;
            Class<?> declaringClass = nestedClass.getDeclaringClass();
            System.out.println("NestedStaticClass.getDeclaringClass(): " + (declaringClass != null ? declaringClass.getName() : "null"));

            Class<?> innerClass = ChildClass.InnerClass.class;
            Class<?> innerDeclaringClass = innerClass.getDeclaringClass();
            System.out.println("InnerClass.getDeclaringClass(): " + (innerDeclaringClass != null ? innerDeclaringClass.getName() : "null"));

            Class<?> topLevelClass = Test.class;
            Class<?> topLevelDeclaringClass = topLevelClass.getDeclaringClass();
            System.out.println("Test.getDeclaringClass(): " + (topLevelDeclaringClass != null ? topLevelDeclaringClass.getName() : "null"));
        } catch (Exception e) {
            System.out.println("Error in getDeclaringClass(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetEnclosingClass() {
        System.out.println("--- Test: getEnclosingClass() ---");
        try {
            Class<?> nestedClass = ChildClass.NestedStaticClass.class;
            Class<?> enclosingClass = nestedClass.getEnclosingClass();
            System.out.println("NestedStaticClass.getEnclosingClass(): " + (enclosingClass != null ? enclosingClass.getName() : "null"));

            Class<?> innerClass = ChildClass.InnerClass.class;
            Class<?> innerEnclosingClass = innerClass.getEnclosingClass();
            System.out.println("InnerClass.getEnclosingClass(): " + (innerEnclosingClass != null ? innerEnclosingClass.getName() : "null"));

            Class<?> topLevelClass = Test.class;
            Class<?> topLevelEnclosingClass = topLevelClass.getEnclosingClass();
            System.out.println("Test.getEnclosingClass(): " + (topLevelEnclosingClass != null ? topLevelEnclosingClass.getName() : "null"));
        } catch (Exception e) {
            System.out.println("Error in getEnclosingClass(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetModifiers() {
        System.out.println("--- Test: getModifiers() ---");
        try {
            Class<?> childClass = ChildClass.class;
            int modifiers = childClass.getModifiers();
            System.out.println("ChildClass.getModifiers(): " + modifiers);
            System.out.println("ChildClass modifiers string: " + java.lang.reflect.Modifier.toString(modifiers));

            Class<?> interfaceClass = TestInterface.class;
            int interfaceModifiers = interfaceClass.getModifiers();
            System.out.println("TestInterface.getModifiers(): " + interfaceModifiers);
            System.out.println("TestInterface modifiers string: " + java.lang.reflect.Modifier.toString(interfaceModifiers));
        } catch (Exception e) {
            System.out.println("Error in getModifiers(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetClassLoader() {
        System.out.println("--- Test: getClassLoader() ---");
        try {
            Class<?> stringClass = String.class;
            ClassLoader stringClassLoader = stringClass.getClassLoader();
            System.out.println("String.getClassLoader(): " + stringClassLoader);

            Class<?> primitiveClass = int.class;
            ClassLoader primitiveClassLoader = primitiveClass.getClassLoader();
            System.out.println("int.getClassLoader(): " + primitiveClassLoader);
        } catch (Exception e) {
            System.out.println("Error in getClassLoader(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetEnumConstants() {
        System.out.println("--- Test: getEnumConstants() ---");
        try {
            Class<?> enumClass = TestEnum.class;
            Object[] enumConstants = enumClass.getEnumConstants();
            System.out.println("TestEnum.getEnumConstants() count: " + (enumConstants != null ? enumConstants.length : 0));
            if (enumConstants != null) {
                for (Object constant : enumConstants) {
                    System.out.println("  - " + constant);
                }
            }

            Class<?> nonEnumClass = Test.class;
            Object[] nonEnumConstants = nonEnumClass.getEnumConstants();
            System.out.println("Test.getEnumConstants(): " + (nonEnumConstants != null ? nonEnumConstants.length : "null"));
        } catch (Exception e) {
            System.out.println("Error in getEnumConstants(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetAnnotations() {
        System.out.println("--- Test: getAnnotations() ---");
        try {
            Class<?> annotatedClass = AnnotatedClass.class;
            java.lang.annotation.Annotation[] annotations = annotatedClass.getAnnotations();
            System.out.println("AnnotatedClass.getAnnotations() count: " + annotations.length);
            for (java.lang.annotation.Annotation annotation : annotations) {
                System.out.println("  - " + annotation.annotationType().getSimpleName());
            }

            Class<?> nonAnnotatedClass = Test.class;
            java.lang.annotation.Annotation[] nonAnnotations = nonAnnotatedClass.getAnnotations();
            System.out.println("Test.getAnnotations() count: " + nonAnnotations.length);
        } catch (Exception e) {
            System.out.println("Error in getAnnotations(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetDeclaredClasses() {
        System.out.println("--- Test: getDeclaredClasses() ---");
        try {
            Class<?> testClass = Test.class;
            Class<?>[] declaredClasses = testClass.getDeclaredClasses();
            System.out.println("Test.getDeclaredClasses() count: " + declaredClasses.length);
            for (Class<?> declaredClass : declaredClasses) {
                System.out.println("  - " + declaredClass.getSimpleName());
            }

            Class<?> childClass = ChildClass.class;
            Class<?>[] childDeclaredClasses = childClass.getDeclaredClasses();
            System.out.println("ChildClass.getDeclaredClasses() count: " + childDeclaredClasses.length);
            for (Class<?> declaredClass : childDeclaredClasses) {
                System.out.println("  - " + declaredClass.getSimpleName());
            }
        } catch (Exception e) {
            System.out.println("Error in getDeclaredClasses(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetFields() {
        System.out.println("--- Test: getFields() ---");
        try {
            Class<?> childClass = ChildClass.class;
            java.lang.reflect.Field[] fields = childClass.getFields();
            System.out.println("ChildClass.getFields() count: " + fields.length);
            for (java.lang.reflect.Field field : fields) {
                System.out.println("  - " + field.getName());
            }

            java.lang.reflect.Field[] declaredFields = childClass.getDeclaredFields();
            System.out.println("ChildClass.getDeclaredFields() count: " + declaredFields.length);
            for (java.lang.reflect.Field field : declaredFields) {
                System.out.println("  - " + field.getName());
            }
        } catch (Exception e) {
            System.out.println("Error in getFields(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetMethods() {
        System.out.println("--- Test: getMethods() ---");
        try {
            Class<?> childClass = ChildClass.class;
            java.lang.reflect.Method[] methods = childClass.getMethods();
            System.out.println("ChildClass.getMethods() count: " + methods.length);

            java.lang.reflect.Method[] declaredMethods = childClass.getDeclaredMethods();
            System.out.println("ChildClass.getDeclaredMethods() count: " + declaredMethods.length);
            for (java.lang.reflect.Method method : declaredMethods) {
                System.out.println("  - " + method.getName());
            }
        } catch (Exception e) {
            System.out.println("Error in getMethods(): " + e.getMessage());
        }
        System.out.println();
    }

    private static void testGetConstructors() {
        System.out.println("--- Test: getConstructors() ---");
        try {
            Class<?> childClass = ChildClass.class;
            java.lang.reflect.Constructor<?>[] constructors = childClass.getConstructors();
            System.out.println("ChildClass.getConstructors() count: " + constructors.length);

            java.lang.reflect.Constructor<?>[] declaredConstructors = childClass.getDeclaredConstructors();
            System.out.println("ChildClass.getDeclaredConstructors() count: " + declaredConstructors.length);
        } catch (Exception e) {
            System.out.println("Error in getConstructors(): " + e.getMessage());
        }
        System.out.println();
    }
}
