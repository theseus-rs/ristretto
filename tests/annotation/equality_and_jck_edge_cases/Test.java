import java.lang.annotation.*;
import java.lang.reflect.*;

// Annotation for testing equality and hashCode contracts
@Retention(RetentionPolicy.RUNTIME)
@interface EqualityTestAnnotation {
    String stringValue() default "test";
    int intValue() default 42;
    String[] arrayValue() default {"a", "b", "c"};
    Class<?> classValue() default Object.class;
}

// Annotation with identical structure for equality testing
@Retention(RetentionPolicy.RUNTIME)
@interface IdenticalStructureAnnotation {
    String stringValue() default "test";
    int intValue() default 42;
    String[] arrayValue() default {"a", "b", "c"};
    Class<?> classValue() default Object.class;
}

// Annotation for testing proxy behavior
@Retention(RetentionPolicy.RUNTIME)
@interface ProxyTestAnnotation {
    String value() default "proxy";
    int number() default 123;
}

// Annotation for testing annotation type validation
@Retention(RetentionPolicy.RUNTIME)
@interface ValidationTestAnnotation {
    String requiredValue();
    String optionalValue() default "optional";
}

@EqualityTestAnnotation(stringValue = "class", intValue = 100, arrayValue = {"x", "y", "z"}, classValue = String.class)
@IdenticalStructureAnnotation(stringValue = "class", intValue = 100, arrayValue = {"x", "y", "z"}, classValue = String.class)
@ProxyTestAnnotation(value = "classProxy", number = 456)
@ValidationTestAnnotation(requiredValue = "required")
public class Test {

    @EqualityTestAnnotation(stringValue = "field", intValue = 200)
    @ProxyTestAnnotation("fieldProxy")
    private String testField;

    @EqualityTestAnnotation(stringValue = "method", intValue = 300, arrayValue = {"1", "2"})
    @ProxyTestAnnotation(value = "methodProxy", number = 789)
    public void testMethod() {
    }

    // Method with identical annotation values for equality testing
    @EqualityTestAnnotation(stringValue = "method", intValue = 300, arrayValue = {"1", "2"})
    public void identicalAnnotationMethod() {
    }

    public static void main(String[] args) {
        System.out.println("=== Annotation Equality, HashCode, and JCK Edge Cases Test ===");

        Class<?> clazz = Test.class;

        // Test annotation equality contracts
        testAnnotationEquality(clazz);

        // Test annotation hashCode contracts
        testAnnotationHashCode(clazz);

        // Test annotation proxy behavior
        testAnnotationProxy(clazz);

        // Test annotation type validation
        testAnnotationTypeValidation(clazz);

        // Test annotation method contracts
        testAnnotationMethodContracts(clazz);

        // Test annotation string representation
        testAnnotationStringRepresentation(clazz);

        // Test annotation class properties
        testAnnotationClassProperties();

        // Test edge cases and error conditions
        testEdgeCasesAndErrors(clazz);
    }

    private static void testAnnotationEquality(Class<?> clazz) {
        System.out.println("=== Annotation Equality Test ===");

        try {
            // Get same annotation multiple times
            EqualityTestAnnotation classAnn1 = clazz.getAnnotation(EqualityTestAnnotation.class);
            EqualityTestAnnotation classAnn2 = clazz.getAnnotation(EqualityTestAnnotation.class);

            System.out.println("Same annotation instance: " + (classAnn1 == classAnn2));
            System.out.println("Annotation equals itself: " + classAnn1.equals(classAnn1));
            System.out.println("Annotation equals retrieved again: " + classAnn1.equals(classAnn2));

            // Test reflexive property
            System.out.println("Reflexive: a.equals(a) = " + classAnn1.equals(classAnn1));

            // Test symmetric property
            System.out.println("Symmetric: a.equals(b) = " + classAnn1.equals(classAnn2));
            System.out.println("Symmetric: b.equals(a) = " + classAnn2.equals(classAnn1));
            System.out.println("Symmetric property holds: " + (classAnn1.equals(classAnn2) == classAnn2.equals(classAnn1)));

            // Test annotations with identical values on different methods
            Method method1 = clazz.getDeclaredMethod("testMethod");
            Method method2 = clazz.getDeclaredMethod("identicalAnnotationMethod");

            EqualityTestAnnotation methodAnn1 = method1.getAnnotation(EqualityTestAnnotation.class);
            EqualityTestAnnotation methodAnn2 = method2.getAnnotation(EqualityTestAnnotation.class);

            if (methodAnn1 != null && methodAnn2 != null) {
                System.out.println("Methods have identical annotation values: " + methodAnn1.equals(methodAnn2));
                System.out.println("Method1 values: string=" + methodAnn1.stringValue() + ", int=" + methodAnn1.intValue() +
                        ", array=" + java.util.Arrays.toString(methodAnn1.arrayValue()));
                System.out.println("Method2 values: string=" + methodAnn2.stringValue() + ", int=" + methodAnn2.intValue() +
                        ", array=" + java.util.Arrays.toString(methodAnn2.arrayValue()));
            }

            // Test annotations with different values
            Field field = clazz.getDeclaredField("testField");
            EqualityTestAnnotation fieldAnn = field.getAnnotation(EqualityTestAnnotation.class);

            if (fieldAnn != null) {
                System.out.println("Class annotation equals field annotation: " + classAnn1.equals(fieldAnn));
                System.out.println("Class int: " + classAnn1.intValue() + ", Field int: " + fieldAnn.intValue());
            }

            // Test null equality
            System.out.println("Annotation equals null: " + classAnn1.equals(null));

            // Test equality with different types
            System.out.println("Annotation equals string: " + classAnn1.equals("not an annotation"));

        } catch (Exception e) {
            System.out.println("Error testing annotation equality: " + e.getMessage());
        }
    }

    private static void testAnnotationHashCode(Class<?> clazz) {
        System.out.println("\n=== Annotation HashCode Test ===");

        try {
            EqualityTestAnnotation classAnn1 = clazz.getAnnotation(EqualityTestAnnotation.class);
            EqualityTestAnnotation classAnn2 = clazz.getAnnotation(EqualityTestAnnotation.class);

            // Test hashCode consistency
            int hash1 = classAnn1.hashCode();
            int hash2 = classAnn1.hashCode();
            System.out.println("HashCode consistent: " + (hash1 == hash2));
            System.out.println("HashCode value: " + hash1);

            // Test equal objects have equal hashCodes
            if (classAnn1.equals(classAnn2)) {
                System.out.println("Equal annotations have equal hashCodes: " + (classAnn1.hashCode() == classAnn2.hashCode()));
            }

            // Test hashCode for annotations with identical values
            Method method1 = clazz.getDeclaredMethod("testMethod");
            Method method2 = clazz.getDeclaredMethod("identicalAnnotationMethod");

            EqualityTestAnnotation methodAnn1 = method1.getAnnotation(EqualityTestAnnotation.class);
            EqualityTestAnnotation methodAnn2 = method2.getAnnotation(EqualityTestAnnotation.class);

            if (methodAnn1 != null && methodAnn2 != null && methodAnn1.equals(methodAnn2)) {
                System.out.println("Identical value annotations have equal hashCodes: " +
                        (methodAnn1.hashCode() == methodAnn2.hashCode()));
                System.out.println("Method1 hashCode: " + methodAnn1.hashCode());
                System.out.println("Method2 hashCode: " + methodAnn2.hashCode());
            }

            // Test different annotations have different hashCodes (not required but expected)
            Field field = clazz.getDeclaredField("testField");
            EqualityTestAnnotation fieldAnn = field.getAnnotation(EqualityTestAnnotation.class);

            if (fieldAnn != null && !classAnn1.equals(fieldAnn)) {
                System.out.println("Different annotations have different hashCodes: " +
                        (classAnn1.hashCode() != fieldAnn.hashCode()));
                System.out.println("Class hashCode: " + classAnn1.hashCode());
                System.out.println("Field hashCode: " + fieldAnn.hashCode());
            }

        } catch (Exception e) {
            System.out.println("Error testing annotation hashCode: " + e.getMessage());
        }
    }

    private static void testAnnotationProxy(Class<?> clazz) {
        System.out.println("\n=== Annotation Proxy Test ===");

        ProxyTestAnnotation proxyAnn = clazz.getAnnotation(ProxyTestAnnotation.class);
        if (proxyAnn != null) {
            System.out.println("Annotation class: " + proxyAnn.getClass().getName());
            System.out.println("Is proxy: " + Proxy.isProxyClass(proxyAnn.getClass()));

            if (Proxy.isProxyClass(proxyAnn.getClass())) {
                InvocationHandler handler = Proxy.getInvocationHandler(proxyAnn);
                System.out.println("Invocation handler: " + handler.getClass().getName());
            }

            // Test annotation methods through proxy
            System.out.println("Proxy value(): " + proxyAnn.value());
            System.out.println("Proxy number(): " + proxyAnn.number());

            // Test annotationType() method
            Class<? extends Annotation> annotationType = proxyAnn.annotationType();
            System.out.println("annotationType(): " + annotationType.getName());
            System.out.println("annotationType() is ProxyTestAnnotation: " + (annotationType == ProxyTestAnnotation.class));

            // Test proxy method invocation
            try {
                Method valueMethod = ProxyTestAnnotation.class.getMethod("value");
                Object result = valueMethod.invoke(proxyAnn);
                System.out.println("Direct method invocation result: " + result);
                System.out.println("Result equals proxy call: " + result.equals(proxyAnn.value()));
            } catch (Exception e) {
                System.out.println("Error invoking annotation method: " + e.getMessage());
            }
        }
    }

    private static void testAnnotationTypeValidation(Class<?> clazz) {
        System.out.println("\n=== Annotation Type Validation Test ===");

        ValidationTestAnnotation validationAnn = clazz.getAnnotation(ValidationTestAnnotation.class);
        if (validationAnn != null) {
            System.out.println("Required value: " + validationAnn.requiredValue());
            System.out.println("Optional value: " + validationAnn.optionalValue());

            // Test annotation type properties
            Class<ValidationTestAnnotation> annType = ValidationTestAnnotation.class;
            System.out.println("Annotation type is annotation: " + annType.isAnnotation());
            System.out.println("Annotation type is interface: " + annType.isInterface());
            System.out.println("Annotation type modifiers: " + Modifier.toString(annType.getModifiers()));

            // Test annotation methods
            Method[] methods = annType.getDeclaredMethods();
            for (Method method : methods) {
                System.out.println("Method: " + method.getName());
                System.out.println("  Return type: " + method.getReturnType().getName());
                System.out.println("  Has default: " + (method.getDefaultValue() != null));
                System.out.println("  Default value: " + method.getDefaultValue());
                System.out.println("  Is abstract: " + Modifier.isAbstract(method.getModifiers()));
                System.out.println("  Parameter count: " + method.getParameterCount());
            }
        }
    }

    private static void testAnnotationMethodContracts(Class<?> clazz) {
        System.out.println("\n=== Annotation Method Contracts Test ===");

        EqualityTestAnnotation ann = clazz.getAnnotation(EqualityTestAnnotation.class);
        if (ann != null) {
            // Test annotationType() contract
            Class<? extends Annotation> type1 = ann.annotationType();
            Class<? extends Annotation> type2 = ann.annotationType();

            System.out.println("annotationType() consistency: " + (type1 == type2));
            System.out.println("annotationType() returns correct type: " + (type1 == EqualityTestAnnotation.class));

            // Test toString() contract
            String str1 = ann.toString();
            String str2 = ann.toString();

            System.out.println("toString() consistency: " + str1.equals(str2));
            System.out.println("toString() format: " + str1);
            System.out.println("toString() contains @: " + str1.contains("@"));
            System.out.println("toString() contains type name: " + str1.contains("EqualityTestAnnotation"));

            // Test method return value consistency
            String value1 = ann.stringValue();
            String value2 = ann.stringValue();
            int int1 = ann.intValue();
            int int2 = ann.intValue();

            System.out.println("stringValue() consistency: " + value1.equals(value2));
            System.out.println("intValue() consistency: " + (int1 == int2));

            // Test array return value consistency
            String[] array1 = ann.arrayValue();
            String[] array2 = ann.arrayValue();

            System.out.println("arrayValue() same instance: " + (array1 == array2));
            System.out.println("arrayValue() equal content: " + java.util.Arrays.equals(array1, array2));
        }
    }

    private static void testAnnotationStringRepresentation(Class<?> clazz) {
        System.out.println("\n=== Annotation String Representation Test ===");

        Annotation[] annotations = clazz.getAnnotations();

        for (Annotation ann : annotations) {
            String toString = ann.toString();
            System.out.println("Annotation: " + ann.annotationType().getSimpleName());
            System.out.println("  toString(): " + toString);
            System.out.println("  Starts with @: " + toString.startsWith("@"));

            // Test toString format requirements
            String expectedPrefix = "@" + ann.annotationType().getName();
            System.out.println("  Expected prefix: " + expectedPrefix);
            System.out.println("  Has expected prefix: " + toString.startsWith(expectedPrefix));

            // Test parentheses for parameters
            boolean hasParameters = toString.contains("(") && toString.contains(")");
            System.out.println("  Has parameters in toString: " + hasParameters);
        }
    }

    private static void testAnnotationClassProperties() {
        System.out.println("\n=== Annotation Class Properties Test ===");

        Class<EqualityTestAnnotation> annClass = EqualityTestAnnotation.class;

        // Test basic class properties
        System.out.println("Class name: " + annClass.getName());
        System.out.println("Simple name: " + annClass.getSimpleName());
        System.out.println("Is annotation: " + annClass.isAnnotation());
        System.out.println("Is interface: " + annClass.isInterface());
        System.out.println("Is abstract: " + Modifier.isAbstract(annClass.getModifiers()));
        System.out.println("Superclass: " + annClass.getSuperclass());

        // Test annotation-specific properties
        System.out.println("Interfaces: " + java.util.Arrays.toString(annClass.getInterfaces()));
        System.out.println("Implements Annotation: " + Annotation.class.isAssignableFrom(annClass));

        // Test method properties
        Method[] methods = annClass.getDeclaredMethods();
        System.out.println("Method count: " + methods.length);

        for (Method method : methods) {
            System.out.println("Method: " + method.getName());
            System.out.println("  Abstract: " + Modifier.isAbstract(method.getModifiers()));
            System.out.println("  Public: " + Modifier.isPublic(method.getModifiers()));
            System.out.println("  Return type: " + method.getReturnType());
            System.out.println("  Parameter count: " + method.getParameterCount());
        }
    }

    private static void testEdgeCasesAndErrors(Class<?> clazz) {
        System.out.println("\n=== Edge Cases and Error Conditions Test ===");

        // Test null parameter handling
        try {
            Annotation nullAnn = clazz.getAnnotation(null);
            System.out.println("getAnnotation(null) returned: " + nullAnn);
        } catch (NullPointerException e) {
            System.out.println("getAnnotation(null) threw NullPointerException: " + e.getMessage());
        } catch (Exception e) {
            System.out.println("getAnnotation(null) threw: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }

        // Test with non-annotation class
        try {
            // Use reflection to bypass compile-time check
            java.lang.reflect.Method isAnnotationPresentMethod = Class.class.getMethod("isAnnotationPresent", Class.class);
            boolean present = (boolean) isAnnotationPresentMethod.invoke(clazz, String.class);
            System.out.println("isAnnotationPresent(String.class): " + present);
        } catch (Exception e) {
            System.out.println("isAnnotationPresent(String.class) threw: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }

        // Test annotation method error handling
        EqualityTestAnnotation ann = clazz.getAnnotation(EqualityTestAnnotation.class);
        if (ann != null) {
            try {
                // Test multiple calls for consistency
                for (int i = 0; i < 5; i++) {
                    String value = ann.stringValue();
                    if (!value.equals("class")) {
                        System.out.println("Inconsistent value at call " + i + ": " + value);
                    }
                }
                System.out.println("Multiple annotation method calls consistent");

            } catch (Exception e) {
                System.out.println("Error in annotation method calls: " + e.getMessage());
            }
        }

        // Test annotation inheritance edge cases
        Class<?> superClass = clazz.getSuperclass();
        if (superClass != null) {
            System.out.println("Superclass: " + superClass.getName());
            System.out.println("Superclass annotation count: " + superClass.getAnnotations().length);
        }

        // Test interface annotations (if any)
        Class<?>[] interfaces = clazz.getInterfaces();
        System.out.println("Interface count: " + interfaces.length);
        for (Class<?> iface : interfaces) {
            System.out.println("Interface " + iface.getName() + " annotation count: " + iface.getAnnotations().length);
        }
    }
}
