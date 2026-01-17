import java.lang.annotation.*;
import java.lang.reflect.*;
import java.util.Arrays;
import java.util.Comparator;

// Annotation with complex nested structures for parsing tests
@Retention(RetentionPolicy.RUNTIME)
@interface ComplexParsingAnnotation {
    String normalString() default "normal";
    String unicodeString() default "\u0048\u0065\u006C\u006C\u006F"; // "Hello"
    String escapedString() default "line1\nline2\ttab\"quote'apostrophe\\backslash";
    int[] largeIntArray() default {-2147483648, 2147483647, 0, 1, -1};
    double[] specialDoubles() default {Double.NaN, Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY, 0.0, -0.0};
    Class<?>[] primitiveClasses() default {boolean.class, byte.class, char.class, short.class, int.class, long.class, float.class, double.class, void.class};
}

// Annotation for testing bytecode constant pool behavior
@Retention(RetentionPolicy.RUNTIME)
@interface ConstantPoolTestAnnotation {
    String constantString() default "CONSTANT_STRING";
    int constantInt() default 12345;
    long constantLong() default 9876543210L;
    float constantFloat() default 3.14159f;
    double constantDouble() default 2.718281828;
    Class<?> constantClass() default String.class;
}

@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE, ElementType.METHOD, ElementType.FIELD})
@interface OrderTestAnnotations {
    OrderTestAnnotation[] value();
}

// Test annotation ordering and visibility
@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE, ElementType.METHOD, ElementType.FIELD, ElementType.PARAMETER})
@Repeatable(OrderTestAnnotations.class)
@interface OrderTestAnnotation {
    int order() default 0;
    String name() default "test";
}

// Multiple annotations for testing order preservation
@OrderTestAnnotation(order = 3, name = "third")
@OrderTestAnnotation(order = 1, name = "first")
@OrderTestAnnotation(order = 2, name = "second")
@ComplexParsingAnnotation(
        normalString = "customNormal",
        unicodeString = "\u4E2D\u6587\u6D4B\u8BD5", // Chinese test
        escapedString = "custom\nwith\ttabs\"and'quotes\\and\\backslashes",
        largeIntArray = {Integer.MIN_VALUE, -1000000, 0, 1000000, Integer.MAX_VALUE},
        specialDoubles = {1.7976931348623157E308, 4.9E-324, Math.PI, Math.E},
        primitiveClasses = {byte.class, short.class, int.class, long.class}
)
@ConstantPoolTestAnnotation(
        constantString = "MODIFIED_CONSTANT",
        constantInt = 54321,
        constantLong = 1122334455667788L,
        constantFloat = 2.71828f,
        constantDouble = 1.414213562373095,
        constantClass = Integer.class
)
public class Test {

    @OrderTestAnnotation(order = 10, name = "fieldFirst")
    @ComplexParsingAnnotation
    @ConstantPoolTestAnnotation
    private String testField;

    @OrderTestAnnotation(order = 20, name = "methodFirst")
    @ComplexParsingAnnotation(
            normalString = "methodString",
            largeIntArray = {1, 2, 3, 4, 5}
    )
    public void testMethod(@OrderTestAnnotation(order = 30, name = "paramFirst") String param) {
    }

    public static void main(String[] args) {
        System.out.println("=== Annotation Parsing and Bytecode Test ===");

        Class<?> clazz = Test.class;

        // Test complex parsing
        testComplexParsing(clazz);

        // Test constant pool behavior
        testConstantPoolBehavior(clazz);

        // Test annotation ordering
        testAnnotationOrdering(clazz);

        // Test bytecode-level properties
        testBytecodeProperties(clazz);

        // Test annotation access performance
        testAnnotationPerformance(clazz);
    }

    private static void testComplexParsing(Class<?> clazz) {
        System.out.println("=== Complex Parsing Test ===");

        ComplexParsingAnnotation complexAnn = clazz.getAnnotation(ComplexParsingAnnotation.class);
        if (complexAnn != null) {
            // Test normal string
            System.out.println("normalString: '" + complexAnn.normalString() + "'");

            // Test unicode string
            String unicode = complexAnn.unicodeString();
            System.out.println("unicodeString: '" + unicode + "'");
            System.out.println("unicodeString length: " + unicode.length());
            System.out.print("unicodeString codepoints: ");
            for (int i = 0; i < unicode.length(); i++) {
                System.out.print("U+" + Integer.toHexString(unicode.charAt(i)).toUpperCase() + " ");
            }
            System.out.println();

            // Test escaped string
            String escaped = complexAnn.escapedString();
            System.out.println("escapedString raw: '" + escaped + "'");
            System.out.println("escapedString processed: '" +
                    escaped.replace("\n", "\\n").replace("\t", "\\t").replace("\"", "\\\"").replace("'", "\\'").replace("\\", "\\\\") + "'");

            // Test large int array
            int[] largeInts = complexAnn.largeIntArray();
            System.out.print("largeIntArray: [");
            for (int i = 0; i < largeInts.length; i++) {
                System.out.print(largeInts[i]);
                if (i < largeInts.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            // Test special doubles
            double[] specialDoubles = complexAnn.specialDoubles();
            System.out.print("specialDoubles: [");
            for (int i = 0; i < specialDoubles.length; i++) {
                double d = specialDoubles[i];
                if (Double.isNaN(d)) {
                    System.out.print("NaN");
                } else if (Double.isInfinite(d)) {
                    System.out.print(d > 0 ? "POSITIVE_INFINITY" : "NEGATIVE_INFINITY");
                } else if (d == 0.0) {
                    System.out.print(Double.toString(d)); // Shows -0.0 vs 0.0
                } else {
                    System.out.print(d);
                }
                if (i < specialDoubles.length - 1) System.out.print(", ");
            }
            System.out.println("]");

            // Test primitive classes
            Class<?>[] primitiveClasses = complexAnn.primitiveClasses();
            System.out.print("primitiveClasses: [");
            for (int i = 0; i < primitiveClasses.length; i++) {
                Class<?> clz = primitiveClasses[i];
                System.out.print(clz.getName() + (clz.isPrimitive() ? "(primitive)" : "(not primitive)"));
                if (i < primitiveClasses.length - 1) System.out.print(", ");
            }
            System.out.println("]");
        }
    }

    private static void testConstantPoolBehavior(Class<?> clazz) {
        System.out.println("\n=== Constant Pool Test ===");

        ConstantPoolTestAnnotation constAnn = clazz.getAnnotation(ConstantPoolTestAnnotation.class);
        if (constAnn != null) {
            System.out.println("constantString: '" + constAnn.constantString() + "'");
            System.out.println("constantInt: " + constAnn.constantInt());
            System.out.println("constantLong: " + constAnn.constantLong());
            System.out.println("constantFloat: " + constAnn.constantFloat());
            System.out.println("constantDouble: " + constAnn.constantDouble());
            System.out.println("constantClass: " + constAnn.constantClass().getName());

            // Test string interning behavior
            String str1 = constAnn.constantString();
            String str2 = constAnn.constantString();
            System.out.println("String instances equal: " + (str1 == str2));
            System.out.println("String values equal: " + str1.equals(str2));

            // Test class instance behavior
            Class<?> clz1 = constAnn.constantClass();
            Class<?> clz2 = constAnn.constantClass();
            System.out.println("Class instances equal: " + (clz1 == clz2));

            // Compare with literal
            System.out.println("Class equals Integer.class: " + (clz1 == Integer.class));
        }
    }

    private static void testAnnotationOrdering(Class<?> clazz) {
        System.out.println("\n=== Annotation Ordering Test ===");

        // Note: The order of annotations in getAnnotations() is not guaranteed
        // but we test the current behavior
        Annotation[] annotations = clazz.getAnnotations();
        System.out.println("Class annotations count: " + annotations.length);

        for (int i = 0; i < annotations.length; i++) {
            Annotation ann = annotations[i];
            System.out.println("Annotation " + i + ": " + ann.annotationType().getSimpleName());

            if (ann instanceof OrderTestAnnotation) {
                OrderTestAnnotation orderAnn = (OrderTestAnnotation) ann;
                System.out.println("  order: " + orderAnn.order() + ", name: " + orderAnn.name());
            }
        }

        // Test field annotations
        try {
            Field field = clazz.getDeclaredField("testField");
            Annotation[] fieldAnnotations = field.getAnnotations();
            System.out.println("\nField annotations count: " + fieldAnnotations.length);
            for (int i = 0; i < fieldAnnotations.length; i++) {
                System.out.println("Field annotation " + i + ": " + fieldAnnotations[i].annotationType().getSimpleName());
            }
        } catch (Exception e) {
            System.out.println("Error accessing field annotations: " + e.getMessage());
        }

        // Test method annotations
        try {
            Method method = clazz.getDeclaredMethod("testMethod", String.class);
            Annotation[] methodAnnotations = method.getAnnotations();
            System.out.println("\nMethod annotations count: " + methodAnnotations.length);
            for (int i = 0; i < methodAnnotations.length; i++) {
                System.out.println("Method annotation " + i + ": " + methodAnnotations[i].annotationType().getSimpleName());
            }

            // Test parameter annotations
            Annotation[][] paramAnnotations = method.getParameterAnnotations();
            System.out.println("Parameter annotation arrays: " + paramAnnotations.length);
            if (paramAnnotations.length > 0) {
                System.out.println("First parameter annotations: " + paramAnnotations[0].length);
                for (Annotation ann : paramAnnotations[0]) {
                    System.out.println("  " + ann.annotationType().getSimpleName());
                }
            }
        } catch (Exception e) {
            System.out.println("Error accessing method annotations: " + e.getMessage());
        }
    }

    private static void testBytecodeProperties(Class<?> clazz) {
        System.out.println("\n=== Bytecode Properties Test ===");

        // Test annotation interface properties
        Class<ComplexParsingAnnotation> annClass = ComplexParsingAnnotation.class;
        System.out.println("ComplexParsingAnnotation is annotation: " + annClass.isAnnotation());
        System.out.println("ComplexParsingAnnotation is interface: " + annClass.isInterface());
        System.out.println("ComplexParsingAnnotation superclass: " + annClass.getSuperclass());
        System.out.println("ComplexParsingAnnotation interfaces: " + Arrays.toString(annClass.getInterfaces()));

        // Test annotation methods
        Method[] annMethods = annClass.getDeclaredMethods();
        // Sort methods by name for deterministic output
        Arrays.sort(annMethods, Comparator.comparing(Method::getName));
        System.out.println("ComplexParsingAnnotation methods count: " + annMethods.length);
        for (Method method : annMethods) {
            System.out.println("  Method: " + method.getName());
            System.out.println("    Return type: " + method.getReturnType().getName());
            // Normalize array default values to avoid hashcode differences
            Object defaultValue = method.getDefaultValue();
            if (defaultValue != null && defaultValue.getClass().isArray()) {
                if (defaultValue instanceof int[]) {
                    System.out.println("    Default value: " + Arrays.toString((int[]) defaultValue));
                } else if (defaultValue instanceof double[]) {
                    System.out.println("    Default value: " + Arrays.toString((double[]) defaultValue));
                } else if (defaultValue instanceof Object[]) {
                    System.out.println("    Default value: " + Arrays.toString((Object[]) defaultValue));
                } else {
                    System.out.println("    Default value: <array>");
                }
            } else {
                System.out.println("    Default value: " + defaultValue);
            }
            System.out.println("    Is abstract: " + Modifier.isAbstract(method.getModifiers()));
            System.out.println("    Is public: " + Modifier.isPublic(method.getModifiers()));
        }

        // Test annotation retention
        Retention retention = annClass.getAnnotation(Retention.class);
        if (retention != null) {
            System.out.println("Retention policy: " + retention.value());
        }

        // Test annotation target
        Target target = annClass.getAnnotation(Target.class);
        if (target != null) {
            System.out.println("Target elements: " + Arrays.toString(target.value()));
        }
    }

    private static void testAnnotationPerformance(Class<?> clazz) {
        System.out.println("\n=== Annotation Performance Test ===");

        // Test multiple accesses to same annotation
        for (int i = 0; i < 1000; i++) {
            ComplexParsingAnnotation ann = clazz.getAnnotation(ComplexParsingAnnotation.class);
            if (ann == null) break; // Shouldn't happen
        }
        System.out.println("1000 getAnnotation() calls completed");

        // Test annotation method invocation
        ComplexParsingAnnotation ann = clazz.getAnnotation(ComplexParsingAnnotation.class);
        if (ann != null) {
            for (int i = 0; i < 1000; i++) {
                String str = ann.normalString();
                if (str == null) break; // Shouldn't happen
            }
            System.out.println("1000 annotation method calls completed");

            // Test array access
            for (int i = 0; i < 1000; i++) {
                int[] array = ann.largeIntArray();
                if (array == null) break; // Shouldn't happen
            }
            System.out.println("1000 annotation array accesses completed");
        }

        // Test isAnnotationPresent vs getAnnotation
        for (int i = 0; i < 1000; i++) {
            boolean present = clazz.isAnnotationPresent(ComplexParsingAnnotation.class);
            if (!present) break; // Shouldn't happen
        }
        System.out.println("1000 isAnnotationPresent() calls completed");
    }
}
